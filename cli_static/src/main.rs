use htmlayout::schemas::Template;
use serde_json;
use std::fs;


fn read_file(path: &str) -> String {
    let data = fs::read_to_string(path).expect(
        "Something stupid happened"
    );
    data
}

fn main() {
    let path = "/home/kyle/projects/html-template/cli_static/template.json";
    let data = read_file(path);
    let template: Template = serde_json::from_str(&data).unwrap();
    println!("{:?}", template);
}
