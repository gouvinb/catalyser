//! Sequence validation error types.
//!
//! Provides [`SequenceContentError`] for handling empty collection
//! validation errors.

use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
};

/// Represents errors encountered during sequence validation.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::error::is_empty_sequence::SequenceContentError;
///
/// let err = SequenceContentError::Empty;
/// assert_eq!(err.to_string(), "sequence is empty");
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SequenceContentError {
    /// Indicates that the sequence is empty.
    Empty,
}

impl Debug for SequenceContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceContentError::Empty => write!(f, "Empty"),
        }
    }
}

impl Display for SequenceContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceContentError::Empty => write!(f, "sequence is empty"),
        }
    }
}

impl Error for SequenceContentError {}
