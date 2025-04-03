//! This module contains an enumeration `StringContentError` for representing possible errors
//! related to validation of string content. This includes checks for empty strings or strings that
//! consist only of blank spaces.
//!
//! The module also provides implementations of the `Debug` and `Display` traits for
//! `StringContentError`, enabling error representation in different formats.

use std::fmt::{Debug, Display, Formatter};

/// Represents possible errors related to string content validation.
pub enum StringContentError {
    /// Indicates the string is empty.
    Empty,
    /// Indicates the string contains only blank spaces. Includes the original string.
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
        write!(
            f,
            "{}",
            match self {
                StringContentError::Empty => "string is empty".to_string(),
                StringContentError::Blank(value) => format!("string is blank (content: `{}`)", value),
            }
        )
    }
}
