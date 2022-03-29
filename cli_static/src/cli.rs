use crate::parsers::template;

use std::collections::HashMap;

type HelpData = HashMap<String, String>;
type CliArgs = HashMap<String, String>;


pub fn get_cli_args(args: &Vec<&str>) -> CliArgs {
    let mut cli_args = CliArgs::new();

    for i in (0..args.len()).step_by(2) {
        cli_args.insert(
            args[i].to_string().replace("--", ""),
            args[i+1].to_string()
        );
    }

    cli_args
}


pub fn make_help(lines: impl Iterator<Item=String>) -> String {
    let params = get_params(lines);
    render_help("htmlayout", &params)
}

fn get_params(lines: impl Iterator<Item=String>) -> HelpData {
    let mut params = HashMap::new();

    for line in lines {
        if template::line_is_var(&line) {
            let var = template::get_var(&line);
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

#[cfg(test)]
mod tests {
    use super::{
        get_cli_args,
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
        let templ = "<html></html>";
        let lines = MockLines::new(templ);
        let expect: HashMap<String, String> = HashMap::new();
        let actual = get_params(lines);
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

        let actual = get_params(lines);
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

        let actual = get_params(lines);
        assert_eq!(expect, actual);
    }

    #[test]
    fn get_cli_args_no_args() {
        let args = vec![];
        let expect = HashMap::new();
        let actual = get_cli_args(&args);
        assert_eq!(expect, actual);
    }

    #[test]
    fn get_cli_args_one_arg() {
        let args = vec!["--foo", "1"];
        let expect = HashMap::from([
            (String::from("foo"), String::from("1"))
        ]);
        let actual = get_cli_args(&args);
        assert_eq!(expect, actual);
    }

    #[test]
    fn get_cli_args_mult_args() {
        let args = vec!["--foo", "1", "--bar", "2"];
        let expect = HashMap::from([
            (String::from("foo"), String::from("1")),
            (String::from("bar"), String::from("2"))
        ]);
        let actual = get_cli_args(&args);
        assert_eq!(expect, actual);
    }
}
