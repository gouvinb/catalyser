//! `stdx` module provides utilities and extensions for working with Rust's standard library.
//!
//! # Submodules
//!
//! - `error`: It defines custom errors present in the `stdx` module.
//! - `primitive_number`: It is dedicated to working with numbers, providing useful types or
//!   complementary methods.
//! - `collections`: It is dedicated to collections, providing utilities for iteration,
//!   transformation or validation.
//! - `string`: It is dedicated to strings, providing useful types or complementary methods.
//! - `extension`: Contains utilities for working with scope functions like closures and
//!   higher-order functions and additional utilities for some <type|struct> manipulation.
//!
//! # Usage
//!
//! Import the specific functionality required into your project, e.g.:
//!
//! ```rust
//! use catalyser::stdx::{
//!     extension::{
//!         str_extension::MultilineStr,
//!         scope_functions_extension::TakeIf
//!     },
//!     collections::NonEmptyVec,
//!     primitive_number::BoundedI32,
//!     string::NonEmptyString
//! };
//!
//! let non_empty_string = NonEmptyString::new("hello world".to_string());
//! let bounded_i32 = BoundedI32::<-10,10>::new(0)
//!     .take_if(|it| it.is_ok());
//! let non_empty_vec = NonEmptyVec::new(vec![0, 1, 2, 3]);
//! let multiline_string = "\
//!     |Indented line 1
//!     |Indented line 2
//! ".trim_margin();
//!
//! ```

pub mod error;

pub mod extension;

pub mod primitive_number;
pub mod collections;
pub mod string;
