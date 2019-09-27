use crate::sqm::array::Array;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class{
    pub items: HashMap<String, String>,
    pub arrays: HashMap<String, Array>,
    pub classes: HashMap<String, Class>,
}

impl Class{
    #[inline]
    pub fn walk(&self, name: &str, mut file: &std::fs::File, depth: u64){
        let t = (0..depth).map(|_| "\t").collect::<String>();
        let t2 = (0..=depth).map(|_| "\t").collect::<String>();

        file.write_all(format!("{}class {}\r\n{}{{\r\n",t, name, t).as_bytes()).unwrap();

        for item in &self.items {
            file.write_all(format!("{}{}={};\r\n", t2, item.0, item.1).as_bytes()).unwrap();
        }

        for array in &self.arrays {
            array.1.walk(array.0, file, depth);
        }

        for class in &self.classes {
            let i = depth + 1;
            class.1.walk(class.0, file, i);
        }

        file.write_all(format!("{}}};\r\n", t).as_bytes()).unwrap();
    }
}