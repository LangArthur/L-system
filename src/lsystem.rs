use std::collections::HashMap;
use rand::Rng;

use crate::config::Rule;

pub struct LSystem {
    axiom: String, // initial state
    rules: HashMap<char, Vec<Rule>>, // rules to apply

    steps: Vec<String>, // an history of the different steps
}

impl LSystem {
    pub fn new(axiom: String, rules: HashMap<char, Vec<Rule>>) -> Self {
        Self {
            axiom,
            rules,
            steps: Vec::<String>::new(),
        }
    }

    fn select_rule(rules: &Vec<Rule>) -> &String {
        let rules_len = rules.len();
        if rules_len == 1 {
            return &rules.first().unwrap().production
        } else {
            let mut rng = rand::thread_rng();
            let random_nb: f32 = rng.gen_range(0.0..1.0);
            let mut i = 0.0f32;
            for (idx, rule) in rules.into_iter().enumerate() {
                if random_nb <= i || idx + 1 == rules_len {
                    return &rule.production;
                }
                i += rule.prob;
            }
            panic!("No rules found (if you see this, it is a bug)");
        }
    }

    fn generate(&mut self, initial_step: String, nbr_of_step: usize) {
        if nbr_of_step <= 0 {
            return;
        }
        let mut new_step = String::new();
        for c in initial_step.chars().into_iter() {
            if let Some(rules) = self.rules.get(&c) {
                new_step.push_str(LSystem::select_rule(rules))
            } else {
                new_step.push(c);
            }
        }
        self.steps.push(new_step);
        self.generate(self.steps[self.steps.len() - 1].clone(), nbr_of_step - 1);
    }

    pub fn get_step(&mut self, nbr_of_step: usize) -> String {
        let generated_nbr_of_step = self.steps.len();
        if nbr_of_step == 0 {
            return self.axiom.clone();
        }
        if generated_nbr_of_step > nbr_of_step {
            self.steps[nbr_of_step - 1].clone()
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

    pub fn reset(&mut self) {
        self.steps.clear();
    }
}