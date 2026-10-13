//! Custom error types used across the [`stdx`](crate::stdx) module.
//!
//! Provides descriptive, domain-specific errors implementing
//! [`std::error::Error`]:
//!
//! - [`is_empty_or_blank_string::StringContentError`]: Errors arising from
//!   empty or whitespace-only strings.
//! - [`is_empty_sequence::SequenceContentError`]: Errors arising from
//!   unexpectedly empty sequences.
//! - [`out_of_bound::OutOfBoundsError`]: Errors arising from out-of-range
//!   numeric values.

pub mod is_empty_or_blank_string;
pub mod is_empty_sequence;
pub mod out_of_bound;
