#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub comment: String,
}

pub fn line_is_var(
    line: &str,
    bgn_placeholder: &str,
    end_placeholder: &str,
) -> bool {
    if line.contains(bgn_placeholder) && line.contains(end_placeholder) {
        return true
    }
    return false
}

pub fn get_var(
    line: &str,
    bgn_placeholder: &str,
    end_placeholder: &str,
) -> Variable {
    let bgn_adj = bgn_placeholder.len() - 1;
    let end_adj = end_placeholder.len() - 1;

    let lloc = line.find(bgn_placeholder).unwrap() + bgn_adj;
    let rloc = line.find(end_placeholder).unwrap() - end_adj;

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

pub fn placeholder_indent(
    line: &str,
    bgn_placeholder: &str,
) -> usize {
    let line_trimmed = line.trim();

    if !line_trimmed.starts_with(bgn_placeholder) {
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
        let bgn_placeholder = "{{";
        let end_placeholder = "}}";
        let line = "<a href=\"{{ var }}\"</a>";
        let expect = true;
        let actual = line_is_var(line, bgn_placeholder, end_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn lines_is_var_false() {
        let bgn_placeholder = "{{";
        let end_placeholder = "}}";
        let line = "<a href=\"http://httpbin.com\"</a>";
        let expect = false;
        let actual = line_is_var(line, bgn_placeholder, end_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn get_var_no_comment() {
        let bgn_placeholder = "{{";
        let end_placeholder = "}}";
        let line = "<a href=\"{{ var }}\"</a>";
        let expect = Variable {
            name: String::from("var"),
            comment: String::from("/path/to/var")
        };
        let actual = get_var(line, bgn_placeholder, end_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn placeholder_indent_inline() {
        let bgn_placeholder = "{{";
        let line = "<a href=\"{{ var }}\"</a>";
        let expect = 0;
        let actual = placeholder_indent(line, bgn_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn placeholder_indent_newline_no_indent() {
        let bgn_placeholder = "{{";
        let line = "{{ var }}";
        let expect = 0;
        let actual = placeholder_indent(line, bgn_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn placeholder_indent_newline_with_indent() {
        let bgn_placeholder = "{{";
        let line = "  {{ var }}";
        let expect = 2;
        let actual = placeholder_indent(line, bgn_placeholder);
        assert_eq!(expect, actual);
    }
}
