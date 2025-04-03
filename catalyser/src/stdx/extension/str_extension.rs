/// An interface for removing the margin of multi-line string. (Kotlin string
/// inspiration)
pub trait MultilineStr {
    /// Removes the margin from each line of the string, where the margin is
    /// determined by the given `margin_prefix`.
    ///
    /// # Parameters
    ///
    /// * `margin_prefix` - The prefix that marks the start of the margin to be
    ///   trimmed.
    ///
    /// # Returns
    ///
    /// A new `String` with the margin removed from each line.
    fn trim_margin_with<M: AsRef<str>>(&self, margin_prefix: M) -> String;

    /// Removes the margin from each line of the string, using "|" as the
    /// default margin prefix.
    ///
    /// # Returns
    ///
    /// A new `String` with the margin removed from each line.
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
