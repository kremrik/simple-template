#![allow(dead_code)]

use htmlayout::{
    cli,
    constants,
    io,
    template,
};

use std::env;
use std::fs::File;
use std::process::exit;

fn main() {
    let h = constants::HELP_SHORT.to_string();
    let help = constants::HELP_LONG.to_string();
    let cli_params: Vec<String> = env::args().collect();
    let template_args = cli::get_cli_args(&cli_params);
    
    if cli_params.contains(&h) || cli_params.contains(&help) {
        run_help(&template_args);
        exit(0);
    } else {
        run(&template_args);
        exit(0);
    }
}

fn run_help(args: &cli::Args) {
    let bgn_placeholder = &args.bgn_placeholder;
    let end_placeholder = &args.end_placeholder;
    let si = io::StdinIterator::new();
    let help = cli::make_help(si, &bgn_placeholder, &end_placeholder);
    println!("{}", help);
}

fn run(args: &cli::Args) {
    let bgn_placeholder = &args.bgn_placeholder;
    let end_placeholder = &args.end_placeholder;
    let si = io::StdinIterator::new();

    for line in si {
        if template::line_is_var(&line, &bgn_placeholder, &end_placeholder) {
            let var = template::get_var(&line, &bgn_placeholder, &end_placeholder).name;
            if !args.params.contains_key(&var) {
                print!("{}", line);
                continue
            }

            let indent_size = template::placeholder_indent(&line, &bgn_placeholder);
            let indent = make_indent(indent_size);
            let path = args.params.get(&var).unwrap();
            let handler = File::open(path).unwrap();
            let fi = io::FileIterator::new(&handler);
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
