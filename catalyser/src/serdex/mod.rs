//!
//! `serdex` module provides utilities and extensions for working with
//! [`serde`](https://docs.rs/serde/latest/serde/) library.
//!
//! # Submodules
//!
//! - `error`: It defines custom errors present in the `serdex` module.
//! - `number`: It is dedicated to working with numbers, providing useful types or complementary
//!   methods.
//! - `sequence`: It is dedicated to collections, providing utilities for iteration, transformation
//!   or validation.
//! - `string`: It is dedicated to strings, providing useful types or complementary methods.
//!
//! # Usage
//!
//! Import the specific functionality required into your project, e.g.:
//!
//! ```rust
//! use catalyser::serdex::{
//!     collection::NonEmptyVec,
//!     number::BoundedI32,
//!     string::NonEmptyString
//! };
//!
//! let non_empty_string = NonEmptyString::new("hello world".to_string());
//! let bounded_i32 = BoundedI32::<-10,10>::new(0);
//! let non_empty_vec = NonEmptyVec::new(vec![0, 1, 2, 3]);
//! ```

pub mod error;

pub mod collection;
pub mod number;
pub mod string;
