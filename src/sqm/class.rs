use crate::sqm::array::Array;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class{
    pub items: HashMap<String, String>,
    pub arrays: HashMap<String, Array>,
    pub classes: HashMap<String, Class>,
}