//! `catalyser` contains submodules that are conditionally compiled based on specific features.
//!
//! ## Modules
//!
//! - `stdx`: Available when the `"std"` feature is enabled. This module provides additional
//!   utilities and extensions related to the
//!   [Rust's standard library](https://doc.rust-lang.org/stable/std/).
//! - `serdex`: Available when the `"serde"` feature is enabled. This module provides utilities
//!   and extensions for working with [`serde`](https://docs.rs/serde/latest/serde/).
//!
//! ## Usage Examples
//!
//! To use the `stdx` module, make sure the `"std"` feature is enabled in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! catalyser = { version = "x.y.z", features = ["std"] }
//! ```
//!
//! Then, you can import and use items from the `stdx` module:
//!
//! ```rust
//! use catalyser::stdx::{
//!     scope_functions_extension::Let,
//!     str_extension::MultilineStr,
//! };
//!
//! "
//!     |Hello
//!     |World
//! ".let_do(|it| it.trim_margin())
//! ```
//!
//! Similarly, for the `serdex` module, ensure the `"serde"` feature is enabled:
//!
//! ```toml
//! [dependencies]
//! catalyser = { version = "x.y.z", features = ["serde"] }
//! ```
//!
//! Then, you can import and use items from the `serdex` module:
//!
//! ```rust
//! use catalyser::serdex::*;
//!
//! struct MyStruct(collection::NonEmptyHashMap<string::NonBlankString, number::BoundedI32<0, 100>>);
//! ```
//!

#[cfg(feature = "std")]
pub mod stdx;

#[cfg(feature = "serde")]
pub mod serdex;
