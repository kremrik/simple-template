pub fn dedent(text: &str) -> String {
    text.to_string()
}


// TESTS
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        dedent,
    };

    #[test]
    fn test_no_common_indent() {
        let text = "foo\nbar";
        let expect = String::from("foo\nbar");
        let actual = dedent(text);
        assert_eq!(expect, actual);
    }
}
