/// Parses commands from an input buffer. Accepts both single and multi-line strings.
pub fn parse(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

#[cfg(test)]
mod parse {
    use super::parse;

    fn test_parse(input: &str, expect: Vec<&str>) {
        let arg_vec = parse(input);
        assert_eq!(arg_vec, expect);
    }

    #[test]
    fn empty() {
        test_parse("", vec![]);
    }

    #[test]
    fn some_words() {
        test_parse("This is a test", vec!["This", "is", "a", "test"]);
    }

    #[test]
    fn line_breaks() {
        test_parse("line\nline\nline", vec!["line", "line", "line"]);
    }
}
