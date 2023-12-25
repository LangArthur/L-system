use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    #[serde(default = "Rule::default_prob")]
    pub prob: f32,
    pub production: String,
}

impl From<String> for Rule {
    fn from(value: String) -> Self {
        Self {
            prob: Rule::default_prob(),
            production: value
        }
    }
}

impl Rule {
    fn default_prob() -> f32 {
        1.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub axiom: String,
    pub rules: HashMap<char, Vec<Rule>>,
    pub delta: f32,
    #[serde(skip, default)]
    pub name: String,
}