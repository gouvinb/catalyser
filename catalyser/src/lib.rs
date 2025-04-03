//! `catalyser` contains submodules that are conditionally compiled based on specific features.
//!
//! ## Modules
//!
//! - `stdx`: Available when the `"std"` feature is enabled. This module provides additional
//!   utilities and extensions related to the
//!   [Rust's standard library](https://doc.rust-lang.org/stable/std/).
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
//! use catalyser::stdx::extension::{
//!     scope_functions_extension::Run,
//!     str_extension::MultilineStr,
//! };
//!
//! "
//!     |Hello
//!     |World
//! ".run(|it| it.trim_margin());
//! ```
//!
//! The `serde` feature ensures that integration with the `serde` crate is enabled.
//! This allows serialization and deserialization of types when the `serde` feature flag is active.
//!
//! ```toml
//! [dependencies]
//! catalyser = { version = "x.y.z", features = ["serde"] }
//! ```
//!

pub mod stdx;
