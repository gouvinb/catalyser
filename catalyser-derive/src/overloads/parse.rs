//! Model of functions, impl blocks, and traits whose parameters may carry `#[default(expr)]`.
//!
//! `fn f(a: u8, #[default(5)] b: u8)` is syntactically valid Rust, so we can parse a
//! plain `syn::ItemFn`, `syn::ItemImpl`, or `syn::ItemTrait` and then extract the `#[default(..)]`
//! attributes from the parameters.
//!
//! Parameters with a default must form a suffix of the parameter list: a parameter
//! without a default cannot follow one that has a default.

use proc_macro2::TokenTree;
use quote::ToTokens;
use std::collections::HashSet;
use syn::{
    Error,
    Expr,
    FnArg,
    Ident,
    ItemFn,
    ItemImpl,
    ItemTrait,
    Pat,
    Receiver,
    Result,
    Signature,
    Type,
};

/// A named parameter with its optional default value.
#[derive(Clone)]
pub struct Param {
    pub ident: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
    pub attrs: Vec<syn::Attribute>,
}

/// A method in an `impl` block, a `trait`, or a free function.
#[derive(Clone)]
pub struct OverloadsMethod {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub sig: Signature,
    pub block: Option<syn::Block>,
    pub receiver: Option<Receiver>,
    pub params: Vec<Param>,
}

/// A free function with its parsed parameters.
pub struct OverloadsFn {
    pub method: OverloadsMethod,
}

/// An `impl` block with transformed methods.
pub struct OverloadsImpl {
    pub item_impl: ItemImpl,
    pub self_ident: Ident,
    pub transformed_methods: Vec<(usize, OverloadsMethod)>,
}

/// A `trait` definition with transformed methods.
pub struct OverloadsTrait {
    pub item_trait: ItemTrait,
    pub trait_ident: Ident,
    pub transformed_methods: Vec<(usize, OverloadsMethod)>,
}

#[allow(clippy::large_enum_variant)]
pub enum OverloadsInput {
    Fn(OverloadsFn),
    Impl(OverloadsImpl),
    Trait(OverloadsTrait),
}

fn check_qualifiers(sig: &Signature) -> Result<()> {
    for tt in sig.to_token_stream() {
        if let TokenTree::Ident(id) = &tt {
            match id.to_string().as_str() {
                "fn" => break,
                "const" | "unsafe" | "extern" => {
                    return Err(Error::new(
                        id.span(),
                        format!("#[overloads] does not support `{id}` functions"),
                    ));
                },
                _ => {},
            }
        }
    }
    if let Some(t) = &sig.variadic {
        return Err(Error::new_spanned(
            t,
            "#[overloads] does not support variadic functions",
        ));
    }
    Ok(())
}

fn parse_params(inputs: &mut syn::punctuated::Punctuated<FnArg, syn::token::Comma>, allow_receiver: bool) -> Result<(Option<Receiver>, Vec<Param>)> {
    let mut params = Vec::new();
    let mut seen = HashSet::new();
    let mut first_default: Option<Ident> = None;
    let mut receiver = None;

    for arg in inputs.iter_mut() {
        match arg {
            FnArg::Receiver(rec) => {
                if !allow_receiver {
                    return Err(Error::new_spanned(
                        rec,
                        "#[overloads] only supports free functions (no `self` receiver)",
                    ));
                }
                if receiver.is_some() {
                    return Err(Error::new_spanned(rec, "duplicate `self` receiver"));
                }
                receiver = Some(rec.clone());
            },
            FnArg::Typed(pat_type) => {
                // Extract #[default(expr)]; every other attribute is kept as-is.
                let mut default = None;
                let mut kept = Vec::new();
                for attr in std::mem::take(&mut pat_type.attrs) {
                    if attr.path().is_ident("default") {
                        if default.is_some() {
                            return Err(Error::new_spanned(&attr, "duplicate #[default] attribute"));
                        }
                        default = Some(attr.parse_args::<Expr>()?);
                    } else {
                        kept.push(attr);
                    }
                }
                pat_type.attrs = kept.clone();

                let ident = match &*pat_type.pat {
                    Pat::Ident(pi) if pi.by_ref.is_none() && pi.subpat.is_none() => pi.ident.clone(),
                    other => {
                        return Err(Error::new_spanned(
                            other,
                            "#[overloads]: the parameter must be a plain identifier",
                        ));
                    },
                };

                if default.is_some() && matches!(&*pat_type.ty, Type::ImplTrait(_)) {
                    return Err(Error::new_spanned(
                        &pat_type.ty,
                        "#[overloads]: a parameter with a default cannot be `impl Trait` \
                         (its type cannot be inferred when the argument is omitted); \
                         use a named generic parameter instead",
                    ));
                }

                if !seen.insert(ident.to_string()) {
                    return Err(Error::new_spanned(&ident, "duplicate parameter name"));
                }

                // Defaulted parameters must be trailing.
                if default.is_some() {
                    first_default.get_or_insert_with(|| ident.clone());
                } else if let Some(prev) = &first_default {
                    return Err(Error::new_spanned(
                        &ident,
                        format!(
                            "parameter `{ident}` has no default but follows `{prev}`, which does; \
                             move all `#[default]` parameters to the end of the signature"
                        ),
                    ));
                }

                params.push(Param {
                    ident,
                    ty: (*pat_type.ty).clone(),
                    default,
                    attrs: kept,
                });
            },
        }
    }

    Ok((receiver, params))
}

fn extract_self_ident(self_ty: &Type) -> Ident {
    match self_ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident.clone()
            } else {
                Ident::new("SelfType", proc_macro2::Span::call_site())
            }
        },
        _ => Ident::new("SelfType", proc_macro2::Span::call_site()),
    }
}

impl OverloadsFn {
    pub fn from_item(mut func: ItemFn) -> Result<Self> {
        check_qualifiers(&func.sig)?;
        let (receiver, params) = parse_params(&mut func.sig.inputs, false)?;
        let method = OverloadsMethod {
            attrs: func.attrs.clone(),
            vis: func.vis.clone(),
            sig: func.sig.clone(),
            block: Some((*func.block).clone()),
            receiver,
            params,
        };
        Ok(Self { method })
    }
}

impl OverloadsImpl {
    pub fn from_item(mut item_impl: ItemImpl) -> Result<Self> {
        if item_impl.trait_.is_some() {
            return Err(Error::new_spanned(
                &item_impl.self_ty,
                "#[overloads] is only supported on inherent impl blocks, not trait implementations",
            ));
        }

        let self_ident = extract_self_ident(&item_impl.self_ty);
        let mut transformed_methods = Vec::new();

        for (idx, item) in item_impl.items.iter_mut().enumerate() {
            if let syn::ImplItem::Fn(method) = item {
                let has_overloads_attr = method.attrs.iter().any(|a| a.path().is_ident("overloads"));
                let has_default_param = method.sig.inputs.iter().any(|arg| {
                    if let FnArg::Typed(pt) = arg {
                        pt.attrs.iter().any(|a| a.path().is_ident("default"))
                    } else {
                        false
                    }
                });

                if has_overloads_attr || has_default_param {
                    method.attrs.retain(|a| !a.path().is_ident("overloads"));
                    check_qualifiers(&method.sig)?;
                    let (receiver, params) = parse_params(&mut method.sig.inputs, true)?;
                    transformed_methods.push((
                        idx,
                        OverloadsMethod {
                            attrs: method.attrs.clone(),
                            vis: method.vis.clone(),
                            sig: method.sig.clone(),
                            block: Some(method.block.clone()),
                            receiver,
                            params,
                        },
                    ));
                }
            }
        }

        Ok(Self {
            item_impl,
            self_ident,
            transformed_methods,
        })
    }
}

impl OverloadsTrait {
    pub fn from_item(mut item_trait: ItemTrait) -> Result<Self> {
        let trait_ident = item_trait.ident.clone();
        let mut transformed_methods = Vec::new();

        for (idx, item) in item_trait.items.iter_mut().enumerate() {
            if let syn::TraitItem::Fn(method) = item {
                let has_overloads_attr = method.attrs.iter().any(|a| a.path().is_ident("overloads"));
                let has_default_param = method.sig.inputs.iter().any(|arg| {
                    if let FnArg::Typed(pt) = arg {
                        pt.attrs.iter().any(|a| a.path().is_ident("default"))
                    } else {
                        false
                    }
                });

                if has_overloads_attr || has_default_param {
                    method.attrs.retain(|a| !a.path().is_ident("overloads"));
                    check_qualifiers(&method.sig)?;
                    let (receiver, params) = parse_params(&mut method.sig.inputs, true)?;
                    transformed_methods.push((
                        idx,
                        OverloadsMethod {
                            attrs: method.attrs.clone(),
                            vis: syn::Visibility::Inherited,
                            sig: method.sig.clone(),
                            block: method.default.clone(),
                            receiver,
                            params,
                        },
                    ));
                }
            }
        }

        Ok(Self {
            item_trait,
            trait_ident,
            transformed_methods,
        })
    }
}

impl OverloadsInput {
    pub fn parse(input: proc_macro2::TokenStream) -> Result<Self> {
        if let Ok(func) = syn::parse2::<ItemFn>(input.clone()) {
            return OverloadsFn::from_item(func).map(OverloadsInput::Fn);
        }
        if let Ok(item_impl) = syn::parse2::<ItemImpl>(input.clone()) {
            return OverloadsImpl::from_item(item_impl).map(OverloadsInput::Impl);
        }
        if let Ok(item_trait) = syn::parse2::<ItemTrait>(input.clone()) {
            return OverloadsTrait::from_item(item_trait).map(OverloadsInput::Trait);
        }

        if syn::parse2::<syn::ImplItemFn>(input.clone()).is_ok() {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "#[overloads] cannot be applied directly to a method inside an `impl` block because Rust does not permit struct definitions inside `impl` blocks; place #[overloads] on the `impl` block itself: `#[overloads] impl StructName { ... }`",
            ));
        }

        if syn::parse2::<syn::TraitItemFn>(input).is_ok() {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "#[overloads] cannot be applied directly to a method inside a `trait` because Rust does not permit struct definitions inside traits; place #[overloads] on the `trait` itself: `#[overloads] trait TraitName { ... }`",
            ));
        }

        Err(Error::new(
            proc_macro2::Span::call_site(),
            "#[overloads] can only be applied to a function (`fn`), an `impl` block, or a `trait`",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(f: &OverloadsFn) -> String {
        let sig = &f.method.sig;
        let block = &f.method.block;
        quote::quote!(#sig #block).to_string()
    }

    fn parse(src: &str) -> Result<OverloadsFn> {
        OverloadsFn::from_item(syn::parse_str::<ItemFn>(src)?)
    }

    #[test]
    fn extracts_overloads() {
        let f = parse("pub fn connect(host: &str, #[default(5432)] port: u16) {}").unwrap();
        assert_eq!(f.method.params.len(), 2);
        assert!(f.method.params[0].default.is_none());
        assert!(f.method.params[1].default.is_some());
        assert_eq!(f.method.params[1].ident, "port");
    }

    #[test]
    fn default_can_be_a_complex_expression() {
        let f = parse("fn f(#[default(Duration::from_secs(5 * 60))] t: Duration) {}").unwrap();
        assert!(f.method.params[0].default.is_some());
    }

    #[test]
    fn plain_fn_has_no_default_attribute() {
        let f = parse("fn f(#[default(1)] a: u8) {}").unwrap();
        assert!(!plain(&f).contains("default"));
    }

    #[test]
    fn keeps_other_param_attributes() {
        let f = parse("fn f(#[allow(unused)] #[default(1)] a: u8) {}").unwrap();
        assert!(plain(&f).contains("allow"));
    }

    #[test]
    fn keeps_generics_where_clause_and_return_type() {
        let f = parse("fn f<T>(#[default(Default::default())] x: T) -> Vec<T> where T: Default { vec![x] }").unwrap();
        let out = plain(&f);
        assert!(out.contains("where T : Default"));
        assert!(out.contains("Vec < T >"));
    }

    #[test]
    fn rejects_self() {
        assert!(parse("fn f(&self, #[default(1)] a: u8) {}").is_err());
    }

    #[test]
    fn rejects_duplicate_names() {
        assert!(parse("fn f(a: u8, a: u8) {}").is_err());
    }

    #[test]
    fn rejects_destructuring() {
        assert!(parse("fn f((a, b): (u8, u8)) {}").is_err());
    }

    #[test]
    fn rejects_duplicate_default_attribute() {
        assert!(parse("fn f(#[default(1)] #[default(2)] a: u8) {}").is_err());
    }

    #[test]
    fn rejects_unsupported_qualifiers() {
        assert!(parse("const fn f(a: u8) {}").is_err());
        assert!(parse("unsafe fn f(a: u8) {}").is_err());
        assert!(parse("extern \"C\" fn f(a: u8) {}").is_err());
    }

    #[test]
    fn rejects_impl_trait_with_default() {
        assert!(parse("fn f(#[default(1)] a: impl Into<u8>) {}").is_err());
        assert!(parse("fn f(a: impl Into<u8>, #[default(1)] b: u8) {}").is_ok());
    }

    #[test]
    fn accepts_overloads_after_required() {
        assert!(parse("fn f(a: u8, b: u8, #[default(1)] c: u8, #[default(2)] d: u8) {}").is_ok());
        assert!(parse("fn f(#[default(1)] a: u8, #[default(2)] b: u8) {}").is_ok());
    }

    #[test]
    fn rejects_required_after_default() {
        let err = parse("fn f(#[default(1)] a: u8, b: u8) {}").err().unwrap();
        assert!(
            err.to_string()
                .contains("`b` has no default but follows `a`")
        );
        assert!(parse("fn f(a: u8, #[default(1)] b: u8, c: u8) {}").is_err());
    }
}
