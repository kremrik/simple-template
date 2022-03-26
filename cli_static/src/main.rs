use htmlayout::io::FileIterator;

use std::fs::File;


fn main() {
    let path = "/home/kyle/projects/html-template/cli_static/template.json";
    // let data = read_file(path);
    // let template: Template = serde_json::from_str(&data).unwrap();
    // println!("{:?}", template);

    let file = File::open(path).unwrap();
    let fi = FileIterator::new(&file);

    for line in fi {
        println!("{}", line);
    }
}
