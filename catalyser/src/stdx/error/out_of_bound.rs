//! Out-of-bounds validation error types.
//!
//! Provides [`OutOfBoundsError`] for handling numeric range validation errors.

use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
};

/// An error type representing cases where a value falls outside an expected
/// range `[min, max]`.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::error::out_of_bound::OutOfBoundsError;
///
/// let high_err: OutOfBoundsError<i32> = OutOfBoundsError::High(0, 100, 150);
/// assert_eq!(high_err.to_string(), "150 is too high (range: 0..100)");
///
/// let low_err: OutOfBoundsError<i32> = OutOfBoundsError::Low(0, 100, -5);
/// assert_eq!(low_err.to_string(), "-5 is too low (range: 0..100)");
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OutOfBoundsError<T> {
    /// Indicates that the value exceeds the upper bound.
    ///
    /// Includes `(min, max, value)`:
    ///
    /// - `min`: The lower bound of the range.
    /// - `max`: The upper bound of the range.
    /// - `value`: The out-of-bounds value.
    High(T, T, T),
    /// Indicates that the value is below the lower bound.
    ///
    /// Includes `(min, max, value)`:
    ///
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

impl<T: Debug + Display> Error for OutOfBoundsError<T> {}
