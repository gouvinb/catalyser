//! Code generation for `#[overloads]`.
//!
//! For a function `connect` the macro emits three things, one per Rust namespace, all
//! sharing the same name so that a single `use path::connect;` imports them together:
//!
//! * value namespace: the original `fn connect(..)`, untouched (minus `#[default]`);
//! * type namespace: a hidden `mod connect` holding `__apply`, a twin function whose
//!   defaulted parameters are `Option<T>`. Defaults are evaluated *there*, i.e. in the
//!   scope of the definition (so paths resolve where the function is written, and a
//!   default may reference the parameters declared before it);
//! * macro namespace: a `macro_rules!` re-exported as `connect`, which maps the call
//!   site arguments (positional first, then named, in any order) onto
//!   `connect::__apply(..)`.

use crate::parse::OverloadsFn;
use proc_macro2::{
    Literal,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    Ident,
    Visibility,
};

pub fn expand(def: &OverloadsFn) -> TokenStream {
    let func = &def.func;
    let apply_mod = apply_module(def);
    let call_macro = call_macro(def);
    quote! {
        #func
        #apply_mod
        #call_macro
    }
}

/// `mod <name> { pub fn __apply(..) }`
fn apply_module(def: &OverloadsFn) -> TokenStream {
    let sig = &def.func.sig;
    let name = &sig.ident;
    let vis = &def.func.vis;
    let output = &sig.output;
    let (impl_generics, _, where_clause) = sig.generics.split_for_impl();

    // Defaulted parameters become `Option<T>`; the others are unchanged.
    let params = def.params.iter().map(|p| {
        let (ident, ty) = (&p.ident, &p.ty);
        if p.default.is_some() {
            quote!(#ident: ::core::option::Option<#ty>)
        } else {
            quote!(#ident: #ty)
        }
    });

    // Resolve defaults in declaration order, so a default can use earlier parameters.
    let resolve = def.params.iter().filter_map(|p| {
        p.default.as_ref().map(|default| {
            let ident = &p.ident;
            quote! {
                let #ident = match #ident {
                    ::core::option::Option::Some(__value) => __value,
                    ::core::option::Option::None => #default,
                };
            }
        })
    });

    let args = def.params.iter().map(|p| &p.ident);
    let is_async = sig.asyncness.is_some();
    let call = if is_async {
        quote!(super::#name(#(#args),*).await)
    } else {
        quote!(super::#name(#(#args),*))
    };
    let asyncness = &sig.asyncness;

    quote! {
        #[doc(hidden)]
        #vis mod #name {
            // Bring the definition scope in: parameter types and defaults are written
            // relative to the module that contains the function.
            #[allow(unused_imports)]
            use super::*;

            #[allow(clippy::too_many_arguments)]
            pub #asyncness fn __apply #impl_generics ( #(#params),* ) #output #where_clause {
                #(#resolve)*
                #call
            }
        }
    }
}

/// The call-site macro, written as a token muncher.
///
/// Its state is a brace group holding one slot per parameter:
///
/// ```text
/// { host: [] port: [::core::option::Option::None] timeout: [::core::option::Option::None] }
/// ```
///
/// A slot is `[]` while a required argument is missing, `[$v]` once it is supplied;
/// a defaulted slot starts as `[None]` and becomes `[Some($v)]`. Because the state is
/// keyed by parameter, named arguments can arrive in any order and the final call is
/// always emitted in declaration order.
///
/// Internal rules, tried in this order:
///
/// * `@pos i`   : consuming positional arguments, `i` already consumed;
/// * `@named`   : consuming `name = value` arguments (positional ones are now an error);
/// * `@finish`  : every argument consumed, emit the call or report a missing argument;
/// * the last rule is the public entry point.
fn call_macro(def: &OverloadsFn) -> TokenStream {
    let name = &def.func.sig.ident;
    let macro_name = format_ident!("__{}_call", name);
    let n = def.params.len();
    let n_lit = Literal::usize_unsuffixed(n);

    let names: Vec<&Ident> = def.params.iter().map(|p| &p.ident).collect();
    let vars: Vec<Ident> = (0..n).map(|j| format_ident!("s{}", j)).collect();
    let is_defaulted = |j: usize| def.params[j].default.is_some();
    let accepted = names
        .iter()
        .map(|i| format!("`{i}`"))
        .collect::<Vec<_>>()
        .join(", ");

    // Slot content while the argument has not been supplied yet.
    let unset = |j: usize| {
        if is_defaulted(j) {
            quote!(::core::option::Option::None)
        } else {
            quote!()
        }
    };
    // Slot content once the argument has been captured as `$v`.
    let filled = |j: usize| {
        if is_defaulted(j) {
            quote!(::core::option::Option::Some($v))
        } else {
            quote!($v)
        }
    };

    // Pattern matching any state, or only states where slot `only_unset` is still unset.
    let state_pat = |only_unset: Option<usize>| {
        let slots = (0..n).map(|j| {
            let nm = names[j];
            if Some(j) == only_unset {
                let c = unset(j);
                quote!(#nm: [ #c ])
            } else {
                let v = &vars[j];
                quote!(#nm: [ $($#v:tt)* ])
            }
        });
        quote!({ #(#slots)* })
    };
    // State rebuilt from the captured slots, with slot `set_slot` replaced by `$v`.
    let state_out = |set_slot: Option<usize>| {
        let slots = (0..n).map(|j| {
            let nm = names[j];
            if Some(j) == set_slot {
                let c = filled(j);
                quote!(#nm: [ #c ])
            } else {
                let v = &vars[j];
                quote!(#nm: [ $($#v)* ])
            }
        });
        quote!({ #(#slots)* })
    };
    let init_state = {
        let slots = (0..n).map(|j| {
            let nm = names[j];
            let c = unset(j);
            quote!(#nm: [ #c ])
        });
        quote!({ #(#slots)* })
    };

    let mut arms: Vec<TokenStream> = Vec::new();

    // --- positional phase --------------------------------------------------------------
    // `name = ...` switches to the named phase.
    arms.push(quote! {
        (@pos $i:tt $s:tt $arg:ident = $($rest:tt)*) => { #name!(@named $s $arg = $($rest)*) }
    });
    for i in 0..n {
        let (cur, next) = (
            Literal::usize_unsuffixed(i),
            Literal::usize_unsuffixed(i + 1),
        );
        let (pat, out) = (state_pat(None), state_out(Some(i)));
        arms.push(quote! {
            (@pos #cur #pat $v:expr $(, $($rest:tt)*)?) => {
                #name!(@pos #next #out $($($rest)*)?)
            }
        });
    }
    let too_many = format!("too many arguments: `{name}` takes {n} parameter(s)");
    arms.push(quote! {
        (@pos #n_lit $s:tt $($rest:tt)+) => { ::core::compile_error!(#too_many) }
    });
    arms.push(quote! {
        (@pos $i:tt $s:tt) => { #name!(@finish $s) }
    });

    // --- named phase -------------------------------------------------------------------
    for (k, nk) in names.iter().enumerate().take(n) {
        let (pat, out) = (state_pat(Some(k)), state_out(Some(k)));
        arms.push(quote! {
            (@named #pat #nk = $v:expr $(, $($rest:tt)*)?) => {
                #name!(@named #out $($($rest)*)?)
            }
        });
    }
    for (_, nk) in names.iter().enumerate().take(n) {
        let twice = format!("argument `{nk}` is specified more than once");
        arms.push(quote! {
            (@named $s:tt #nk = $($rest:tt)*) => { ::core::compile_error!(#twice) }
        });
    }
    let unknown_prefix = "unknown argument `";
    let unknown_suffix = format!("`; `{name}` accepts: {accepted}");
    arms.push(quote! {
        (@named $s:tt $other:ident = $($rest:tt)*) => {
            ::core::compile_error!(::core::concat!(
                #unknown_prefix, ::core::stringify!($other), #unknown_suffix
            ))
        }
    });
    arms.push(quote! {
        (@named $s:tt $($rest:tt)+) => {
            ::core::compile_error!("positional argument cannot follow a named argument")
        }
    });
    arms.push(quote! {
        (@named $s:tt) => { #name!(@finish $s) }
    });

    // --- finish --------------------------------------------------------------------------
    for k in (0..n).filter(|&k| !is_defaulted(k)) {
        let pat = state_pat(Some(k));
        let missing = format!("missing required argument `{}`", names[k]);
        arms.push(quote! {
            (@finish #pat) => { ::core::compile_error!(#missing) }
        });
    }
    {
        let pat = state_pat(None);
        let args = vars.iter().map(|v| quote!($($#v)*));
        arms.push(quote! {
            (@finish #pat) => { #name::__apply( #(#args),* ) }
        });
    }

    // --- public entry point ----------------------------------------------------------------
    arms.push(quote! {
        ($($args:tt)*) => { #name!(@pos 0 #init_state $($args)*) }
    });

    // A `pub` function is re-exported at `pub(crate)` only: `#[macro_export]` would need
    // the definition's module path, which a proc-macro cannot know.
    let reexport_vis = match &def.func.vis {
        Visibility::Public(_) => quote!(pub(crate)),
        other => quote!(#other),
    };

    quote! {
        #[allow(unused_macros)]
        macro_rules! #macro_name { #(#arms);* }
        #[allow(unused_imports)]
        #reexport_vis use #macro_name as #name;
    }
}
