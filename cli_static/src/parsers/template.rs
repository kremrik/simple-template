pub fn line_is_var(line: &str) -> bool {
    if line.contains("{{") && line.contains("}}") {
        return true
    }
    return false
}

// pub fn get_var(line: &str) -> String {
//     let mut variable = String::new();
//     let mut state = StatePlaceholder::Before;

//     line.

//     variable
// }


// TESTS
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{line_is_var, get_var};

    #[test]
    fn lines_is_var_true() {
        let line = "<a href=\"{{ var }}\"</a>";
        let expect = true;
        let actual = line_is_var(line);
        assert_eq!(expect, actual);
    }

    #[test]
    fn lines_is_var_false() {
        let line = "<a href=\"http://httpbin.com\"</a>";
        let expect = false;
        let actual = line_is_var(line);
        assert_eq!(expect, actual);
    }

    #[test]
    fn get_var_no_comment() {
        let line = "<a href=\"{{ var }}\"</a>";
        let expect = String::from("var");
        let actual = get_var(line);
        assert_eq!(expect, actual);
    }
}
