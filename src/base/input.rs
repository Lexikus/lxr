#![allow(dead_code)]

use std::collections::HashMap;

use crate::util::key::Input as KeyInput;
use crate::util::key::Action;
use crate::util::key::Key;

pub struct Input<'a> {
    input: &'a HashMap<i32, KeyInput>,
}


impl<'a> Input<'a> {
    pub fn init(inputs: &'a HashMap<i32, KeyInput>) -> Input {
        Input {
            input: inputs
        }
    }

    pub fn is_key_pressed(&self, key: i32) -> bool {
        // self.input[key].action == Action::Press || self.input[key].action == Action::Release
        false
    }
}

