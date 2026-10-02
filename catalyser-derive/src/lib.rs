//! Derive and attribute macros for Catalyser.

mod expand;
mod parse;

use crate::parse::OverloadsFn;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    ItemFn,
    parse_macro_input,
};

/// Declares default values on the parameters of a free function:
///
/// ```ignore
/// #[overloads]
/// fn connect(host: &str, #[default(5432)] port: u16) { /* .. */ }
///
/// use net::connect; // imports the function *and* the macro
/// connect!("localhost");
/// connect!("localhost", 1234);
/// ```
///
/// Parameters with `#[default]` must come after all parameters without one.
///
/// The plain function keeps working exactly as before; the `connect!` macro is the
/// entry point that fills in omitted arguments.
#[proc_macro_attribute]
pub fn overloads(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(Span::call_site(), "#[overloads] does not take arguments")
            .to_compile_error()
            .into();
    }
    let item = parse_macro_input!(item as ItemFn);
    match OverloadsFn::from_item(item) {
        Ok(def) => expand::expand(&def).into(),
        Err(e) => e.to_compile_error().into(),
    }
}
