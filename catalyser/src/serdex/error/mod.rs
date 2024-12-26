//! This module defines custom errors present in the `serdex` module.
//!
//! These custom errors are used to handle specific cases where the default error types are not
//! sufficient, providing more context and control over the error handling process.
//!
//! # Submodules
//!
//! - `is_empty_or_blank_string`: Handles errors arising from blank or empty strings.
//! - `is_empty_sequence`: Handles errors arising from empty sequences.
//! - `out_of_bound`: Handles errors arising from out-of-bound numbers.

pub mod is_empty_or_blank_string;
pub mod is_empty_sequence;
pub mod out_of_bound;
