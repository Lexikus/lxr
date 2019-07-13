#![allow(dead_code)]

use std::collections::HashMap;

use crate::util::key::Input as KeyInput;
use crate::util::key::Action;
use crate::util::key::Key;
use crate::util::key::Modifier;

pub struct Input {
    inputs: HashMap<Key, KeyInput>,
}

impl Input {
    pub fn new() -> Input {
        let mut collection: HashMap<Key, KeyInput> = HashMap::with_capacity(121);

        collection.insert(Key::Space, KeyInput { key: Key::Space, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Apostrophe, KeyInput { key: Key::Apostrophe, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Comma, KeyInput { key: Key::Comma, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Minus, KeyInput { key: Key::Minus, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Period, KeyInput { key: Key::Period, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Slash, KeyInput { key: Key::Slash, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num0, KeyInput { key: Key::Num0, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num1, KeyInput { key: Key::Num1, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num2, KeyInput { key: Key::Num2, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num3, KeyInput { key: Key::Num3, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num4, KeyInput { key: Key::Num4, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num5, KeyInput { key: Key::Num5, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num6, KeyInput { key: Key::Num6, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num7, KeyInput { key: Key::Num7, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num8, KeyInput { key: Key::Num8, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Num9, KeyInput { key: Key::Num9, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Semicolon, KeyInput { key: Key::Semicolon, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Equal, KeyInput { key: Key::Equal, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::A, KeyInput { key: Key::A, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::B, KeyInput { key: Key::B, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::C, KeyInput { key: Key::C, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::D, KeyInput { key: Key::D, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::E, KeyInput { key: Key::E, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F, KeyInput { key: Key::F, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::G, KeyInput { key: Key::G, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::H, KeyInput { key: Key::H, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::I, KeyInput { key: Key::I, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::J, KeyInput { key: Key::J, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::K, KeyInput { key: Key::K, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::L, KeyInput { key: Key::L, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::M, KeyInput { key: Key::M, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::N, KeyInput { key: Key::N, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::O, KeyInput { key: Key::O, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::P, KeyInput { key: Key::P, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Q, KeyInput { key: Key::Q, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::R, KeyInput { key: Key::R, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::S, KeyInput { key: Key::S, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::T, KeyInput { key: Key::T, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::U, KeyInput { key: Key::U, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::V, KeyInput { key: Key::V, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::W, KeyInput { key: Key::W, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::X, KeyInput { key: Key::X, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Y, KeyInput { key: Key::Y, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Z, KeyInput { key: Key::Z, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::LeftBracket, KeyInput { key: Key::LeftBracket, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Backslash, KeyInput { key: Key::Backslash, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::RightBracket, KeyInput { key: Key::RightBracket, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::GraveAccent, KeyInput { key: Key::GraveAccent, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::World1, KeyInput { key: Key::World1, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::World2, KeyInput { key: Key::World2, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Escape, KeyInput { key: Key::Escape, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Enter, KeyInput { key: Key::Enter, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Tab, KeyInput { key: Key::Tab, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Backspace, KeyInput { key: Key::Backspace, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Insert, KeyInput { key: Key::Insert, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Delete, KeyInput { key: Key::Delete, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Right, KeyInput { key: Key::Right, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Left, KeyInput { key: Key::Left, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Down, KeyInput { key: Key::Down, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Up, KeyInput { key: Key::Up, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::PageUp, KeyInput { key: Key::PageUp, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::PageDown, KeyInput { key: Key::PageDown, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Home, KeyInput { key: Key::Home, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::End, KeyInput { key: Key::End, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::CapsLock, KeyInput { key: Key::CapsLock, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::ScrollLock, KeyInput { key: Key::ScrollLock, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::NumLock, KeyInput { key: Key::NumLock, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::PrintScreen, KeyInput { key: Key::PrintScreen, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Pause, KeyInput { key: Key::Pause, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F1, KeyInput { key: Key::F1, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F2, KeyInput { key: Key::F2, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F3, KeyInput { key: Key::F3, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F4, KeyInput { key: Key::F4, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F5, KeyInput { key: Key::F5, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F6, KeyInput { key: Key::F6, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F7, KeyInput { key: Key::F7, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F8, KeyInput { key: Key::F8, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F9, KeyInput { key: Key::F9, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F10, KeyInput { key: Key::F10, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F11, KeyInput { key: Key::F11, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F12, KeyInput { key: Key::F12, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F13, KeyInput { key: Key::F13, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F14, KeyInput { key: Key::F14, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F15, KeyInput { key: Key::F15, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F16, KeyInput { key: Key::F16, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F17, KeyInput { key: Key::F17, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F18, KeyInput { key: Key::F18, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F19, KeyInput { key: Key::F19, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F20, KeyInput { key: Key::F20, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F21, KeyInput { key: Key::F21, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F22, KeyInput { key: Key::F22, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F23, KeyInput { key: Key::F23, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F24, KeyInput { key: Key::F24, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::F25, KeyInput { key: Key::F25, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp0, KeyInput { key: Key::Kp0, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp1, KeyInput { key: Key::Kp1, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp2, KeyInput { key: Key::Kp2, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp3, KeyInput { key: Key::Kp3, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp4, KeyInput { key: Key::Kp4, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp5, KeyInput { key: Key::Kp5, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp6, KeyInput { key: Key::Kp6, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp7, KeyInput { key: Key::Kp7, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp8, KeyInput { key: Key::Kp8, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Kp9, KeyInput { key: Key::Kp9, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpDecimal, KeyInput { key: Key::KpDecimal, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpDivide, KeyInput { key: Key::KpDivide, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpMultiply, KeyInput { key: Key::KpMultiply, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpSubtract, KeyInput { key: Key::KpSubtract, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpAdd, KeyInput { key: Key::KpAdd, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpEnter, KeyInput { key: Key::KpEnter, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::KpEqual, KeyInput { key: Key::KpEqual, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::LeftShift, KeyInput { key: Key::LeftShift, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::LeftControl, KeyInput { key: Key::LeftControl, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::LeftAlt, KeyInput { key: Key::LeftAlt, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::LeftSuper, KeyInput { key: Key::LeftSuper, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::RightShift, KeyInput { key: Key::RightShift, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::RightControl, KeyInput { key: Key::RightControl, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::RightAlt, KeyInput { key: Key::RightAlt, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::RightSuper, KeyInput { key: Key::RightSuper, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Menu, KeyInput { key: Key::Menu, action: Action::Release, modifier: Modifier::Unknown });
        collection.insert(Key::Unknown, KeyInput { key: Key::Unknown, action: Action::Release, modifier: Modifier::Unknown });

        Input {
            inputs: collection,
        }
    }

    pub fn update(&mut self, key: &Key, input: KeyInput) {
        let entry = self.inputs.get_mut(key).unwrap();

        if entry.action == Action::Repeat && input.action == Action::Press {
            return;
        }

        *self.inputs.get_mut(key).unwrap() = input;
    }

    pub fn is_key_pressed(&mut self, key: &Key) -> bool {
        let mut entry = self.inputs.get_mut(key).unwrap();

        if entry.action == Action::Press {
            entry.action = Action::Repeat;
            return true;
        }

        return self.inputs[key].action == Action::Press;
    }

    pub fn is_key_pressed_down(&mut self, key: &Key) -> bool {
        return self.inputs[key].action == Action::Press || self.inputs[key].action == Action::Repeat;
    }
}

