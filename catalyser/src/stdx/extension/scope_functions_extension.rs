//! Kotlin-inspired scope functions for concise, fluent operations on values.
//!
//! Provides traits and functions that allow operating on values within
//! dedicated closures:
//!
//! - [`Apply`]: Mutate or inspect a value and return the original value.
//! - [`Run`]: Transform a value into another value using a closure.
//! - [`TakeIf`]: Conditionally retain `Some(self)` if a predicate is met,
//!   otherwise `None`.
//! - [`TakeUnless`]: Conditionally retain `Some(self)` unless a predicate is
//!   met, otherwise `None`.
//! - [`repeat`]: Execute a closure `n` times with the current iteration index.
//!
//! # Examples
//!
//! ```rust
//! use catalyser::stdx::extension::scope_functions_extension::{
//!     Apply, Run, TakeIf, TakeUnless, repeat,
//! };
//!
//! // Apply
//! let number = 10.apply(|n| *n += 5);
//! assert_eq!(number, 15);
//!
//! // Run
//! let formatted = vec![1, 2, 3]
//!     .run(|v| v.len())
//!     .run(|len| format!("Length: {len}"));
//! assert_eq!(formatted, "Length: 3");
//!
//! // TakeIf & TakeUnless
//! assert_eq!(100.take_if(|&n| n > 50), Some(100));
//! assert_eq!(100.take_unless(|&n| n > 50), None);
//!
//! // Repeat
//! let mut total = 0;
//! repeat(4, |i| total += i);
//! assert_eq!(total, 6); // 0 + 1 + 2 + 3
//! ```

/// A trait that passes a mutable reference of `self` to a closure and returns
/// `self`.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::extension::scope_functions_extension::Apply;
///
/// let greeting = String::from("Hello")
///     .apply(|s| s.push_str(", world!"))
///     .apply(|s| s.make_ascii_uppercase());
/// assert_eq!(greeting, "HELLO, WORLD!");
/// ```
pub trait Apply: Sized {
    /// Mutates or inspects `self` using `block` and returns `self`.
    fn apply<F>(mut self, block: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        block(&mut self);
        self
    }
}

impl<T> Apply for T {}

/// A trait that passes `self` by value to a closure and returns the
/// closure's result.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::extension::scope_functions_extension::Run;
///
/// let word_count = "The quick brown fox"
///     .run(|s| s.split_whitespace())
///     .run(|words| words.count());
/// assert_eq!(word_count, 4);
/// ```
pub trait Run {
    /// Transforms `self` by passing it into `block` and returning the
    /// computed result.
    fn run<R, F>(self, block: F) -> R
    where
        F: FnOnce(Self) -> R,
        Self: Sized;
}

impl<T> Run for T {
    fn run<R, F>(self, block: F) -> R
    where
        F: FnOnce(Self) -> R,
        Self: Sized,
    {
        block(self)
    }
}

/// A trait that conditionally returns `Some(self)` if a predicate is
/// satisfied.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::extension::scope_functions_extension::TakeIf;
///
/// let age = 21;
/// assert_eq!(age.take_if(|&a| a >= 18), Some(21));
/// assert_eq!(16.take_if(|&a| a >= 18), None);
/// ```
pub trait TakeIf: Sized {
    /// Returns `Some(self)` if `predicate(&self)` is `true`, otherwise `None`.
    fn take_if<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool;
}

impl<T> TakeIf for T {
    fn take_if<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if predicate(&self) { Some(self) } else { None }
    }
}

/// A trait that conditionally returns `Some(self)` unless a predicate is
/// satisfied.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::extension::scope_functions_extension::TakeUnless;
///
/// let temperature = 25;
/// assert_eq!(temperature.take_unless(|&t| t < 0), Some(25));
/// assert_eq!((-5).take_unless(|&t| t < 0), None);
/// ```
pub trait TakeUnless: Sized {
    /// Returns `Some(self)` if `predicate(&self)` is `false`, otherwise `None`.
    fn take_unless<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool;
}

impl<T> TakeUnless for T {
    fn take_unless<F>(self, predicate: F) -> Option<Self>
    where
        F: FnOnce(&Self) -> bool,
    {
        if !predicate(&self) { Some(self) } else { None }
    }
}

/// Executes the given closure `action` for `times` iterations.
///
/// Passes the zero-based index `0..times` to each invocation of `action`.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::extension::scope_functions_extension::repeat;
///
/// let mut sequence = Vec::new();
/// repeat(3, |index| sequence.push(index * 2));
/// assert_eq!(sequence, vec![0, 2, 4]);
/// ```
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
    fn test_apply_trait() {
        let value = 0;
        let new_value = value
            .apply(|v| {
                assert_eq!(*v, 0);
                *v += 1;
            })
            .apply(|v| {
                assert_eq!(*v, 1);
            });

        assert_eq!(value, 0);
        assert_eq!(new_value, 1);
    }

    #[test]
    fn test_run_trait() {
        let value = vec![0];
        let new_value = value
            .clone()
            .run(|v| {
                assert_eq!(v, vec![0]);
                let first_item_edited = *v.first().unwrap() + 1;
                assert_eq!(first_item_edited, 1);
                first_item_edited
            })
            .run(|v| {
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
