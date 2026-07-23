/// Parses commands from an input buffer. Accepts both single and multi-line strings.
///
/// # Examples
///
/// ```
/// use aash::input::parser;
///
/// let line = "This is a test";
/// let arg_vec = parser::parse(line);
/// arg_vec.iter().for_each(|arg| {
///     println!("{}", arg);
/// });
/// ```
pub fn parse(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

#[cfg(test)]
mod parse {
    #[test]
    /// Split a generic line of words.
    fn some_words() {
        let input = "This is a test";
        let arg_vec = super::parse(input);
        assert_eq!(arg_vec, vec!["This", "is", "a", "test"]);
    }

    #[test]
    /// Empty input should result in an empty vector.
    fn empty() {
        let input = "";
        let args = super::parse(input);
        assert_eq!(args, vec![] as Vec<&str>);
    }

    #[test]
    /// Line breaks in the input should be treated like other whitespace.
    fn line_breaks() {
        let input = "line\nline\nline";
        let args = super::parse(input);
        assert_eq!(args, vec!["line", "line", "line"]);
    }
}
