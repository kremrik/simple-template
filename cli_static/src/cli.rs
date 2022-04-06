use crate::template;

use std::collections::HashMap;

type HelpData = HashMap<String, String>;
type CliArgs = HashMap<String, String>;


#[derive(Debug, PartialEq)]
pub struct Args {
    pub bgn_placeholder: String,
    pub end_placeholder: String,
    pub params: CliArgs,
}


pub fn get_cli_args(args: &Vec<String>) -> Args {
    let mut cli_args = CliArgs::new();

    // TODO: this can not longer expect even number of params
    for i in (1..args.len()).step_by(2) {
        cli_args.insert(
            args[i].to_string().replace("--", ""),
            args[i+1].to_string()
        );
    }

    let mut bgn_placeholder = String::from("{{");
    let mut end_placeholder = String::from("}}");
    let bgn = String::from("bgn");
    let end = String::from("end");

    if cli_args.contains_key(&bgn) {
        bgn_placeholder = cli_args.get(&bgn).unwrap().to_string();
        cli_args.remove(&bgn).unwrap();
    }

    if cli_args.contains_key(&end) {
        end_placeholder = cli_args.get(&end).unwrap().to_string();
        cli_args.remove(&end).unwrap();
    }

    Args {
        bgn_placeholder: bgn_placeholder,
        end_placeholder: end_placeholder,
        params: cli_args
    }
}


pub fn make_help(
    lines: impl Iterator<Item=String>,
    bgn_placeholder: &str,
    end_placeholder: &str,
) -> String {
    let params = get_params(lines, bgn_placeholder, end_placeholder);
    render_help("htmlayout", &params)
}

fn get_params(
    lines: impl Iterator<Item=String>,
    bgn_placeholder: &str,
    end_placeholder: &str,
) -> HelpData {
    let mut params = HashMap::new();

    for line in lines {
        if template::line_is_var(&line, bgn_placeholder, end_placeholder) {
            let var = template::get_var(&line, bgn_placeholder, end_placeholder);
            params.insert(var.name, var.comment);
        }
    }

    params
}

fn render_help(
    name: &str,
    parameters: &HelpData,
) -> String {
    let hname = bold("NAME");
    let hsynopsis = bold("SYNOPSIS");
    let hdescription = bold("DESCRIPTION");

    let synopsis = format!("cat <template> | {} [OPTION]...", name);
    let description = fmt_params(parameters);

    format!("{}\n\t{}\n\n{}\n\t{}\n\n{}\n{}\n", 
        hname,
        name,
        hsynopsis,
        synopsis,
        hdescription,
        description,
    )
}

fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

fn fmt_params(params: &HelpData) -> String {
    let mut args: Vec<String> = Vec::new();

    for (name, comment) in params.iter() {
        let h = fmt_param(name, comment);
        args.push(h);
    }

    args.join("\n")
}

fn fmt_param(name: &str, comment: &str) -> String {
    format!("\t--{}\n\t\t{}", bold(name), comment)
}


// TESTS
// --------------------------------------------------------

// these are here until i feel like testing the actual help output...
#[cfg(test)]
mod tests {
    use super::{
        get_params,
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
        let bgn_placeholder = "{{";
        let end_placeholder = "}}";
        let templ = "<html></html>";
        let lines = MockLines::new(templ);
        let expect: HashMap<String, String> = HashMap::new();
        let actual = get_params(lines, bgn_placeholder, end_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn make_help_one_placeholder() {
        let bgn_placeholder = "{{";
        let end_placeholder = "}}";
        let templ = r#"
        <html>
            {{ foo }}
        </html>
        "#;
        let lines = MockLines::new(templ);
        let mut expect = HashMap::new();
        expect.insert(String::from("foo"), String::from("/path/to/foo"));

        let actual = get_params(lines, bgn_placeholder, end_placeholder);
        assert_eq!(expect, actual);
    }

    #[test]
    fn make_help_mult_placeholders() {
        let bgn_placeholder = "{{";
        let end_placeholder = "}}";
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

        let actual = get_params(lines, bgn_placeholder, end_placeholder);
        assert_eq!(expect, actual);
    }
}
