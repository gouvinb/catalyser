//! Compile-time range-bounded integer and floating-point types.
//!
//! Provides the [`generate_bounded_num`](crate::generate_bounded_num) and
//! [`generate_bounded_float`](crate::generate_bounded_float) macros along with
//! predefined bounded integer types like [`BoundedI32`] and [`BoundedU8`].
//!
//! # Examples
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use catalyser::stdx::primitive_number::BoundedI32;
//!
//! // Successfully construct a value within bounds [0, 100]
//! let percentage = BoundedI32::<0, 100>::new(75)?;
//! assert_eq!(percentage.into_inner(), 75);
//!
//! // Out-of-bounds inputs return an error
//! let invalid = BoundedI32::<0, 100>::new(105);
//! assert!(invalid.is_err());
//! # Ok(())
//! # }
//! ```

/// Defines a bounded integer struct with const generic `MIN` and `MAX` bounds.
///
/// # Examples
///
/// ```rust
/// # use crate::stdx::error::out_of_bound::OutOfBoundsError;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::generate_bounded_num;
/// use catalyser::stdx::error::out_of_bound::OutOfBoundsError;
///
/// generate_bounded_num!(MyByte, u8);
///
/// let byte = MyByte::<10, 20>::new(15)?;
/// assert_eq!(byte.into_inner(), 15);
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! generate_bounded_num {
    ($name:ident, $type_name:ident) => {
        #[doc = concat!("A [`", stringify!($name), "`] bounded between `MIN` and `MAX` (inclusive).")]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize),
            serde(transparent)
        )]
        pub struct $name<const MIN: $type_name, const MAX: $type_name>($type_name);

        impl<const MIN: $type_name, const MAX: $type_name> $name<MIN, MAX> {
            #[doc = concat!("Creates a new [`", stringify!($name), "`] from `num`.\n")]
            ///
            /// # Errors
            ///
            /// - Returns [`OutOfBoundsError::Low`] if `num < MIN`.
            /// - Returns [`OutOfBoundsError::High`] if `num > MAX`.
            #[allow(unused)]
            pub fn new(num: $type_name) -> Result<Self, $crate::stdx::error::out_of_bound::OutOfBoundsError<$type_name>> {
                if MIN > num {
                    Err($crate::stdx::error::out_of_bound::OutOfBoundsError::Low(MIN, MAX, num))
                } else if num > MAX {
                    Err($crate::stdx::error::out_of_bound::OutOfBoundsError::High(MIN, MAX, num))
                } else {
                    Ok(Self(num))
                }
            }

            #[doc = concat!("Creates a new [`", stringify!($name), "`] without bounds checking.\n")]
            ///
            /// # Safety
            ///
            /// The caller must guarantee that `num` is between `MIN` and `MAX`
            /// (inclusive).
            #[allow(unused)]
            pub unsafe fn new_unchecked(num: $type_name) -> Self {
                Self(num)
            }

            /// Returns the inner primitive value.
            #[allow(unused)]
            pub fn into_inner(self) -> $type_name {
                self.0
            }
        }

        #[cfg(feature = "serde")]
        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        impl<'de, const MIN: $type_name, const MAX: $type_name> serde::Deserialize<'de> for $name<MIN, MAX> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <$type_name as serde::Deserialize<'de>>::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }

        impl<const MIN: $type_name, const MAX: $type_name> ::core::fmt::Display for $name<MIN, MAX> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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

/// A macro to generate a bounded floating-point type with fixed minimum and
/// maximum values.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use catalyser::generate_bounded_float;
/// use catalyser::stdx::error::out_of_bound::OutOfBoundsError;
///
/// generate_bounded_float!(BoundedF32, 0.0, 100.0, f32);
///
/// let bounded = BoundedF32::new(50.0)?;
/// assert_eq!(bounded.into_inner(), 50.0);
///
/// assert!(BoundedF32::new(150.0).is_err());
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! generate_bounded_float {
    ($name:ident, $min:expr, $max:expr, $type_name:ident) => {
        #[doc = concat!("A [`", stringify!($name), "`] bounded between `MIN` and `MAX` (inclusive).")]
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        #[repr(transparent)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize),
            serde(transparent)
        )]
        pub struct $name($type_name);

        impl $name {
            /// The minimum permissible value.
            pub const MIN: $type_name = $min;
            /// The maximum permissible value.
            pub const MAX: $type_name = $max;

            #[doc = concat!("Creates a new [`", stringify!($name), "`] from `num`.\n")]
            ///
            /// # Errors
            ///
            /// - Returns [`OutOfBoundsError::Low`] if `num < Self::MIN`.
            /// - Returns [`OutOfBoundsError::High`] if `num > Self::MAX`.
            #[allow(unused)]
            pub fn new(num: $type_name) -> Result<Self, $crate::stdx::error::out_of_bound::OutOfBoundsError<$type_name>> {
                if Self::MIN > num {
                    Err($crate::stdx::error::out_of_bound::OutOfBoundsError::Low(Self::MIN, Self::MAX, num))
                } else if num > Self::MAX {
                    Err($crate::stdx::error::out_of_bound::OutOfBoundsError::High(Self::MIN, Self::MAX, num))
                } else {
                    Ok(Self(num))
                }
            }

            #[doc = concat!("Creates a new [`", stringify!($name), "`] without bounds checking.\n")]
            ///
            /// # Safety
            ///
            /// The caller must guarantee that `num` is between `Self::MIN` and
            /// `Self::MAX` (inclusive).
            #[allow(unused)]
            pub unsafe fn new_unchecked(num: $type_name) -> Self {
                Self(num)
            }

            /// Returns the inner primitive float value.
            #[allow(unused)]
            pub fn into_inner(self) -> $type_name {
                self.0
            }
        }

        #[cfg(feature = "serde")]
        #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <$type_name as serde::Deserialize<'de>>::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
