use std::collections::HashMap;

use serde::{Serialize, Deserialize, Deserializer};

use crate::utility::count_chars;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    #[serde(default = "Rule::default_prob")]
    pub prob: f32,
    #[serde(deserialize_with = "Rule::deserialize_production")]
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

    fn deserialize_production<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rule_str: String = Deserialize::deserialize(deserializer)?;
        if count_chars(&rule_str, '[') != count_chars(&rule_str, ']') {
            return Err(serde::de::Error::custom("Mismatched square brackets"))
        }
        return Ok(rule_str)
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