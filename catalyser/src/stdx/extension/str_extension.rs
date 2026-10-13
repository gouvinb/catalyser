//! Utilities for multiline string trimming and margin alignment.
//!
//! Provides the [`MultilineStr`] extension trait for cleaning up multiline
//! string literals.

/// An extension trait for stripping margins and aligning multiline strings.
///
/// Inspired by Kotlin's `String.trimMargin()`.
///
/// # Examples
///
/// ```rust
/// use catalyser::stdx::extension::str_extension::MultilineStr;
///
/// let message = "
///     |Hello,
///     |world!
/// ".trim_margin();
/// assert_eq!(message, "Hello,\nworld!");
/// ```
pub trait MultilineStr {
    /// Strips leading whitespace and the specified `margin_prefix` from each
    /// line.
    ///
    /// # Parameters
    ///
    /// - `margin_prefix`: The prefix that marks the boundary of the margin to
    ///   be trimmed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use catalyser::stdx::extension::str_extension::MultilineStr;
    ///
    /// let text = "
    ///     #First line
    ///     #Second line
    /// ".trim_margin_with("#");
    /// assert_eq!(text, "First line\nSecond line");
    /// ```
    fn trim_margin_with<M: AsRef<str>>(&self, margin_prefix: M) -> String;

    /// Strips leading whitespace and the default margin prefix (`"|"`) from
    /// each line.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use catalyser::stdx::extension::str_extension::MultilineStr;
    ///
    /// let text = "
    ///     |Line 1
    ///     |Line 2
    /// ".trim_margin();
    /// assert_eq!(text, "Line 1\nLine 2");
    /// ```
    fn trim_margin(&self) -> String {
        self.trim_margin_with("|")
    }
}

impl<S: AsRef<str>> MultilineStr for S {
    fn trim_margin_with<M: AsRef<str>>(&self, margin_prefix: M) -> String {
        let prefix = margin_prefix.as_ref();
        let lines: Vec<&str> = self
            .as_ref()
            .lines()
            .map(|line| line.trim_start())
            .collect();

        if lines.is_empty() {
            return String::new();
        } else if lines.len() == 1 {
            return lines[0]
                .strip_prefix(prefix)
                .unwrap_or(lines[0])
                .to_string();
        }

        let cleaned_lines = remove_empty_lines(&lines);

        let mut processed_lines: Vec<&str> = Vec::with_capacity(cleaned_lines.len());

        cleaned_lines.iter().for_each(|line| {
            if let Some(stripped_line) = line.strip_prefix(prefix) {
                processed_lines.push(stripped_line);
            }
        });
        processed_lines.join("\n")
    }
}

fn remove_empty_lines<'a>(lines: &[&'a str]) -> Vec<&'a str> {
    let mut iter = lines.iter().peekable();
    if iter.peek().is_some_and(|&&line| line.is_empty()) {
        iter.next();
    }

    let cleaned_lines: Vec<&str> = iter
        .clone()
        .filter(|&&line| !(iter.peek().is_none() && line.is_empty()))
        .copied()
        .collect();
    cleaned_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_margin_with() {
        let input = "\
|Hello, world!
|This is a test.";
        let expected = "\
Hello, world!
This is a test.";

        assert_eq!(input.trim_margin_with("|"), expected);
    }

    #[test]
    fn test_trim_margin_with_custom_prefix() {
        let input = "\
%Hello, world!
%This is a test.";
        let expected = "\
Hello, world!
This is a test.";

        assert_eq!(input.trim_margin_with("%"), expected);
    }

    #[test]
    fn test_trim_margin_with_no_prefix_match() {
        let input = "\
#Hello, world!
|This is a test.";
        // Only the line with the prefix "|" will be stripped
        let expected = "\
This is a test.";

        assert_eq!(input.trim_margin_with("|"), expected);
    }

    #[test]
    fn test_trim_margin() {
        let input = "\
            |Indented line 1
            |Indented line 2
        ";
        let expected = "\
Indented line 1
Indented line 2";

        assert_eq!(input.trim_margin(), expected);
    }

    #[test]
    fn test_trim_margin_with_empty_lines() {
        let input = "\
\n\
|Hello
|World


";
        let expected = "\
Hello
World";

        assert_eq!(input.trim_margin(), expected);
    }

    #[test]
    fn test_trim_margin_with_only_prefix() {
        let input = "|";
        let expected = "";

        assert_eq!(input.trim_margin(), expected);
    }

    #[test]
    fn test_trim_margin_with_extra_whitespace() {
        let input = "   |   Hello   \n   |   World   ";
        let expected = "   Hello   \n   World   ";

        assert_eq!(input.trim_margin(), expected);
    }
}
