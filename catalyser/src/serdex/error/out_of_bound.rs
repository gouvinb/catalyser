//! This module contains an enumeration `OutOfBoundsError` for representing cases when a value is
//! out of bounds. This includes scenarios where the value is either higher or lower than the
//! specified range.
//!
//! The module also provides implementations of the `Debug` and `Display` traits for
//! `OutOfBoundsError`, allowing for detailed and user-friendly error representations in various
//! formats.

use std::fmt::{Debug, Display, Formatter};

/// An error type representing cases when a value is out of bounds.
pub enum OutOfBoundsError<T> {
    /// Indicates that the value exceeds the upper bound.
    /// Includes `(min, max, value)`:
    /// - `min`: The lower bound of the range.
    /// - `max`: The upper bound of the range.
    /// - `value`: The out-of-bounds value.
    High(T, T, T),
    /// Indicates that the value is below the lower bound.
    /// Includes `(min, max, value)`:
    /// - `min`: The lower bound of the range.
    /// - `max`: The upper bound of the range.
    /// - `value`: The out-of-bounds value.
    Low(T, T, T),
}

impl<T: Display> Debug for OutOfBoundsError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutOfBoundsError::High(min, max, value) => write!(f, "High(min = {}, max = {}, value = {})", min, max, value),
            OutOfBoundsError::Low(min, max, value) => write!(f, "Low(min = {}, max = {}, value = {})", min, max, value),
        }
    }
}

impl<T: Display> Display for OutOfBoundsError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OutOfBoundsError::High(min, max, value) => format!("{} is too high (range: {}..{})", value, min, max),
                OutOfBoundsError::Low(min, max, value) => format!("{} is too low (range: {}..{})", value, min, max),
            }
        )
    }
}
