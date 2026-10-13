//! Standard library extensions and ergonomic utilities.
//!
//! # Submodules
//!
//! - [`error`]: Custom domain error types implementing [`std::error::Error`].
//! - [`primitive_number`]: Bounded integer and floating-point types enforced
//!   at runtime.
//! - [`collections`]: Non-empty collection wrappers ensuring non-empty
//!   invariants at runtime.
//! - [`string`]: Validated string types such as
//!   [`NonEmptyString`](string::NonEmptyString) and
//!   [`NonBlankString`](string::NonBlankString).
//! - [`extension`]: Kotlin-inspired scope functions
//!   ([`Apply`](extension::scope_functions_extension::Apply),
//!   [`Run`](extension::scope_functions_extension::Run),
//!   [`TakeIf`](extension::scope_functions_extension::TakeIf)) and string
//!   utilities ([`MultilineStr`](extension::str_extension::MultilineStr)).
//!
//! # Examples
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use catalyser::stdx::{
//!     collections::NonEmptyVec,
//!     extension::{
//!         scope_functions_extension::TakeIf,
//!         str_extension::MultilineStr,
//!     },
//!     primitive_number::BoundedI32,
//!     string::NonEmptyString,
//! };
//!
//! let non_empty_string = NonEmptyString::new("hello world".to_string())?;
//! let bounded_i32 = BoundedI32::<-10, 10>::new(0)?
//!     .take_if(|it| it.into_inner() == 0);
//! assert!(bounded_i32.is_some());
//!
//! let non_empty_vec = NonEmptyVec::new(vec![0, 1, 2, 3])?;
//! assert_eq!(non_empty_vec.into_inner().len(), 4);
//!
//! let multiline_string = "\
//!     |Indented line 1
//!     |Indented line 2
//! ".trim_margin();
//! assert_eq!(multiline_string, "Indented line 1\nIndented line 2");
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod extension;

pub mod collections;
pub mod primitive_number;
pub mod string;
