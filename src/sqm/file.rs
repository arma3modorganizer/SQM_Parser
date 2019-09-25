use crate::sqm::array::Array;
use crate::sqm::class::Class;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct File{
    pub items: HashMap<String, String>,
    pub arrays: HashMap<String, Array>,
    pub classes: HashMap<String, Class>,
}