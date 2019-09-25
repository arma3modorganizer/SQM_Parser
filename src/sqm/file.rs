use crate::sqm::item::Item;
use crate::sqm::array::Array;
use crate::sqm::class::Class;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct File{
    pub items: Vec<Item>,
    pub arrays: Vec<Array>,
    pub classes: Vec<Class>,
}