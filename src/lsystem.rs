use std::collections::HashMap;

// the simplest class of L-systems
pub struct DOLSystem {
    alphabet: String, // symbols available
    axiom: String, // initial state
    rules: HashMap<char, String>, // rules to apply

    steps: Vec<String> // an history of the different steps
}

impl DOLSystem {
    pub fn new() -> Self {
        let axiom = "a".to_string();
        Self {
            alphabet: "ab".to_string(),
            axiom: axiom.clone(),
            rules: std::collections::HashMap::from([
                ('a', "ab".to_string()),
                ('b', "a".to_string()),
            ]),

            steps: Vec::<String>::new(),
        }
    }

    fn generate(self: &mut Self, initial_step: String, nbr_of_step: usize) {
        if nbr_of_step <= 0 {
            return;
        }
        let mut new_step = String::new();
        for c in initial_step.chars().into_iter() {
            if let Some(new_element) = self.rules.get(&c) {
                new_step.push_str(&new_element);
            } else {
                new_step.push(c);
            }
        }
        self.steps.push(new_step);
        self.generate(self.steps[self.steps.len() - 1].clone(), nbr_of_step - 1);
    }

    pub fn get_step(self: &mut Self, nbr_of_step: usize) -> String {
        let generated_nbr_of_step = self.steps.len();
        if nbr_of_step == 0 {
            return self.axiom.clone();
        }
        if generated_nbr_of_step > nbr_of_step {
            self.steps[nbr_of_step].clone()
        } else {
            let initial_step = if generated_nbr_of_step == 0 {
                self.axiom.clone()
            } else {
                self.steps[generated_nbr_of_step - 1].clone()
            };
            self.generate(initial_step, nbr_of_step - generated_nbr_of_step);
            self.steps.last().unwrap().clone()
        }
    }
}
