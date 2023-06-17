use std::collections::HashMap;

// the simplest class of L-systems
struct DOLSystem {
    alphabet: String, // symbols available
    axiom: String, // initial state
    rules: HashMap<char, String>, // rules to apply

    steps: Vec<String> // an history of the different steps
}

impl DOLSystem {
    fn generate(self: &mut Self, initial_step: String, nbr_of_step: usize) {
        if nbr_of_step <= 0 {
            return;
        }
        let mut new_step = String::new();
        for c in initial_step.chars().into_iter() {
            if let Some(new_element) = self.rules.get(&c) {
                new_step.push_str(&new_element);
            }
        }
        self.steps.push(new_step);
        self.generate(self.steps[self.steps.len() - 1].clone(), nbr_of_step - 1);
    }

    pub fn get_step(self: &mut Self, nbr_of_step: usize) -> String {
        let generated_nbr_of_step = self.steps.len();
        if generated_nbr_of_step > nbr_of_step {
            self.steps[nbr_of_step].clone()
        } else {
            self.generate(self.steps[generated_nbr_of_step].clone(), nbr_of_step - generated_nbr_of_step);
            self.steps.last().unwrap().clone()
        }
    }
}
