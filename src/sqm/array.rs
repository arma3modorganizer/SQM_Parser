use serde::{Serialize, Deserialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Array{
    pub values: Vec<String>
}

impl Array{
    #[inline]
    pub fn walk(&self, name: &str, mut file: &std::fs::File, depth: u64){
        let t = (0..depth).map(|_| "\t").collect::<String>();
        let t2 = (0..=depth).map(|_| "\t").collect::<String>();
        file.write_all(format!("{}{}[]={{\r\n",t, name).as_bytes()).unwrap();

        for v in &self.values {
            file.write_all(format!("{}{},\r\n",t2, v).as_bytes()).unwrap();
        }
        file.write_all(format!("{}}};\r\n", t).as_bytes()).unwrap();
    }
}