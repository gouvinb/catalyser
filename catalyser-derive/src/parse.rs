//! Model of a free function whose parameters may carry `#[default(expr)]`.
//!
//! `fn f(a: u8, #[default(5)] b: u8)` is syntactically valid Rust, so we can parse a
//! plain `syn::ItemFn` and then extract the `#[default(..)]` attributes from the
//! parameters. (The more Kotlin-like `b: u8 = 5` is rejected by rustc itself before an
//! attribute macro ever runs, hence the attribute form.)
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
    Pat,
    Result,
    Type,
};

/// A named parameter with its optional default value.
pub struct Param {
    pub ident: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

/// The cleaned function (without `#[default]`) and the model of its parameters.
pub struct OverloadsFn {
    pub func: ItemFn,
    pub params: Vec<Param>,
}

impl OverloadsFn {
    pub fn from_item(mut func: ItemFn) -> Result<Self> {
        // Qualifiers are checked on tokens rather than through `Signature` fields:
        // those field names have changed across syn releases (e.g. `unsafety`).
        for tt in func.sig.to_token_stream() {
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
        if let Some(t) = &func.sig.variadic {
            return Err(Error::new_spanned(
                t,
                "#[overloads] does not support variadic functions",
            ));
        }

        let mut params = Vec::new();
        let mut seen = HashSet::new();
        // First parameter carrying a default, used to enforce the "defaults last" rule.
        let mut first_default: Option<Ident> = None;

        for arg in func.sig.inputs.iter_mut() {
            let pat_type = match arg {
                FnArg::Typed(pt) => pt,
                other => {
                    return Err(Error::new_spanned(
                        other,
                        "#[overloads] only supports free functions (no `self` receiver)",
                    ));
                },
            };

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
            pat_type.attrs = kept;

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
            });
        }

        Ok(Self { func, params })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(f: &OverloadsFn) -> String {
        f.func.to_token_stream().to_string()
    }

    fn parse(src: &str) -> Result<OverloadsFn> {
        OverloadsFn::from_item(syn::parse_str::<ItemFn>(src)?)
    }

    #[test]
    fn extracts_overloads() {
        let f = parse("pub fn connect(host: &str, #[default(5432)] port: u16) {}").unwrap();
        assert_eq!(f.params.len(), 2);
        assert!(f.params[0].default.is_none());
        assert!(f.params[1].default.is_some());
        assert_eq!(f.params[1].ident, "port");
    }

    #[test]
    fn default_can_be_a_complex_expression() {
        let f = parse("fn f(#[default(Duration::from_secs(5 * 60))] t: Duration) {}").unwrap();
        assert!(f.params[0].default.is_some());
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
