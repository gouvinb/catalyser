//! This module provides functionality for working with validated strings in a type-safe manner,
//! ensuring that strings adhere to specific content rules during their creation and use.
//!
//! The primary types and traits included in this module are:
//!
//! - [StringContentValidator]: A trait for defining validation rules that strings must satisfy
//!   to be considered valid. Implementations of this trait specify the validation logic.
//! - [ValidatedString]: A wrapper around [String] that ensures its content complies with the rules
//!   defined by a [StringContentValidator]. This guarantees that all instances of [ValidatedString]
//!   are known to be valid according to the associated rules.
//!
//! # Features
//!
//! - **Type-Safe String Validation**: By using generics and the [ValidatedString] type, this module
//!   statically enforces that string content is valid according to the specified validation rules.
//!
//! - **Custom Validators**: You can implement the [StringContentValidator] trait to define custom
//!   validation logic for specific use cases.
//!
//! - **Serialization and Deserialization**: [ValidatedString] supports `serde` serialization and
//!   deserialization, ensuring that validated strings remain valid through these operations.
//!
//! # Example Types
//!
//! - [NonEmptyString]: A type alias for [ValidatedString] that uses the [NonEmptyValidator] to
//!   enforce non-empty content.
//! - [NonBlankString]: A type alias for [ValidatedString] that uses the [NonBlankValidator] to
//!   enforce non-blank (non-whitespace) content.
//!
//! ## Examples
//!
//! ```rust
//! use catalyser::serdex::string::{NonEmptyString, NonBlankString};
//!
//! // Using NonEmptyString
//! let valid_non_empty = NonEmptyString::new("Hello".to_string());
//! assert!(valid_non_empty.is_ok());
//!
//! let invalid_non_empty = NonEmptyString::new("".to_string());
//! assert!(invalid_non_empty.is_err());
//!
//! // Using NonBlankString
//! let valid_non_blank = NonBlankString::new("  Hello  ".to_string());
//! assert!(valid_non_blank.is_ok());
//!
//! let invalid_non_blank = NonBlankString::new("   ".to_string());
//! assert!(invalid_non_blank.is_err());
//! ```
//!
//! # Safety
//!
//! Be cautious with the `new_unchecked` method provided by `ValidatedString`, as it allows creating
//! instances without validation. Only use this method when you are certain the input complies with
//! the expected validation rules.

use crate::serdex::error::is_empty_or_blank_string::StringContentError;
use serde::{de::Error, Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

/// A trait for validating and creating `ValidatedString` instances with specific content rules.
///
/// This trait is implemented by types that define rules for string content validation. The
/// `validate_and_create` method is used to create a `ValidatedString` if the provided input
/// meets the criteria defined in the implementor.
pub trait StringContentValidator: Sized {
    /// Validates and creates a `ValidatedString` instance if the input satisfies the content rules.
    ///
    /// # Parameters
    ///
    /// - `input`: The input string to validate.
    ///
    /// # Returns
    ///
    /// - `Ok(ValidatedString<Self>)`: If the input meets the validation criteria.
    /// - `Err(StringContentError)`: If the input fails validation, indicating the error type.
    fn validate_and_create(input: String) -> Result<ValidatedString<Self>, StringContentError>;
}

/// A wrapper around `String` that ensures its content adheres to the rules
/// defined by the `StringContentValidator` trait.
///
/// `ValidatedString` uses the generic type parameter `T` to specify the validator
/// to apply for content validation.
#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ValidatedString<T: StringContentValidator>(String, PhantomData<T>);

impl<T: StringContentValidator> ValidatedString<T> {
    /// Creates a new `ValidatedString` by validating the provided input string.
    ///
    /// # Parameters
    ///
    /// - `string`: The input string to validate and wrap.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: If the input string passes validation.
    /// - `Err(StringContentError)`: If the input string fails validation with the appropriate error.
    pub fn new(string: String) -> Result<Self, StringContentError> {
        T::validate_and_create(string)
    }

    /// Creates a new `ValidatedString` without performing any validation.
    ///
    /// # Parameters
    ///
    /// - `string`: The input string to wrap.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it assumes the caller has ensured the input is valid without
    /// verification.
    pub unsafe fn new_unchecked(string: String) -> Self {
        Self(string, PhantomData)
    }

    /// Consumes the `ValidatedString` and returns the inner `String`.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<T: StringContentValidator> Display for ValidatedString<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<'de, T: StringContentValidator> Deserialize<'de> for ValidatedString<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        ValidatedString::<T>::new(string).map_err(Error::custom)
    }
}

/// Validator that ensures a string is not empty.
pub struct NonEmptyValidator;

impl StringContentValidator for NonEmptyValidator {
    /// Validates that the input string is not empty.
    ///
    /// # Parameters
    ///
    /// - `input`: The input string to validate.
    ///
    /// # Returns
    ///
    /// - `Ok(ValidatedString<Self>)` if the input is not empty.
    /// - `Err(StringContentError::Empty)` if the input is empty.
    fn validate_and_create(input: String) -> Result<ValidatedString<Self>, StringContentError> {
        if input.is_empty() {
            return Err(StringContentError::Empty);
        }
        Ok(ValidatedString(input, PhantomData))
    }
}

/// Validator that ensures a string is not blank (not just whitespace).
pub struct NonBlankValidator;

impl StringContentValidator for NonBlankValidator {
    /// Validates that the input string is not blank.
    ///
    /// # Parameters
    ///
    /// - `input`: The input string to validate.
    ///
    /// # Returns
    ///
    /// - `Ok(ValidatedString<Self>)` if the input is not blank.
    /// - `Err(StringContentError::Blank)` if the input contains only whitespace.
    fn validate_and_create(input: String) -> Result<ValidatedString<Self>, StringContentError> {
        if input.trim().is_empty() {
            return Err(StringContentError::Blank(input.to_string()));
        }
        Ok(ValidatedString(input, PhantomData))
    }
}

/// A `ValidatedString` that ensures the content is non-empty.
///
/// # Examples
///
/// ```
/// use catalyser::serdex::string::NonEmptyString;
///
/// let valid = NonEmptyString::new("Hello".to_string());
/// assert!(valid.is_ok());
///
/// let empty = NonEmptyString::new("".to_string());
/// assert!(empty.is_err());
/// ```
pub type NonEmptyString = ValidatedString<NonEmptyValidator>;

/// A `ValidatedString` that ensures the content is non-blank.
///
/// # Examples
///
/// ```
/// use catalyser::serdex::string::NonBlankString;
///
/// let valid = NonBlankString::new("Hello".to_string());
/// assert!(valid.is_ok());
///
/// let blank = NonBlankString::new(" \t\n".to_string());
/// assert!(blank.is_err());
/// ```
pub type NonBlankString = ValidatedString<NonBlankValidator>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string_new_success() {
        let non_blank_input = "Hello".to_string();
        let non_blank_result = NonEmptyString::new(non_blank_input.clone());
        assert!(non_blank_result.is_ok());
        assert_eq!(non_blank_result.unwrap().into_inner(), non_blank_input);

        let blank_input = " \t\n".to_string();
        let blank_result = NonEmptyString::new(blank_input.clone());
        assert!(blank_result.is_ok());
        assert_eq!(blank_result.unwrap().into_inner(), blank_input);
    }

    #[test]
    fn test_non_empty_string_new_empty() {
        let input = "".to_string();
        let result = NonEmptyString::new(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_non_empty_string_new_unchecked() {
        let non_blank_input = "Hello".to_string();
        let non_blank_result = unsafe { NonEmptyString::new_unchecked(non_blank_input.clone()) };
        assert_eq!(non_blank_result.into_inner(), non_blank_input);

        let blank_input = " \t\n".to_string();
        let blank_result = unsafe { NonEmptyString::new_unchecked(blank_input.clone()) };
        assert_eq!(blank_result.into_inner(), blank_input);
    }

    #[test]
    fn test_non_blank_string_new_success() {
        let input = "Hello".to_string();
        let result = NonBlankString::new(input.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), input);
    }

    #[test]
    fn test_non_blank_string_new_blank() {
        let input = " \t\n".to_string();
        let result = NonBlankString::new(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_non_blank_string_new_unchecked() {
        let input = "Hello".to_string();
        let result = unsafe { NonBlankString::new_unchecked(input.clone()) };
        assert_eq!(result.into_inner(), input);
    }

    #[test]
    fn test_display_non_empty_string() {
        let input = "Display Test".to_string();
        let non_empty = NonEmptyString::new(input.clone()).unwrap();
        assert_eq!(format!("{}", non_empty), input);
    }

    #[test]
    fn test_display_non_blank_string() {
        let input = "Display Test".to_string();
        let non_blank = NonBlankString::new(input.clone()).unwrap();
        assert_eq!(format!("{}", non_blank), input);
    }

    #[test]
    fn test_serde_non_empty_string() {
        let input = "Serialize Test".to_string();
        let non_empty = NonEmptyString::new(input.clone()).unwrap();

        // Serialize
        let serialized = serde_json::to_string(&non_empty).unwrap();
        assert_eq!(serialized, format!("\"{}\"", input));

        // Deserialize
        let deserialized: NonEmptyString = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.into_inner(), input);
    }

    #[test]
    fn test_serde_non_blank_string() {
        let input = "Serialize Test".to_string();
        let non_blank = NonBlankString::new(input.clone()).unwrap();

        // Serialize
        let serialized = serde_json::to_string(&non_blank).unwrap();
        assert_eq!(serialized, format!("\"{}\"", input));

        // Deserialize
        let deserialized: NonBlankString = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.into_inner(), input);
    }
}
