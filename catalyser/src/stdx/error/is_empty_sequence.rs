//! This module contains an enumeration `SequenceContentError` for representing possible errors
//! related to the validation of sequences. This includes checks for cases such as empty sequences.
//!
//! The module also provides implementations of the `Debug` and `Display` traits for
//! `SequenceContentError`, enabling error representation in different formats.

use std::fmt::{Debug, Display, Formatter};

/// Represents possible errors related to sequence validation.
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
        write!(
            f,
            "{}",
            match self {
                SequenceContentError::Empty => "sequence is empty".to_string(),
            }
        )
    }
}
