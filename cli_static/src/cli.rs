use crate::parsers::template;

use std::collections::HashMap;


pub fn make_help(
    lines: impl Iterator<Item=String>
) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for line in lines {
        if template::line_is_var(&line) {
            let var = template::get_var(&line);
            params.insert(var.name, var.comment);
        }
    }

    params
}


// TESTS
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        make_help,
    };
    use std::collections::HashMap;

    struct MockLines<'l> {
        lines: std::str::Lines<'l>
    }

    impl<'l> MockLines<'l> {
        fn new(data: &str) -> MockLines {
            MockLines {
                lines: data.lines()
            }
        }
    }

    impl<'l> Iterator for MockLines<'l> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(line) = self.lines.next() {
                return Some(line.to_string());
            }
            None
        }
    }


    #[test]
    fn make_help_no_placeholders() {
        let templ = "<html></html>";
        let lines = MockLines::new(templ);
        let expect: HashMap<String, String> = HashMap::new();
        let actual = make_help(lines);
        assert_eq!(expect, actual);
    }

    #[test]
    fn make_help_one_placeholder() {
        let templ = r#"
        <html>
            {{ foo }}
        </html>
        "#;
        let lines = MockLines::new(templ);
        let mut expect = HashMap::new();
        expect.insert(String::from("foo"), String::from("/path/to/foo"));

        let actual = make_help(lines);
        assert_eq!(expect, actual);
    }

    #[test]
    fn make_help_mult_placeholders() {
        let templ = r#"
        <html>
            {{ foo }}
            <a href="{{ bar }}">click me</a>
        </html>
        "#;
        let lines = MockLines::new(templ);
        let mut expect = HashMap::new();
        expect.insert(String::from("foo"), String::from("/path/to/foo"));
        expect.insert(String::from("bar"), String::from("/path/to/bar"));

        let actual = make_help(lines);
        assert_eq!(expect, actual);
    }
}
