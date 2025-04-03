//! This module provides a mechanism to define and work with numeric types that are bounded between
//! a specified minimum and maximum value, both for integer and floating-point types. This ensures
//! that invalid values outside the range are caught at runtime, providing additional safety and
//! correctness in computations.
//!
//! ## Features
//!
//! - **Integer Bounds:** Create bounded integer types with customizable ranges.
//! - **Floating-point Bounds:** Define bounded floating-point types with predetermined ranges.
//! - **Serialization:** Supports `serde` for (de)serializing the bounded numbers.
//! - **Validation:** Provides utilities for creating bounded numbers and validating inputs at
//!   runtime.
//!
//! ## Usage
//!
//! ### Generating Bounded Integer Types
//!
//! Use the `generate_bounded_num` macro to define a structured integer type bounded by specified
//! minimum and maximum values.
//!
//! ```rust
//! use serde::{de::Error, Deserialize, Serialize};
//! use std::fmt::{Display, Formatter};
//! use catalyser::stdx::{
//!     error::out_of_bound::OutOfBoundsError,
//!     primitive_number::BoundedI8
//! };
//!
//! // Successfully create a bounded integer
//! let value: BoundedI8<0, 100> = BoundedI8::new(42).unwrap();
//! assert_eq!(value.into_inner(), 42);
//!
//! // Attempt to create a value outside the bounds
//! let invalid_value = BoundedI8::<0, 100>::new(101);
//! assert!(invalid_value.is_err());
//! ```
//!
//! ### Generating Bounded Floating-Point Types
//!
//! Use the `generate_bounded_float` macro to create a floating-point type that is restricted to a
//! specific range.
//!
//! ```rust
//! use serde::{de::Error, Deserialize, Serialize};
//! use std::fmt::{Display, Formatter};
//! use catalyser::{
//!     generate_bounded_float,
//!     stdx::{
//!         error::out_of_bound::OutOfBoundsError,
//!         primitive_number::BoundedI8,
//!     }
//! };
//!
//! generate_bounded_float!(BoundedF32, 0.0, 100.0, f32);
//!
//! // Successfully create a bounded float
//! let bounded_float = BoundedF32::new(75.5).unwrap();
//! assert_eq!(bounded_float.into_inner(), 75.5);
//!
//! // Attempt to create a bounded float outside the range
//! let invalid_float = BoundedF32::new(150.0);
//! assert!(invalid_float.is_err());
//! ```
//!
//! ## Customization
//!
//! The library allows you to easily expand the set of bounded numeric types by invoking the
//! provided macros in your own codebase. Both integer and floating-point types are supported, and
//! the bounds can be fully customized.

use crate::stdx::error::out_of_bound::OutOfBoundsError;
use serde::{de::Error, Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[macro_export]
macro_rules! generate_bounded_num {
    ($name:ident, $type_name:ident) => {
        #[doc = concat!("A [`", stringify!($name), "`](", stringify!($name), ")` that's bounded between two values (inclusive)")]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde(transparent))]
        pub struct $name<const MIN: $type_name, const MAX: $type_name>($type_name);

        impl<const MIN: $type_name, const MAX: $type_name> $name<MIN, MAX> {
            #[doc = concat!("Creates a new [`", stringify!($name), "`](Self) from `num`.\n")]
            ///
            /// # Parameters
            ///
            /// - `num` - The value to be validated and wrapped as a new bounded number.
            ///
            /// # Returns
            ///
            /// - `Ok(Self)` if `num` is within `MIN` and `MAX`.
            /// - `Err(OutOfBoundsError)` if `num` is outside `MIN` and `MAX`.
            #[allow(unused)]
            pub fn new(num: $type_name) -> Result<Self, OutOfBoundsError<$type_name>> {
                if MIN > num {
                    Err(OutOfBoundsError::Low(MIN, MAX, num))
                } else if num > MAX {
                    Err(OutOfBoundsError::High(MIN, MAX, num))
                } else {
                    Ok(Self(num))
                }
            }

            #[doc = concat!("Create a new clamped [`", stringify!($name), "`] (unchecked). Assumes `num` is already clamped between `MIN` and `MAX` (inclusive).\n")]
            ///
            /// # Parameters
            ///
            /// - `num` - The value to be wrapped as a new bounded number. This must already be
            ///   validated.
            ///
            /// # Returns
            ///
            /// - `Self` - A new instance of the bounded number wrapping the provided value.
            ///
            /// # Safety
            ///
            /// This function is marked unsafe because it assumes that `num` is already checked to
            /// be within the range from `MIN` to `MAX` (inclusive). The caller must ensure that
            /// this assumption is upheld.
            #[allow(unused)]
            pub unsafe fn new_unchecked(num: $type_name) -> Self {
                Self(num)
            }

            /// Returns the value as a primitive type
            #[allow(unused)]
            pub fn into_inner(self) -> $type_name {
                self.0
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, const MIN: $type_name, const MAX: $type_name> Deserialize<'de> for $name<MIN, MAX> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                match Deserialize::deserialize(deserializer).map(Self::new)? {
                    Ok(result) => Ok(result),
                    Err(err) => Err(D::Error::custom(err)),
                }
            }
        }

        impl<const MIN: $type_name, const MAX: $type_name> Display for $name<MIN, MAX> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

generate_bounded_num!(BoundedI8, i8);
generate_bounded_num!(BoundedI16, i16);
generate_bounded_num!(BoundedI32, i32);
generate_bounded_num!(BoundedI64, i64);
generate_bounded_num!(BoundedI128, i128);
generate_bounded_num!(BoundedIsize, isize);

generate_bounded_num!(BoundedU8, u8);
generate_bounded_num!(BoundedU16, u16);
generate_bounded_num!(BoundedU32, u32);
generate_bounded_num!(BoundedU64, u64);
generate_bounded_num!(BoundedU128, u128);
generate_bounded_num!(BoundedUsize, usize);

/// A macro to generate a bounded float type with specified minimum and maximum values.
///
/// This macro creates a type that represents a floating-point number bounded between
/// specific minimum and maximum values. The type provides functionality to safely create
/// an instance, validate its bounds, and perform serialization and deserialization.
///
/// # Example
///
/// ```rust
/// use serde::{de::Error, Deserialize, Serialize};
/// use std::fmt::{Display, Formatter};
/// use catalyser::{
///     generate_bounded_float,
///     stdx::{
///         error::out_of_bound::OutOfBoundsError,
///         primitive_number::BoundedI8,
///     }
/// };
///
/// generate_bounded_float!(BoundedF32, 0.0, 100.0, f32);
///
/// // Successfully create a bounded float within the range
/// let bounded = BoundedF32::new(50.0).unwrap();
/// assert_eq!(bounded.into_inner(), 50.0);
///
/// // Attempt to create a bounded float outside the range
/// let result = BoundedF32::new(150.0);
/// assert!(result.is_err());
/// ```
///
/// # Parameters
///
/// - `$name`: The name of the generated bounded float type.
/// - `$min`: The minimum value allowed for the type.
/// - `$max`: The maximum value allowed for the type.
/// - `$type_name`: The primitive floating-point type (e.g., `f32`, `f64`).
///
/// # Safety
///
/// The `new_unchecked` method assumes that the value provided is already clamped between
/// the minimum and maximum values. If this is not guaranteed, it may lead to undefined behavior.
#[macro_export]
macro_rules! generate_bounded_float {
    ($name:ident, $min:expr, $max:expr, $type_name:ident) => {
        #[doc = concat!("A [`", stringify!($name), "`](", stringify!($name), ")` that's bounded between two values (inclusive)")]
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        #[repr(transparent)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde(transparent))]
        pub struct $name($type_name);

        impl $name {
            pub const MIN: $type_name = $min;
            pub const MAX: $type_name = $max;

            #[doc = concat!("Creates a new [`", stringify!($name), "`](Self) from `num`.\n")]
            ///
            /// # Parameters
            ///
            /// - `num` - The value to be validated and wrapped as a new bounded number.
            ///
            /// # Returns
            ///
            /// - `Ok(Self)` if `num` is within `MIN` and `MAX`.
            /// - `Err(OutOfBoundsError)` if `num` is outside `MIN` and `MAX`.
            #[allow(unused)]
            pub fn new(num: $type_name) -> Result<Self, OutOfBoundsError<$type_name>> {
                if Self::MIN > num {
                    Err(OutOfBoundsError::Low(Self::MIN, Self::MAX, num))
                } else if num > Self::MAX {
                    Err(OutOfBoundsError::High(Self::MIN, Self::MAX, num))
                } else {
                    Ok(Self(num))
                }
            }

            #[doc = concat!("Create a new clamped [`", stringify!($name), "`] (unchecked). Assumes `num` is already clamped between `MIN` and `MAX` (inclusive).\n")]
            ///
            /// # Parameters
            ///
            /// - `num` - The value to be wrapped as a new bounded number. This must already be
            ///   validated.
            ///
            /// # Returns
            ///
            /// - `Self` - A new instance of the bounded number wrapping the provided value.
            ///
            /// # Safety
            ///
            /// This function is marked unsafe because it assumes that `num` is already checked to
            /// be within the range from `MIN` to `MAX` (inclusive). The caller must ensure that
            /// this assumption is upheld.
            #[allow(unused)]
            pub unsafe fn new_unchecked(num: $type_name) -> Self {
                Self(num)
            }

            /// Returns the value as a primitive type
            #[allow(unused)]
            pub fn into_inner(self) -> $type_name {
                self.0
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = $type_name::deserialize(deserializer)?;
                $name::new(value).map_err(D::Error::custom)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use serde_json;

    #[test]
    fn test_bounded_num() {
        macro_rules! generate_bounded_num_test {
            ($type_name:ident) => {
                let value: Result<$type_name<1, 100>, _> = $type_name::new(50);
                assert!(value.is_ok());
                assert_eq!(value.unwrap().into_inner(), 50);

                let too_low: Result<$type_name<1, 100>, _> = $type_name::new(0);
                assert!(too_low.is_err());

                let too_high: Result<$type_name<1, 100>, _> = $type_name::new(101);
                assert!(too_high.is_err());
            };
        }

        generate_bounded_num_test!(BoundedI8);
        generate_bounded_num_test!(BoundedI16);
        generate_bounded_num_test!(BoundedI32);
        generate_bounded_num_test!(BoundedI64);
        generate_bounded_num_test!(BoundedI128);
        generate_bounded_num_test!(BoundedIsize);

        generate_bounded_num_test!(BoundedU8);
        generate_bounded_num_test!(BoundedU16);
        generate_bounded_num_test!(BoundedU32);
        generate_bounded_num_test!(BoundedU64);
        generate_bounded_num_test!(BoundedU128);
        generate_bounded_num_test!(BoundedUsize);
    }

    #[test]
    fn test_bounded_num_unchecked() {
        unsafe {
            macro_rules! generate_bounded_num_unchecked_test {
                ($type_name:ident) => {
                    let value = $type_name::<0, 100>::new_unchecked(50);
                    assert_eq!(value.into_inner(), 50);
                };
            }

            generate_bounded_num_unchecked_test!(BoundedI8);
            generate_bounded_num_unchecked_test!(BoundedI16);
            generate_bounded_num_unchecked_test!(BoundedI32);
            generate_bounded_num_unchecked_test!(BoundedI64);
            generate_bounded_num_unchecked_test!(BoundedI128);
            generate_bounded_num_unchecked_test!(BoundedIsize);

            generate_bounded_num_unchecked_test!(BoundedU8);
            generate_bounded_num_unchecked_test!(BoundedU16);
            generate_bounded_num_unchecked_test!(BoundedU32);
            generate_bounded_num_unchecked_test!(BoundedU64);
            generate_bounded_num_unchecked_test!(BoundedU128);
            generate_bounded_num_unchecked_test!(BoundedUsize);
        }
    }

    #[test]
    fn test_bounded_num_display() {
        macro_rules! generate_bounded_num_display_test {
            ($type_name:ident) => {
                let value = $type_name::<0, 100>::new(50).unwrap();
                assert_eq!(value.to_string(), "50");
            };
        }

        generate_bounded_num_display_test!(BoundedI8);
        generate_bounded_num_display_test!(BoundedI16);
        generate_bounded_num_display_test!(BoundedI32);
        generate_bounded_num_display_test!(BoundedI64);
        generate_bounded_num_display_test!(BoundedI128);
        generate_bounded_num_display_test!(BoundedIsize);

        generate_bounded_num_display_test!(BoundedU8);
        generate_bounded_num_display_test!(BoundedU16);
        generate_bounded_num_display_test!(BoundedU32);
        generate_bounded_num_display_test!(BoundedU64);
        generate_bounded_num_display_test!(BoundedU128);
        generate_bounded_num_display_test!(BoundedUsize);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_bounded_num_serde() {
        macro_rules! generate_bounded_num_serde_test {
            ($type_name:ident) => {
                let value = $type_name::<1, 100>::new(50).unwrap();
                let serialized = serde_json::to_string(&value).unwrap();
                assert_eq!(serialized, "50");

                let deserialized: Result<$type_name<1, 100>, _> = serde_json::from_str(&serialized);
                assert!(deserialized.is_ok());
                assert_eq!(deserialized.unwrap().into_inner(), 50);

                let invalid_data = "0";
                let result: Result<$type_name<1, 100>, _> = serde_json::from_str(invalid_data);
                assert!(result.is_err());
            };
        }

        generate_bounded_num_serde_test!(BoundedI8);
        generate_bounded_num_serde_test!(BoundedI16);
        generate_bounded_num_serde_test!(BoundedI32);
        generate_bounded_num_serde_test!(BoundedI64);
        generate_bounded_num_serde_test!(BoundedI128);
        generate_bounded_num_serde_test!(BoundedIsize);

        generate_bounded_num_serde_test!(BoundedU8);
        generate_bounded_num_serde_test!(BoundedU16);
        generate_bounded_num_serde_test!(BoundedU32);
        generate_bounded_num_serde_test!(BoundedU64);
        generate_bounded_num_serde_test!(BoundedU128);
        generate_bounded_num_serde_test!(BoundedUsize);
    }

    #[test]
    fn test_bounded_float() {
        macro_rules! generate_bounded_float_test {
            ($bounded_type_name:ident, $type_name:ident) => {
                generate_bounded_float!($bounded_type_name, 0.0, 100.0, $type_name);

                let value = $bounded_type_name::new(50.0);
                assert!(value.is_ok());
                assert_eq!(value.unwrap().into_inner(), 50.0);

                let too_low = $bounded_type_name::new(-10.0);
                assert!(too_low.is_err());

                let too_high = $bounded_type_name::new(200.0);
                assert!(too_high.is_err());
            };
        }

        generate_bounded_float_test!(BoundedFloat32between0And100, f32);
        generate_bounded_float_test!(BoundedFloat64between0And100, f64);
    }

    #[test]
    fn test_bounded_float_unchecked() {
        unsafe {
            macro_rules! generate_bounded_float_unchecked_test {
                ($bounded_type_name:ident, $type_name:ident) => {
                    generate_bounded_float!($bounded_type_name, 0.0, 100.0, $type_name);

                    let value = $bounded_type_name::new_unchecked(50.0);
                    assert_eq!(value.into_inner(), 50.0);
                };
            }

            generate_bounded_float_unchecked_test!(BoundedFloat32between0And100, f32);
            generate_bounded_float_unchecked_test!(BoundedFloat64between0And100, f64);
        }
    }

    #[test]
    fn test_bounded_float_display() {
        macro_rules! generate_bounded_float_display_test {
            ($bounded_type_name:ident, $type_name:ident) => {
                generate_bounded_float!($bounded_type_name, 0.0, 100.0, $type_name);

                let value = $bounded_type_name::new(50.0).unwrap();
                assert_eq!(value.to_string(), "50");
            };
        }

        generate_bounded_float_display_test!(BoundedFloat32between0And100, f32);
        generate_bounded_float_display_test!(BoundedFloat64between0And100, f64);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_bounded_float_serde() {
        macro_rules! generate_bounded_float_serde_test {
            ($bounded_type_name:ident, $type_name:ident) => {
                generate_bounded_float!($bounded_type_name, 0.0, 100.0, $type_name);

                let value = $bounded_type_name::new(50.0).unwrap();
                let serialized = serde_json::to_string(&value).unwrap();
                assert_eq!(serialized, "50.0");

                let deserialized: Result<$bounded_type_name, _> = serde_json::from_str(&serialized);
                assert!(deserialized.is_ok());
                assert_eq!(deserialized.unwrap().into_inner(), 50.0);

                let invalid_data = "-10.0";
                let result: Result<$bounded_type_name, _> = serde_json::from_str(invalid_data);
                assert!(result.is_err());
            };
        }

        generate_bounded_float_serde_test!(BoundedFloat32between0And100, f32);
        generate_bounded_float_serde_test!(BoundedFloat64between0And100, f64);
    }
}
