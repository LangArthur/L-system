use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Model {
    pub axiom: String,
    pub rules: HashMap<char, String>,
    pub delta: f32,
    #[serde(skip, default)]
    pub name: String,
}