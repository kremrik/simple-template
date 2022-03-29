#![allow(dead_code)]

use htmlayout::cli::make_help;
use htmlayout::io::{
    FileIterator,
    StdinIterator,
};

use std::fs::File;
use std::io::stdin;


fn main() {
    // let path = "/home/kyle/projects/html-template/templates/base.html";

    // let file = File::open(path).unwrap();
    // let fi = FileIterator::new(&file);

    // let help = make_help(fi);
    // println!("{}", help);


    let si = StdinIterator::new();
    for line in si {
        print!("> {}", line);
    }
}
