use serde::{Serialize, Deserialize};

use std::collections::{HashMap};


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Template {
    template: String,
    output: String,
    dependencies: HashMap<String, String>,
}
