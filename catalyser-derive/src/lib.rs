//! Derive and attribute macros for Catalyser.

mod overloads;

use crate::overloads::{
    expand,
    parse::OverloadsInput,
};
use proc_macro::TokenStream;
use proc_macro2::Span;

/// Declares default values on the parameters of a free function, methods inside an `impl` block, or a `trait`:
///
/// ### Free function
/// ```ignore
/// #[overloads]
/// pub fn add(a: i32, #[default(10)] b: i32) -> i32 {
///     a + b
/// }
///
/// // Generated items:
/// // 1. AddArgs struct with defaulted parameters and Default implementation
/// // 2. Original function `add(a: i32, b: i32)` kept with original signature
/// // 3. `add_overload(a, args: AddArgs)` overloaded function
/// // 4. `add_default(a)` shortcut function calling `add_overload(a, AddArgs::default())`
/// ```
///
/// ### `impl` block
/// ```ignore
/// struct Sub {
///     a: i32,
///     b: i32,
///     c: i32,
/// }
///
/// #[overloads]
/// impl Sub {
///     pub fn new(a: i32, #[default(10)] b: i32, #[default(10)] c: i32) -> Self {
///         Sub { a, b, c }
///     }
///
///     pub fn compute(&self, #[default(1)] factor: i32) -> i32 {
///         (self.a + self.b + self.c) * factor
///     }
/// }
///
/// // Generated items:
/// // 1. SubNewArgs and SubComputeArgs structs with Default implementation outside `impl Sub`
/// // 2. Original `new` and `compute` methods kept in `impl Sub`
/// // 3. `new_overload`, `new_default`, `compute_overload`, `compute_default` generated in `impl Sub`
/// ```
///
/// ### `trait`
/// ```ignore
/// #[overloads]
/// pub trait Mod {
///     fn modulo(&self, a: i32, #[default(10)] b: i32) -> i32;
/// }
///
/// // Generated items:
/// // 1. ModModuloArgs struct with Default implementation outside `trait Mod`
/// // 2. Original `modulo(&self, a: i32, b: i32)` required method in `trait Mod`
/// // 3. `modulo_overload(&self, a: i32, args: ModModuloArgs)` default trait method calling `self.modulo`
/// // 4. `modulo_default(&self, a: i32)` default trait method calling `self.modulo_overload`
/// ```
///
/// Parameters with `#[default]` must come after all parameters without one.
#[proc_macro_attribute]
pub fn overloads(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(Span::call_site(), "#[overloads] does not take arguments")
            .to_compile_error()
            .into();
    }
    match OverloadsInput::parse(item.into()) {
        Ok(input) => expand::expand(&input).into(),
        Err(e) => e.to_compile_error().into(),
    }
}
