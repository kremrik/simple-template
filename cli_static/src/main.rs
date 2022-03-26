use htmlayout::io::FileIterator;

use std::fs::File;


fn main() {
    let path = "/home/kyle/projects/html-template/cli_static/template.json";
    let line = "<a href=\"{{ var }}\"</a>";
    // let data = read_file(path);
    // let template: Template = serde_json::from_str(&data).unwrap();
    // println!("{:?}", template);

    // let file = File::open(path).unwrap();
    // let fi = FileIterator::new(&file);

    // for line in fi {
    //     println!("{}", line);
    // }

    let lloc = line.find("{").unwrap() + 1;
    let rloc = line.rfind("}").unwrap() - 1;
    
    let mut var = String::new();
    for (idx, chr) in line.chars().enumerate() {
        if idx > lloc && idx < rloc {
            var.push(chr);
        }
    }

    println!("{}", var);
}
