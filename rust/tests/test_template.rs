use simpletemplate::template;


#[test]
fn line_is_var_true() {
    let bgn_placeholder = "{{";
    let end_placeholder = "}}";
    let line = "<a href=\"{{ var }}\"</a>";
    let expect = true;
    let actual = template::line_is_var(line, bgn_placeholder, end_placeholder);
    assert_eq!(expect, actual);
}

#[test]
fn line_is_var_false() {
    let bgn_placeholder = "{{";
    let end_placeholder = "}}";
    let line = "<a href=\"http://httpbin.com\"</a>";
    let expect = false;
    let actual = template::line_is_var(line, bgn_placeholder, end_placeholder);
    assert_eq!(expect, actual);
}

#[test]
fn get_var_no_comment() {
    let bgn_placeholder = "{{";
    let end_placeholder = "}}";
    let line = "<a href=\"{{ var }}\"</a>";
    let expect = template::Variable {
        name: String::from("var"),
        comment: String::from("/path/to/var")
    };
    let actual = template::get_var(line, bgn_placeholder, end_placeholder);
    assert_eq!(expect, actual);
}

#[test]
fn placeholder_indent_inline() {
    let bgn_placeholder = "{{";
    let line = "<a href=\"{{ var }}\"</a>";
    let expect = 0;
    let actual = template::placeholder_indent(line, bgn_placeholder);
    assert_eq!(expect, actual);
}

#[test]
fn placeholder_indent_newline_no_indent() {
    let bgn_placeholder = "{{";
    let line = "{{ var }}";
    let expect = 0;
    let actual = template::placeholder_indent(line, bgn_placeholder);
    assert_eq!(expect, actual);
}

#[test]
fn placeholder_indent_newline_with_indent() {
    let bgn_placeholder = "{{";
    let line = "  {{ var }}";
    let expect = 2;
    let actual = template::placeholder_indent(line, bgn_placeholder);
    assert_eq!(expect, actual);
}
