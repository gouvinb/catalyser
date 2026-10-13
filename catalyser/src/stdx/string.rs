//! Type-safe validated string wrappers ensuring content rules at runtime.
//!
//! Provides [`ValidatedString`] alongside pre-configured aliases
//! [`NonEmptyString`] and [`NonBlankString`].
//!
//! # Examples
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use catalyser::stdx::string::{NonBlankString, NonEmptyString};
//!
//! // Non-empty string validation
//! let valid_non_empty = NonEmptyString::new("Hello".to_string())?;
//! assert_eq!(valid_non_empty.into_inner(), "Hello");
//! assert!(NonEmptyString::new("".to_string()).is_err());
//!
//! // Non-blank string validation (rejects whitespace-only strings)
//! let valid_non_blank = NonBlankString::new("  Hello  ".to_string())?;
//! assert_eq!(valid_non_blank.into_inner(), "  Hello  ");
//! assert!(NonBlankString::new("   \t\n ".to_string()).is_err());
//! # Ok(())
//! # }
//! ```

use crate::stdx::error::is_empty_or_blank_string::StringContentError;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
    de::Error,
};
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    marker::PhantomData,
};

/// A trait for validating and creating [`ValidatedString`] instances.
///
/// Implementors define custom validation rules for string contents.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::stdx::string::{NonEmptyValidator, StringContentValidator};
///
/// let validated =
///     NonEmptyValidator::validate_and_create("Ferris".to_string())?;
/// assert_eq!(validated.into_inner(), "Ferris");
/// # Ok(())
/// # }
/// ```
pub trait StringContentValidator: Sized {
    /// Validates the input string and creates a [`ValidatedString`] if valid.
    ///
    /// # Errors
    ///
    /// Returns [`StringContentError`] if the input fails the validation rules.
    fn validate_and_create(input: String) -> Result<ValidatedString<Self>, StringContentError>;
}

/// A wrapper around [`String`] that enforces validation rules defined by `T`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::stdx::string::NonEmptyString;
///
/// let greeting = NonEmptyString::new("Hello, Rustacean!".to_string())?;
/// assert_eq!(greeting.into_inner(), "Hello, Rustacean!");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ValidatedString<T: StringContentValidator>(String, PhantomData<T>);

impl<T: StringContentValidator> ValidatedString<T> {
    /// Validates and wraps `string` into a [`ValidatedString`].
    ///
    /// # Errors
    ///
    /// Returns [`StringContentError`] if `string` fails validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use catalyser::stdx::string::NonEmptyString;
    ///
    /// let valid = NonEmptyString::new("Valid content".to_string())?;
    /// assert_eq!(valid.into_inner(), "Valid content");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(string: String) -> Result<Self, StringContentError> {
        T::validate_and_create(string)
    }

    /// Creates a [`ValidatedString`] without validating its content.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `string` satisfies the validation
    /// invariants for `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use catalyser::stdx::string::NonEmptyString;
    ///
    /// let valid = unsafe {
    ///     NonEmptyString::new_unchecked("Known non-empty".to_string())
    /// };
    /// assert_eq!(valid.into_inner(), "Known non-empty");
    /// ```
    pub unsafe fn new_unchecked(string: String) -> Self {
        Self(string, PhantomData)
    }

    /// Consumes the wrapper and returns the underlying [`String`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use catalyser::stdx::string::NonEmptyString;
    ///
    /// let valid = NonEmptyString::new("Rust".to_string())?;
    /// assert_eq!(valid.into_inner(), "Rust");
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<T: StringContentValidator> Display for ValidatedString<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, T: StringContentValidator> Deserialize<'de> for ValidatedString<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        ValidatedString::<T>::new(string).map_err(Error::custom)
    }
}

/// A validator that requires strings to be non-empty (length > 0).
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::stdx::string::{NonEmptyValidator, StringContentValidator};
///
/// assert!(
///     NonEmptyValidator::validate_and_create("hello".to_string()).is_ok()
/// );
/// assert!(NonEmptyValidator::validate_and_create("".to_string()).is_err());
/// # Ok(())
/// # }
/// ```
pub struct NonEmptyValidator;

impl StringContentValidator for NonEmptyValidator {
    /// Validates that `input` is not empty.
    ///
    /// # Errors
    ///
    /// Returns [`StringContentError::Empty`] if `input.is_empty()`.
    fn validate_and_create(input: String) -> Result<ValidatedString<Self>, StringContentError> {
        if input.is_empty() {
            return Err(StringContentError::Empty);
        }
        Ok(ValidatedString(input, PhantomData))
    }
}

/// A validator that requires strings to contain at least one non-whitespace
/// character.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::stdx::string::{NonBlankValidator, StringContentValidator};
///
/// assert!(
///     NonBlankValidator::validate_and_create("hello".to_string()).is_ok()
/// );
/// assert!(NonBlankValidator::validate_and_create("   ".to_string()).is_err());
/// # Ok(())
/// # }
/// ```
pub struct NonBlankValidator;

impl StringContentValidator for NonBlankValidator {
    /// Validates that `input` contains at least one non-whitespace character.
    ///
    /// # Errors
    ///
    /// Returns [`StringContentError::Blank`] if `input.trim().is_empty()`.
    fn validate_and_create(input: String) -> Result<ValidatedString<Self>, StringContentError> {
        if input.trim().is_empty() {
            return Err(StringContentError::Blank(input));
        }
        Ok(ValidatedString(input, PhantomData))
    }
}

/// A [`ValidatedString`] that is guaranteed to be non-empty.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::stdx::string::NonEmptyString;
///
/// let valid = NonEmptyString::new("Hello".to_string())?;
/// assert_eq!(valid.into_inner(), "Hello");
///
/// assert!(NonEmptyString::new("".to_string()).is_err());
/// # Ok(())
/// # }
/// ```
pub type NonEmptyString = ValidatedString<NonEmptyValidator>;

/// A [`ValidatedString`] that is guaranteed to contain at least one
/// non-whitespace character.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::stdx::string::NonBlankString;
///
/// let valid = NonBlankString::new("  Hello  ".to_string())?;
/// assert_eq!(valid.into_inner(), "  Hello  ");
///
/// assert!(NonBlankString::new(" \t\n ".to_string()).is_err());
/// # Ok(())
/// # }
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
    #[cfg(feature = "serde")]
    fn test_non_empty_string_serde() {
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
    #[cfg(feature = "serde")]
    fn test_non_blank_string_serde() {
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
