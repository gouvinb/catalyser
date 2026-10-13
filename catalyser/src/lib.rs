//! Extensions and ergonomics helpers to simplify and enhance Rust development.
//!
//! `catalyser` brings expressive patterns—inspired by Kotlin and standard
//! library extensions in modern languages—into type-safe, idiomatic Rust. It
//! includes scope functions
//! ([`Apply`](stdx::extension::scope_functions_extension::Apply),
//! [`Run`](stdx::extension::scope_functions_extension::Run),
//! [`TakeIf`](stdx::extension::scope_functions_extension::TakeIf)), multiline
//! string formatting
//! ([`MultilineStr`](stdx::extension::str_extension::MultilineStr)),
//! validated and bounded types
//! ([`NonEmptyString`](stdx::string::NonEmptyString),
//! [`NonEmptyVec`](stdx::collections::NonEmptyVec),
//! [`BoundedI32`](stdx::primitive_number::BoundedI32)), and optional function
//! overloads with default parameters.
//!
//! ## Examples
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use catalyser::stdx::{
//!     collections::NonEmptyVec,
//!     extension::{
//!         scope_functions_extension::{Apply, Run, TakeIf},
//!         str_extension::MultilineStr,
//!     },
//!     primitive_number::BoundedI32,
//!     string::NonEmptyString,
//! };
//!
//! // Scope functions: chain transformations and mutations cleanly
//! let greeting = "Hello, world!"
//!     .run(|s| s.to_uppercase())
//!     .apply(|s| s.push('!'));
//! assert_eq!(greeting, "HELLO, WORLD!!");
//!
//! // Multiline strings: trim margin prefixes
//! let message = "
//!     |Welcome to Catalyser!
//!     |Enjoy idiomatic ergonomics.
//! ".trim_margin();
//! assert_eq!(message, "Welcome to Catalyser!\nEnjoy idiomatic ergonomics.");
//!
//! // Type-safe validated strings and collections
//! let valid_str = NonEmptyString::new("Rustacean".to_string())?;
//! assert_eq!(valid_str.into_inner(), "Rustacean");
//!
//! let list = NonEmptyVec::new(vec![10, 20, 30])?;
//! assert_eq!(list.into_inner(), vec![10, 20, 30]);
//!
//! // Bounded numeric values validated at runtime
//! let speed = BoundedI32::<0, 120>::new(80)?;
//! assert_eq!(speed.into_inner(), 80);
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! - `derive`: Enables the `overloads` macro attribute for default parameters
//!   and argument structs. Extra dependency: `catalyser-derive`.
//! - `serde`: Enables `Serialize` and `Deserialize` implementations on
//!   validated types (`NonEmptyString`, `NonEmptyCollection`, `BoundedNum`).
//!   Extra dependency: `serde`.
//!
//! ## Coming from Kotlin
//!
//! If you are coming from Kotlin, you will find familiar constructs ported to
//! idiomatic Rust:
//!
//! - `value.let { ... }` / `value.run { ... }`:
//!   [`Run`](stdx::extension::scope_functions_extension::Run)
//! - `value.apply { ... }`:
//!   [`Apply`](stdx::extension::scope_functions_extension::Apply)
//! - `value.takeIf { ... }`:
//!   [`TakeIf`](stdx::extension::scope_functions_extension::TakeIf)
//! - `value.takeUnless { ... }`:
//!   [`TakeUnless`](stdx::extension::scope_functions_extension::TakeUnless)
//! - `repeat(times) { ... }`:
//!   [`repeat`](stdx::extension::scope_functions_extension::repeat)
//! - `"""...""".trimMargin()`:
//!   [`MultilineStr`](stdx::extension::str_extension::MultilineStr)
//! - `fun f(a: Int, b: Int = 10)`:
//!   [`overloads`](../catalyser_derive/attr.overloads.html)

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod stdx;

/// Attribute macro supporting default arguments and generated overload
/// structs on functions, `impl` blocks, and traits.
///
/// For comprehensive documentation, syntax rules, and examples, see
/// [`catalyser_derive::overloads`].
///
/// # Examples
///
/// ```rust
/// use catalyser::overloads;
///
/// #[overloads]
/// pub fn connect(host: &str, #[default(5432)] port: u16) -> String {
///     format!("{host}:{port}")
/// }
///
/// // Positional call
/// assert_eq!(connect("localhost", 8080), "localhost:8080");
///
/// // Shortcut for default arguments
/// assert_eq!(connect_default("localhost"), "localhost:5432");
///
/// // Typed overload struct
/// assert_eq!(
///     connect_overload(
///         "localhost",
///         ConnectArgs {
///             port: 3000,
///             ..ConnectArgs::default()
///         }
///     ),
///     "localhost:3000"
/// );
/// ```
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use catalyser_derive::overloads;
