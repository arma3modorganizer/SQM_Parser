use serde::{Serialize, Deserialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Array{
    pub values: Vec<String>
}

impl Array{
    pub fn walk(&self, name: &String, mut file: &std::fs::File, depth: u64){
        let t = (0..depth).map(|_| "\t").collect::<String>();
        let t2 = (0..depth+1).map(|_| "\t").collect::<String>();
        file.write(format!("{}{}[]={{\r\n",t, name).as_bytes());

        for v in &self.values {
            file.write(format!("{}{},\r\n",t2, v).as_bytes());
        }
        file.write(format!("{}}};\r\n", t).as_bytes());
    }
}