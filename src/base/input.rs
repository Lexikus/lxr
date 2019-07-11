#![allow(dead_code)]

use std::collections::HashMap;

use crate::util::key::Input as KeyInput;
use crate::util::key::Action;
use crate::util::key::Key;

pub struct Input {
    inputs: HashMap<Key, KeyInput>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            inputs: HashMap::with_capacity(121),
        }
    }

    pub fn update(&mut self, key: Key, input: KeyInput) {
        let entry = self.inputs.entry(key).or_insert(input);
        entry.key = input.key;
        entry.action = input.action;
        entry.modifier = input.modifier;
    }

    pub fn is_key_pressed(&mut self, key: &Key) -> bool {
        if !self.inputs.contains_key(key) {
            return false;
        }
        self.inputs.get_mut(&key).unwrap().action = Action::Repeat;

        return self.inputs[key].action == Action::Press || self.inputs[key].action == Action::Repeat;
    }
}

