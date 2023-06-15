use std::collections::HashMap;

struct System {
    alphabet: String, // symbols available
    axiom: String, // initial state
    rules: HashMap<char, String> // rules to apply
}