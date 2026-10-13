//! String content validation error types.
//!
//! Provides [`StringContentError`] for handling empty or blank string
//! validation errors.

use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
};

/// Represents errors encountered during string content validation.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::error::is_empty_or_blank_string::StringContentError;
///
/// let empty_err = StringContentError::Empty;
/// assert_eq!(empty_err.to_string(), "string is empty");
///
/// let blank_err = StringContentError::Blank("   ".to_string());
/// assert_eq!(blank_err.to_string(), "string is blank (content: `   `)");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub enum StringContentError {
    /// Indicates that the validated string is completely empty.
    Empty,
    /// Indicates that the validated string contains only whitespace
    /// characters.
    Blank(String),
}

impl Debug for StringContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StringContentError::Empty => write!(f, "Empty"),
            StringContentError::Blank(value) => write!(f, "Blank(value = `{}`)", value.escape_debug()),
        }
    }
}

impl Display for StringContentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StringContentError::Empty => write!(f, "string is empty"),
            StringContentError::Blank(value) => write!(f, "string is blank (content: `{value}`)"),
        }
    }
}

impl Error for StringContentError {}
