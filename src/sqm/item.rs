use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item{
    pub key: String,
    pub value: String,
}