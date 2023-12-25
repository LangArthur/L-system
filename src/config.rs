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
    // #[serde(deserialize_with = "Model::deserialize_rules")]
    pub rules: HashMap<char, Vec<Rule>>,
    pub delta: f32,
    #[serde(skip, default)]
    pub name: String,
}

impl Model {
    // FIXME: enable single string deserialization (see https://github.com/serde-rs/serde/issues/1470)
    // fn deserialize_rules<'de, D>(deserializer: D) -> Result<HashMap<char, Vec<Rule>>, D::Error>
    // where
    //     D: Deserializer<'de>
    // {
    //     let mut result = HashMap::<char, Vec<Rule>>::new();
    //     let sorted_rules: HashMap<char, Vec<String>> = Deserialize::deserialize(deserializer)?;
    //     for (key, str_rules) in sorted_rules {
    //         let rules = Vec::<Rule>::new();
    //         for str_rule in str_rules {
    //             println!("{}", str_rule);
    //         }
    //         result.insert(key, rules);
    //     }
    //     Err(serde::de::Error::custom("dummy"))
    // }
}