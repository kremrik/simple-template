#![allow(dead_code)]

use htmlayout::cli::{
    make_help,
    get_cli_args,
};
use htmlayout::io::{
    FileIterator,
    StdinIterator,
};
use htmlayout::parsers::template;

use std::{env};
use std::fs::File;
use std::process::exit;

fn main() {
    let bgn_placeholder = "{{";
    let end_placeholder = "}}";

    let h = String::from("-h");
    let help = String::from("--help");
    let cli_params: Vec<String> = env::args().collect();
    
    if cli_params.contains(&h) || cli_params.contains(&help) {
        let si = StdinIterator::new();
        let help = make_help(si, bgn_placeholder, end_placeholder);
        println!("{}", help);
        exit(0);
    }

    let si = StdinIterator::new();
    let template_args = get_cli_args(&cli_params);

    for line in si {
        if template::line_is_var(&line, bgn_placeholder, end_placeholder) {
            let var = template::get_var(&line, bgn_placeholder, end_placeholder).name;
            if !template_args.contains_key(&var) {
                print!("{}", line);
                continue
            }

            let indent_size = template::placeholder_indent(&line, bgn_placeholder);
            let indent = make_indent(indent_size);
            let path = template_args.get(&var).unwrap();
            let handler = File::open(path).unwrap();
            let fi = FileIterator::new(&handler);
            for var_line in fi {
                println!("{}{}", indent, var_line);
            }
        } else {
            print!("{}", line);
        }
    }
}

fn make_indent(size: usize) -> String {
    let mut indent = String::new();
    for _ in 0..size {
        indent.push(' ');
    }
    indent
}
