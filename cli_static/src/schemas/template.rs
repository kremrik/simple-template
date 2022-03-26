use serde::{Serialize, Deserialize};


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Template {
    template: String,
    dependencies: Option<Vec<Dependency>>,
    output: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Dependency {
    variable: String,
    template: String,
}
