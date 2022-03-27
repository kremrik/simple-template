#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub comment: String,
}

pub fn line_is_var(line: &str) -> bool {
    if line.contains("{{") && line.contains("}}") {
        return true
    }
    return false
}

pub fn get_var(line: &str) -> Variable {
    let lloc = line.find("{").unwrap() + 1;
    let rloc = line.find("}").unwrap() - 1;

    let mut variable = String::new();
    for (idx, chr) in line.chars().enumerate() {
        if idx > lloc && idx < rloc {
            variable.push(chr)
        }
    }

    variable = variable.trim().to_string();
    let comment = format!("/path/to/{}", variable);

    Variable {
        name: variable,
        comment: comment,
    }
}

pub fn placeholder_indent(line: &str) -> usize {
    let line_trimmed = line.trim();

    if !line_trimmed.starts_with("{") {
        return 0
    }

    line.len() - line_trimmed.len()
}


// TESTS
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        Variable,
        line_is_var,
        get_var,
        placeholder_indent
    };

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
        let expect = Variable {
            name: String::from("var"),
            comment: String::from("/path/to/var")
        };
        let actual = get_var(line);
        assert_eq!(expect, actual);
    }

    #[test]
    fn placeholder_indent_inline() {
        let line = "<a href=\"{{ var }}\"</a>";
        let expect = 0;
        let actual = placeholder_indent(line);
        assert_eq!(expect, actual);
    }

    #[test]
    fn placeholder_indent_newline_no_indent() {
        let line = "{{ var }}";
        let expect = 0;
        let actual = placeholder_indent(line);
        assert_eq!(expect, actual);
    }

    #[test]
    fn placeholder_indent_newline_with_indent() {
        let line = "  {{ var }}";
        let expect = 2;
        let actual = placeholder_indent(line);
        assert_eq!(expect, actual);
    }
}
