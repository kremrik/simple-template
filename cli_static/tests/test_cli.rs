use htmlayout::cli;

use std::collections::HashMap;


#[test]
fn get_cli_args_no_args() {
    let args = vec![String::from("sourcename")];
    let expect = cli::Args {
        bgn_placeholder: String::from("{{"),
        end_placeholder: String::from("}}"),
        params: HashMap::new()
    };
    let actual = cli::get_cli_args(&args);
    assert_eq!(expect, actual);
}

#[test]
fn get_cli_args_one_arg() {
    let args = vec![
        String::from("sourcename"),
        String::from("--foo"),
        String::from("1")
    ];
    let params = HashMap::from([
        (String::from("foo"), String::from("1"))
    ]);
    let expect = cli::Args {
        bgn_placeholder: String::from("{{"),
        end_placeholder: String::from("}}"),
        params: params,
    };

    let actual = cli::get_cli_args(&args);
    assert_eq!(expect, actual);
}

#[test]
fn get_cli_args_mult_args() {
    let args = vec![
        String::from("sourcename"),
        String::from("--foo"),
        String::from("1"),
        String::from("--bar"),
        String::from("2"),
    ];
    let params = HashMap::from([
        (String::from("foo"), String::from("1")),
        (String::from("bar"), String::from("2"))
    ]);
    let expect = cli::Args {
        bgn_placeholder: String::from("{{"),
        end_placeholder: String::from("}}"),
        params: params,
    };

    let actual = cli::get_cli_args(&args);
    assert_eq!(expect, actual);
}

#[test]
fn get_cli_args_custom_placeholders() {
    let args = vec![
        String::from("sourcename"),
        String::from("--bgn"),
        String::from("<<"),
        String::from("--end"),
        String::from(">>"),
        String::from("--foo"),
        String::from("1"),
        String::from("--bar"),
        String::from("2"),
    ];
    let params = HashMap::from([
        (String::from("foo"), String::from("1")),
        (String::from("bar"), String::from("2"))
    ]);
    let expect = cli::Args {
        bgn_placeholder: String::from("<<"),
        end_placeholder: String::from(">>"),
        params: params,
    };

    let actual = cli::get_cli_args(&args);
    assert_eq!(expect, actual);
}

#[test]
fn get_cli_args_one_custom_placeholder() {
    let args = vec![
        String::from("sourcename"),
        String::from("--end"),
        String::from(">>"),
        String::from("--foo"),
        String::from("1"),
        String::from("--bar"),
        String::from("2"),
    ];
    let params = HashMap::from([
        (String::from("foo"), String::from("1")),
        (String::from("bar"), String::from("2"))
    ]);
    let expect = cli::Args {
        bgn_placeholder: String::from("{{"),
        end_placeholder: String::from(">>"),
        params: params,
    };

    let actual = cli::get_cli_args(&args);
    assert_eq!(expect, actual);
}
