//! This module provides multiple utility traits and functions to enhance the functionality of
//! Rust's standard library types.
//!
//! These utilities are inspired by the scope functions of Kotlin, offering flexible ways to work
//! with values in dedicated scopes.
//!
//! ## Traits:
//!
//! - **Also**: A trait that allows temporary mutation or inspection of a value using a closure,
//!   returning the original value.
//! - **Let**: A trait to transform a value into another value using a closure.
//! - **TakeIf**: A trait to conditionally return an `Option<Self>` if a predicate is satisfied.
//! - **TakeUnless**: A trait to conditionally return an `Option<Self>` unless a predicate is
//!   satisfied.
//!
//! ## Functions:
//!
//! - **repeat**: Iteratively calls an action a specified number of times, passing the current
//!   iteration index as a parameter.
//!
//! ## Examples:
//!
//! ### `Also` Trait
//!
//! ```rust
//! use catalyser::stdx::scope_functions_extension::Also;
//!
//! let value = 0;
//! let result = value
//!     .also(|v| { *v += 1; });
//! assert_eq!(result, 1);
//! ```
//!
//! ### `Let` Trait
//!
//! ```rust
//! use catalyser::stdx::scope_functions_extension::Let;
//!
//! let value = vec![0];
//! let transformed = value
//!     .let_do(|v| v.first().unwrap() + 1)
//!     .let_do(|v| v.to_string());
//! assert_eq!(transformed, "1");
//! ```
//!
//! ### `TakeIf` Trait
//!
//! ```rust
//! use catalyser::stdx::scope_functions_extension::TakeIf;
//!
//! let value = 10;
//! assert_eq!(value.take_if(|&v| v > 5), Some(10));
//! assert_eq!(value.take_if(|&v| v < 5), None);
//! ```
//!
//! ### `TakeUnless` Trait
//!
//! ```rust
//! use catalyser::stdx::scope_functions_extension::TakeUnless;
//!
//! let value = 10;
//! assert_eq!(value.take_unless(|&v| v > 5), None);
//! assert_eq!(value.take_unless(|&v| v < 5), Some(10));
//! ```
//!
//! ### `repeat` Function
//!
//! ```rust
//! use catalyser::stdx::scope_functions_extension::repeat;
//!
//! let mut sum = 0;
//! repeat(5, |index| {
//!     sum += index;
//! });
//! assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4
//! ```

/// Calls the specified function `block` with `self` value as its argument and returns `self` value.
pub trait Also: Sized {
    fn also<F>(mut self, block: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        block(&mut self);
        self
    }
}

impl<T> Also for T {}

/// Calls the specified function `block` with `self` value as its argument and returns its result.
pub trait Let {
    fn let_do<R, F>(self, block: F) -> R
    where
        F: FnOnce(Self) -> R,
        Self: Sized;
}

impl<T> Let for T {
    fn let_do<R, F>(self, block: F) -> R
    where
        F: FnOnce(Self) -> R,
        Self: Sized,
    {
        block(self)
    }
}

/// Returns `self` if it satisfies the given `predicate`, or `None` if it doesn't.
pub trait TakeIf: Sized {
    fn take_if<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool;
}

impl<T> TakeIf for T {
    fn take_if<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if predicate(&self) {
            Some(self)
        } else {
            None
        }
    }
}

/// Returns `self` if it does NOT satisfy the given `predicate`, or `None` if it does.
pub trait TakeUnless: Sized {
    fn take_unless<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool;
}

impl<T> TakeUnless for T {
    fn take_unless<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if !predicate(&self) {
            Some(self)
        } else {
            None
        }
    }
}

/// Executes the given function `action` specified number of `times`.
///
/// A zero-based index of the current iteration is passed as a parameter to `action`.
pub fn repeat<F>(times: usize, mut action: F)
where
    F: FnMut(usize),
{
    for index in 0..times {
        action(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_also_trait() {
        let value = 0;
        let new_value = value
            .also(|v| {
                assert_eq!(*v, 0);
                *v += 1;
            })
            .also(|v| {
                assert_eq!(*v, 1);
            });

        assert_eq!(value, 0);
        assert_eq!(new_value, 1);
    }

    #[test]
    fn test_let_do_trait() {
        let value = vec![0];
        let new_value = value
            .clone()
            .let_do(|v| {
                assert_eq!(v, vec![0]);
                let first_item_edited = *v.first().unwrap() + 1;
                assert_eq!(first_item_edited, 1);
                first_item_edited
            })
            .let_do(|v| {
                assert_eq!(v, 1);
                let v_str = format!("{}", v);
                assert_eq!(v_str, "1");
                v_str
            });

        assert_eq!(value.clone(), vec![0]);
        assert_eq!(new_value, "1");
    }

    #[test]
    fn test_take_if_trait() {
        let value = 10;
        let result = value.take_if(|&v| v > 5);
        assert_eq!(result, Some(10));

        let result = value.take_if(|&v| v < 5);
        assert_eq!(result, None);
    }

    #[test]
    fn test_take_unless_trait() {
        let value = 10;
        let result = value.take_unless(|&v| v > 5);
        assert_eq!(result, None);

        let result = value.take_unless(|&v| v < 5);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_repeat_function() {
        let mut sum = 0;
        repeat(5, |index| {
            sum += index;
        });

        assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4
    }
}
