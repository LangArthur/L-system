use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub axiom: String,
    pub rules: HashMap<char, String>,
    pub delta: f32,
    pub name: String,
}