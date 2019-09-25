use crate::sqm::item::Item;
use crate::sqm::array::Array;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class{
    pub key: String,
    pub items: Vec<Item>,
    pub arrays: Vec<Array>,
    pub classes: Vec<Class>,
}