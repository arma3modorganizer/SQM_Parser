use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Array{
    pub key: String,
    pub values: Vec<String>
}