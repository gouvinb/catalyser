//! Code generation for `#[overloads]`.
//!
//! For a function `add(a: i32, #[default(10)] b: i32) -> i32`, the macro emits:
//! 1. `struct AddArgs { pub b: i32 }`
//! 2. `impl Default for AddArgs { fn default() -> Self { let b = 10; Self { b } } }`
//! 3. `pub fn add(a: i32, b: i32) -> i32 { a + b }`
//! 4. `pub fn add_overload(a: i32, args: AddArgs) -> i32 { add(a, args.b) }`
//! 5. `pub fn add_default(a: i32) -> i32 { add_overload(a, AddArgs::default()) }`
//!
//! For an `impl` block:
//! ```ignore
//! #[overloads]
//! impl Sub {
//!     pub fn new(a: i32, #[default(10)] b: i32) -> Self { Sub { a, b } }
//! }
//! ```
//! The macro emits `SubNewArgs` struct outside `impl Sub`, and `new`, `new_overload`, `new_default` inside `impl Sub`.
//!
//! For a `trait`:
//! ```ignore
//! #[overloads]
//! pub trait Mod {
//!     fn modulo(&self, a: i32, #[default(10)] b: i32) -> i32;
//! }
//! ```
//! The macro emits `ModModuloArgs` struct outside `trait Mod`, and `modulo`, `modulo_overload`, `modulo_default` inside `trait Mod`.

use crate::overloads::parse::{
    OverloadsFn,
    OverloadsImpl,
    OverloadsInput,
    OverloadsMethod,
    OverloadsTrait,
};
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use std::collections::{
    HashMap,
    HashSet,
};
use syn::{
    GenericParam,
    Generics,
    Ident,
    Type,
};

pub fn expand(input: &OverloadsInput) -> TokenStream {
    match input {
        OverloadsInput::Fn(f) => expand_fn(f),
        OverloadsInput::Impl(i) => expand_impl(i),
        OverloadsInput::Trait(t) => expand_trait(t),
    }
}

pub fn expand_fn(def: &OverloadsFn) -> TokenStream {
    let expanded = expand_method(None, None, &def.method, &def.method.vis, false);
    let struct_defs = expanded.struct_defs;
    let methods = expanded.impl_methods;
    quote! {
        #struct_defs
        #methods
    }
}

pub fn expand_impl(def: &OverloadsImpl) -> TokenStream {
    let mut all_struct_defs = Vec::new();
    let mut transformed_map: HashMap<usize, TokenStream> = HashMap::new();

    for (idx, method) in &def.transformed_methods {
        let expanded = expand_method(
            Some(&def.self_ident),
            Some(&def.item_impl.generics),
            method,
            &method.vis,
            false,
        );
        all_struct_defs.push(expanded.struct_defs);
        transformed_map.insert(*idx, expanded.impl_methods);
    }

    let mut impl_items = Vec::new();
    for (idx, item) in def.item_impl.items.iter().enumerate() {
        if let Some(transformed) = transformed_map.get(&idx) {
            impl_items.push(transformed.clone());
        } else {
            impl_items.push(item.to_token_stream());
        }
    }

    let (impl_generics, _, where_clause) = def.item_impl.generics.split_for_impl();
    let self_ty = &def.item_impl.self_ty;
    let unsafety = &def.item_impl.unsafety;
    let attrs = &def.item_impl.attrs;

    let impl_block = quote! {
        #(#attrs)*
        #unsafety impl #impl_generics #self_ty #where_clause {
            #(#impl_items)*
        }
    };

    quote! {
        #(#all_struct_defs)*
        #impl_block
    }
}

pub fn expand_trait(def: &OverloadsTrait) -> TokenStream {
    let mut all_struct_defs = Vec::new();
    let mut transformed_map: HashMap<usize, TokenStream> = HashMap::new();

    for (idx, method) in &def.transformed_methods {
        let expanded = expand_method(
            Some(&def.trait_ident),
            Some(&def.item_trait.generics),
            method,
            &def.item_trait.vis,
            true,
        );
        all_struct_defs.push(expanded.struct_defs);
        transformed_map.insert(*idx, expanded.impl_methods);
    }

    let mut trait_items = Vec::new();
    for (idx, item) in def.item_trait.items.iter().enumerate() {
        if let Some(transformed) = transformed_map.get(&idx) {
            trait_items.push(transformed.clone());
        } else {
            trait_items.push(item.to_token_stream());
        }
    }

    let (impl_generics, _, where_clause) = def.item_trait.generics.split_for_impl();
    let trait_ident = &def.item_trait.ident;
    let unsafety = &def.item_trait.unsafety;
    let attrs = &def.item_trait.attrs;
    let vis = &def.item_trait.vis;
    let colon_token = &def.item_trait.colon_token;
    let supertraits = &def.item_trait.supertraits;

    let trait_block = quote! {
        #(#attrs)*
        #vis #unsafety trait #trait_ident #impl_generics #colon_token #supertraits #where_clause {
            #(#trait_items)*
        }
    };

    quote! {
        #(#all_struct_defs)*
        #trait_block
    }
}

struct ExpandedMethod {
    struct_defs: TokenStream,
    impl_methods: TokenStream,
}

fn expand_method(
    self_ident: Option<&Ident>,
    impl_generics: Option<&Generics>,
    method: &OverloadsMethod,
    struct_vis: &syn::Visibility,
    is_trait: bool,
) -> ExpandedMethod {
    let name = &method.sig.ident;
    let vis = &method.vis;
    let attrs = &method.attrs;
    let asyncness = &method.sig.asyncness;
    let output = &method.sig.output;
    let block = &method.block;
    let receiver = &method.receiver;

    let (method_impl_generics, _, method_where_clause) = method.sig.generics.split_for_impl();

    let method_vis = if is_trait { quote!() } else { quote!(#vis) };

    let pascal_name = snake_to_pascal(&name.to_string());
    let args_struct_ident = if let Some(self_id) = self_ident {
        let pascal_self = snake_to_pascal(&self_id.to_string());
        format_ident!("{}{}Args", pascal_self, pascal_name, span = name.span())
    } else {
        format_ident!("{}Args", pascal_name, span = name.span())
    };

    let overload_fn_ident = format_ident!("{}_overload", name, span = name.span());
    let default_fn_ident = format_ident!("{}_default", name, span = name.span());

    let receiver_param = receiver.as_ref().map(|r| quote!(#r,));

    let all_param_bindings: Vec<_> = method
        .params
        .iter()
        .map(|p| {
            let (ident, ty, param_attrs) = (&p.ident, &p.ty, &p.attrs);
            quote!(#(#param_attrs)* #ident: #ty)
        })
        .collect();

    let original_fn = if let Some(block) = block {
        quote! {
            #(#attrs)*
            #method_vis #asyncness fn #name #method_impl_generics (
                #receiver_param
                #(#all_param_bindings),*
            ) #output #method_where_clause #block
        }
    } else {
        quote! {
            #(#attrs)*
            #method_vis #asyncness fn #name #method_impl_generics (
                #receiver_param
                #(#all_param_bindings),*
            ) #output #method_where_clause;
        }
    };

    let (required_params, defaulted_params): (Vec<_>, Vec<_>) = method.params.iter().partition(|p| p.default.is_none());

    let req_param_bindings: Vec<_> = required_params
        .iter()
        .map(|p| {
            let (ident, ty, param_attrs) = (&p.ident, &p.ty, &p.attrs);
            quote!(#(#param_attrs)* #ident: #ty)
        })
        .collect();
    let req_param_idents: Vec<_> = required_params.iter().map(|p| &p.ident).collect();

    let call_orig_target = if receiver.is_some() {
        quote!(self.#name)
    } else if self_ident.is_some() {
        quote!(Self::#name)
    } else {
        quote!(#name)
    };

    let call_overload_target = if receiver.is_some() {
        quote!(self.#overload_fn_ident)
    } else if self_ident.is_some() {
        quote!(Self::#overload_fn_ident)
    } else {
        quote!(#overload_fn_ident)
    };

    if defaulted_params.is_empty() {
        let call_main = if asyncness.is_some() {
            quote!(#call_orig_target(#(#req_param_idents),*).await)
        } else {
            quote!(#call_orig_target(#(#req_param_idents),*))
        };

        let struct_defs = quote! {
            #struct_vis struct #args_struct_ident;

            impl ::core::default::Default for #args_struct_ident {
                fn default() -> Self {
                    Self
                }
            }
        };

        let overload_fn = quote! {
            #method_vis #asyncness fn #overload_fn_ident #method_impl_generics (
                #receiver_param
                #(#req_param_bindings,)*
                _args: #args_struct_ident
            ) #output #method_where_clause {
                #call_main
            }
        };

        let default_fn = quote! {
            #method_vis #asyncness fn #default_fn_ident #method_impl_generics (
                #receiver_param
                #(#req_param_bindings),*
            ) #output #method_where_clause {
                #call_main
            }
        };

        let impl_methods = quote! {
            #original_fn
            #overload_fn
            #default_fn
        };

        return ExpandedMethod {
            struct_defs,
            impl_methods,
        };
    }

    let empty_generics = Generics::default();
    let base_generics = impl_generics.unwrap_or(&empty_generics);
    let combined_generics = combine_generics(base_generics, &method.sig.generics);

    let defaulted_types: Vec<&Type> = defaulted_params.iter().map(|p| &p.ty).collect();
    let args_generics = filter_generics(&combined_generics, &defaulted_types);
    let (args_impl_generics, args_type_generics, args_where_clause) = args_generics.split_for_impl();
    let args_turbofish = args_type_generics.as_turbofish();

    let struct_fields: Vec<_> = defaulted_params
        .iter()
        .map(|p| {
            let (ident, ty, param_attrs) = (&p.ident, &p.ty, &p.attrs);
            quote! {
                #(#param_attrs)*
                pub #ident: #ty,
            }
        })
        .collect();

    let struct_def = quote! {
        #struct_vis struct #args_struct_ident #args_generics #args_where_clause {
            #(#struct_fields)*
        }
    };

    let default_bindings: Vec<_> = defaulted_params
        .iter()
        .map(|p| {
            let ident = &p.ident;
            let default_expr = p.default.as_ref().unwrap();
            quote! {
                let #ident = #default_expr;
            }
        })
        .collect();
    let default_fields: Vec<_> = defaulted_params.iter().map(|p| &p.ident).collect();

    let default_impl = quote! {
        impl #args_impl_generics ::core::default::Default for #args_struct_ident #args_type_generics #args_where_clause {
            fn default() -> Self {
                #(#default_bindings)*
                Self {
                    #(#default_fields),*
                }
            }
        }
    };

    let struct_defs = quote! {
        #struct_def
        #default_impl
    };

    let def_param_args: Vec<_> = defaulted_params
        .iter()
        .map(|p| {
            let ident = &p.ident;
            quote!(args.#ident)
        })
        .collect();

    let call_orig_from_overload = if asyncness.is_some() {
        quote!(#call_orig_target(#(#req_param_idents,)* #(#def_param_args),*).await)
    } else {
        quote!(#call_orig_target(#(#req_param_idents,)* #(#def_param_args),*))
    };

    let overload_fn = quote! {
        #method_vis #asyncness fn #overload_fn_ident #method_impl_generics (
            #receiver_param
            #(#req_param_bindings,)*
            args: #args_struct_ident #args_type_generics
        ) #output #method_where_clause {
            #call_orig_from_overload
        }
    };

    let call_overload_default = if asyncness.is_some() {
        quote!(#call_overload_target(#(#req_param_idents,)* #args_struct_ident #args_turbofish::default()).await)
    } else {
        quote!(#call_overload_target(#(#req_param_idents,)* #args_struct_ident #args_turbofish::default()))
    };

    let default_fn = quote! {
        #method_vis #asyncness fn #default_fn_ident #method_impl_generics (
            #receiver_param
            #(#req_param_bindings),*
        ) #output #method_where_clause {
            #call_overload_default
        }
    };

    let impl_methods = quote! {
        #original_fn
        #overload_fn
        #default_fn
    };

    ExpandedMethod {
        struct_defs,
        impl_methods,
    }
}

fn combine_generics(impl_generics: &Generics, method_generics: &Generics) -> Generics {
    let mut combined = impl_generics.clone();
    for param in &method_generics.params {
        combined.params.push(param.clone());
    }
    if let Some(method_where) = &method_generics.where_clause {
        let where_clause = combined
            .where_clause
            .get_or_insert_with(|| syn::WhereClause {
                where_token: method_where.where_token,
                predicates: syn::punctuated::Punctuated::new(),
            });
        for pred in &method_where.predicates {
            where_clause.predicates.push(pred.clone());
        }
    }
    combined
}

fn snake_to_pascal(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    if result.is_empty() {
        result.push_str("Fn");
    }
    result
}

fn collect_tokens(ts: TokenStream, idents: &mut HashSet<String>) {
    for tt in ts {
        match tt {
            TokenTree::Ident(id) => {
                idents.insert(id.to_string());
            },
            TokenTree::Group(g) => {
                collect_tokens(g.stream(), idents);
            },
            TokenTree::Punct(_) | TokenTree::Literal(_) => {},
        }
    }
}

fn filter_generics(generics: &Generics, types: &[&Type]) -> Generics {
    let mut used_idents = HashSet::new();
    for ty in types {
        collect_tokens(ty.to_token_stream(), &mut used_idents);
    }

    let mut filtered = generics.clone();
    filtered.params = filtered
        .params
        .into_iter()
        .filter(|param| match param {
            GenericParam::Type(t) => used_idents.contains(&t.ident.to_string()),
            GenericParam::Lifetime(l) => used_idents.contains(&l.lifetime.ident.to_string()),
            GenericParam::Const(c) => used_idents.contains(&c.ident.to_string()),
        })
        .collect();

    if let Some(where_clause) = &mut filtered.where_clause {
        let active_generic_idents: HashSet<String> = filtered
            .params
            .iter()
            .map(|param| match param {
                GenericParam::Type(t) => t.ident.to_string(),
                GenericParam::Lifetime(l) => l.lifetime.ident.to_string(),
                GenericParam::Const(c) => c.ident.to_string(),
            })
            .collect();

        where_clause.predicates = where_clause
            .predicates
            .clone()
            .into_iter()
            .filter(|pred| {
                let mut pred_idents = HashSet::new();
                collect_tokens(pred.to_token_stream(), &mut pred_idents);
                pred_idents
                    .iter()
                    .any(|id| active_generic_idents.contains(id))
            })
            .collect();

        if where_clause.predicates.is_empty() {
            filtered.where_clause = None;
        }
    }

    filtered
}
