use crate::sqm::array::Array;
use crate::sqm::class::Class;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct File{
    pub items: HashMap<String, String>,
    pub arrays: HashMap<String, Array>,
    pub classes: HashMap<String, Class>,
}

impl File {
    pub fn walk(&self, mut file: &std::fs::File){
        let depth: u64 = 0;

        println!("{:?}", self);

        for item in &self.items {
            file.write_all(format!("{}={};\r\n", item.0, item.1).as_bytes()).unwrap();
        }

        for array in &self.arrays {
            array.1.walk(array.0, file, depth);
        }

        for class in &self.classes {
            class.1.walk(class.0, file, depth);
        }
    }
}

