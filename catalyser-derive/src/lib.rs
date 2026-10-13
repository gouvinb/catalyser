//! Procedural macros for the `catalyser` crate.
//!
//! Ever wished Rust had default function arguments like Kotlin or Python, but
//! without sacrificing strong static typing and explicit semantics?
//! `catalyser-derive` provides the [`overloads`] attribute macro to bridge that
//! gap with zero runtime overhead.
//!
//! ## 30-Second Quick Start
//!
//! Attach `#[overloads]` to a free function and annotate any trailing
//! parameters with `#[default(...)]`. The macro generates a typed argument
//! struct alongside convenient helper functions:
//!
//! ```rust
//! use catalyser_derive::overloads;
//!
//! #[overloads]
//! pub fn connect(
//!     host: &str,
//!     #[default(5432)] port: u16,
//!     #[default(30)] timeout_secs: u64,
//! ) -> String {
//!     format!("{host}:{port} (timeout: {timeout_secs}s)")
//! }
//!
//! // 1. Positional call with all arguments:
//! assert_eq!(
//!     connect("localhost", 8080, 10),
//!     "localhost:8080 (timeout: 10s)"
//! );
//!
//! // 2. Default shortcut call (uses all default values):
//! assert_eq!(connect_default("localhost"), "localhost:5432 (timeout: 30s)");
//!
//! // 3. Typed overload call with named arguments struct:
//! assert_eq!(
//!     connect_overload(
//!         "localhost",
//!         ConnectArgs {
//!             timeout_secs: 15,
//!             ..ConnectArgs::default()
//!         }
//!     ),
//!     "localhost:5432 (timeout: 15s)"
//! );
//! ```
//!
//! ## Feature Flags
//!
//! | Feature   | Description                           | Dependencies |
//! |:----------|:--------------------------------------|:-------------|
//! | `default` | Core macro attribute (`#[overloads]`) | None         |
//!
//! ## Coming from Kotlin
//!
//! In Kotlin, default arguments and named parameters are first-class language
//! features:
//!
//! ```text
//! // Kotlin
//! fun greet(
//!     name: String,
//!     greeting: String = "Hello",
//!     punctuation: String = "!",
//! ): String = "$greeting, $name$punctuation"
//!
//! greet("Kotlin")                      // "Hello, Kotlin!"
//! greet("Kotlin", "Hi")                // "Hi, Kotlin!"
//! greet("Kotlin", punctuation = "!!!") // "Hello, Kotlin!!!"
//! ```
//!
//! In Rust with `catalyser-derive`:
//!
//! ```rust
//! use catalyser_derive::overloads;
//!
//! #[overloads]
//! pub fn greet(
//!     name: &str,
//!     #[default("Hello".to_string())] greeting: String,
//!     #[default("!".to_string())] punctuation: String,
//! ) -> String {
//!     format!("{greeting}, {name}{punctuation}")
//! }
//!
//! // All defaults:
//! assert_eq!(greet_default("Rust"), "Hello, Rust!");
//!
//! // Positional:
//! assert_eq!(greet("Rust", "Hi".to_string(), "!".to_string()), "Hi, Rust!");
//!
//! // Overload struct for specific overrides:
//! assert_eq!(
//!     greet_overload(
//!         "Rust",
//!         GreetArgs {
//!             punctuation: "!!!".to_string(),
//!             ..GreetArgs::default()
//!         }
//!     ),
//!     "Hello, Rust!!!"
//! );
//! ```
//!
//! ## End-to-End Overview and Limitations
//!
//! ### Supported Targets
//!
//! The `#[overloads]` attribute can be placed on:
//!
//! 1. Free functions: Emits `<FnName>Args` (implementing [`Default`]),
//!    `<fn_name>_overload`, and `<fn_name>_default`.
//! 2. Inherent `impl` blocks: Annotating `#[overloads]` on an `impl Type` block
//!    transforms all methods inside that contain `#[default(...)]` parameters.
//!    Generates `<Type><MethodName>Args` structs outside the block and
//!    `<method>_overload` / `<method>_default` methods inside the block.
//! 3. Trait definitions: Annotating `#[overloads]` on a `trait TraitName`
//!    definition generates `<TraitName><MethodName>Args` structs outside the
//!    trait, and adds default trait methods for `<method>_overload` and
//!    `<method>_default`.
//!
//! ### Limitations and Design Choices
//!
//! - Declaration order: Default expressions are evaluated in declaration order
//!   inside the generated [`Default::default`] implementation.
//! - Ordering rule: All parameters annotated with `#[default(...)]` must be
//!   placed after all required parameters without `#[default]`.
//! - Import by name: The generated struct and helper functions live in the same
//!   scope as the annotated function. When importing from another module,
//!   remember to import `<FnName>Args`, `<fn_name>_overload`, and
//!   `<fn_name>_default` if you intend to use them.
//! - Unsupported constructs: `#[overloads]` cannot be placed on standalone
//!   methods without annotating their enclosing `impl` or `trait` block.
//!   Destructured patterns (e.g. `(a, b): (i32, i32)`) and `impl Trait`
//!   parameters cannot carry `#[default(...)]`.

mod overloads;

use crate::overloads::{
    expand,
    parse::OverloadsInput,
};
use proc_macro::TokenStream;
use proc_macro2::Span;

/// Generates default-argument structs and overload helper functions.
///
/// Annotate free functions, inherent `impl` blocks, or `trait` definitions
/// with `#[overloads]` and mark trailing parameters with `#[default(<expr>)]`.
///
/// # Examples
///
/// ## Free Function
///
/// ```rust
/// use catalyser_derive::overloads;
///
/// #[overloads]
/// pub fn calculate_total(
///     subtotal: u32,
///     #[default(5)] tax_percent: u32,
///     #[default(0)] discount: u32,
/// ) -> u32 {
///     subtotal + (subtotal * tax_percent / 100) - discount
/// }
///
/// // Positional call
/// assert_eq!(calculate_total(100, 10, 5), 105);
///
/// // Shortcut with all defaults
/// assert_eq!(calculate_total_default(100), 105);
///
/// // Typed overload struct
/// let total = calculate_total_overload(
///     100,
///     CalculateTotalArgs {
///         discount: 10,
///         ..CalculateTotalArgs::default()
///     },
/// );
/// assert_eq!(total, 95);
/// ```
///
/// ## Inherent `impl` Block
///
/// ```rust
/// use catalyser_derive::overloads;
///
/// struct Server {
///     host: String,
///     port: u16,
/// }
///
/// #[overloads]
/// impl Server {
///     pub fn new(host: String, #[default(8080)] port: u16) -> Self {
///         Self { host, port }
///     }
///
///     pub fn ping(&self, #[default(3)] retries: u32) -> String {
///         format!("Pinging {}:{} {} times", self.host, self.port, retries)
///     }
/// }
///
/// let server = Server::new_default("127.0.0.1".to_string());
/// assert_eq!(server.ping_default(), "Pinging 127.0.0.1:8080 3 times");
/// ```
///
/// ## `trait` Definition
///
/// ```rust
/// use catalyser_derive::overloads;
///
/// #[overloads]
/// pub trait Logger {
///     fn log(
///         &self,
///         message: &str,
///         #[default("INFO".to_string())] level: String,
///     );
/// }
/// ```
///
/// # Errors
///
/// This macro does not panic at compile time. Instead, it emits helpful
/// compiler errors (`syn::Error`) in the following scenarios:
///
/// - Passing attributes or arguments directly to `#[overloads]` (for example,
///   `#[overloads(arg)]`).
/// - Placing required parameters after defaulted parameters.
/// - Applying `#[default(...)]` to `self` receivers, destructured patterns, or
///   `impl Trait` parameters.
/// - Marking `const` or `unsafe` functions with `#[overloads]`.
/// - Placing `#[overloads]` on unsupported items such as `impl Trait for Type`
///   blocks.
///
/// # Panics
///
/// This macro never panics. All validation errors are returned as compiler
/// diagnostics.
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
