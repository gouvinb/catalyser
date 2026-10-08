//! `catalyser` contains submodules that are conditionally compiled based on specific features.
//!
//! ## Modules and Features
//!
//! - `stdx`: Provides additional utilities and extensions for standard types.
//! - `serde`: Enables integration with `serde` for serialization/deserialization of utility types.
//! - `derive`: Enables the `#[overloads]` macro attribute to support default parameters and typed
//!   overload structures on free functions, `impl` blocks, and `traits`.
//!
//! ## Usage Examples
//!
//! ### Scope functions and String extensions
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
//! ### Default and Overloaded Parameters (`derive` feature)
//!
//! ```rust
//! use catalyser::overloads;
//!
//! #[overloads]
//! pub fn connect(host: &str, #[default(5432)] port: u16) -> String {
//!     format!("{host}:{port}")
//! }
//!
//! // 1. Full positional call
//! assert_eq!(connect("localhost", 8080), "localhost:8080");
//!
//! // 2. Typed overload using generated struct
//! assert_eq!(connect_overload("localhost", ConnectArgs { port: 3000 }), "localhost:3000");
//!
//! // 3. Shortcut for default arguments
//! assert_eq!(connect_default("localhost"), "localhost:5432");
//! ```
//!
//! ```toml
//! [dependencies]
//! catalyser = { version = "x.y.z", features = ["derive", "serde"] }
//! ```

pub mod stdx;

#[cfg(feature = "derive")]
pub use catalyser_derive::overloads;
