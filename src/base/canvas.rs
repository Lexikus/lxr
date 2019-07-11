#![allow(dead_code)]

extern crate gl;
extern crate glfw;

const OPENGL_MAJOR_VERSION: u32 = 4;
const OPENGL_MINOR_VERSION: u32 = 0;

use crate::util::key::Input;
use crate::util::key::Key;
use crate::util::key::Action;
use crate::util::key::Modifier;

use super::input::Input as InputController;

use std::sync::mpsc::Receiver;

use glfw::Context;
use glfw::WindowEvent;

pub struct Canvas {
    title: String,
    width: u32,
    height: u32,
    window: glfw::Window,
    glfw: glfw::Glfw,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Option<Canvas> {
        let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(glfw) => glfw,
            Err(_) => return None,
        };

        glfw.window_hint(glfw::WindowHint::ContextVersion(
            OPENGL_MAJOR_VERSION,
            OPENGL_MINOR_VERSION,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, receiver) =
            match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
                Some((window, receiver)) => (window, receiver),
                None => return None,
            };

        glfw.make_context_current(Some(&window));

        gl::load_with(|s| window.get_proc_address(s));

        window.set_key_polling(true);

        Some(Canvas {
            title: title.to_owned(),
            width: width,
            height: height,
            window: window,
            glfw: glfw,
            events: receiver
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn get_window(&self) -> &glfw::Window {
        &self.window
    }

    pub fn set_vsync(&mut self, enable: bool) {
        if enable {
            self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
        }
    }

    pub fn poll_events(&mut self, input_controller: &mut InputController) {
        self.glfw.poll_events();

        for (_, message) in glfw::flush_messages(&mut self.events) {
            match message {
                WindowEvent::Key(key, _, action, modifiers) => {

                    let action = match action {
                        glfw::Action::Press => Action::Press,
                        glfw::Action::Release => Action::Release,
                        glfw::Action::Repeat => Action::Repeat,
                    };

                    let key = match key {
                        glfw::Key::Space => Key::Space,
                        glfw::Key::Apostrophe => Key::Apostrophe,
                        glfw::Key::Comma => Key::Comma,
                        glfw::Key::Minus => Key::Minus,
                        glfw::Key::Period => Key::Period,
                        glfw::Key::Slash => Key::Slash,
                        glfw::Key::Num0 => Key::Num0,
                        glfw::Key::Num1 => Key::Num1,
                        glfw::Key::Num2 => Key::Num2,
                        glfw::Key::Num3 => Key::Num3,
                        glfw::Key::Num4 => Key::Num4,
                        glfw::Key::Num5 => Key::Num5,
                        glfw::Key::Num6 => Key::Num6,
                        glfw::Key::Num7 => Key::Num7,
                        glfw::Key::Num8 => Key::Num8,
                        glfw::Key::Num9 => Key::Num9,
                        glfw::Key::Semicolon => Key::Semicolon,
                        glfw::Key::Equal => Key::Equal,
                        glfw::Key::A => Key::A,
                        glfw::Key::B => Key::B,
                        glfw::Key::C => Key::C,
                        glfw::Key::D => Key::D,
                        glfw::Key::E => Key::E,
                        glfw::Key::F => Key::F,
                        glfw::Key::G => Key::G,
                        glfw::Key::H => Key::H,
                        glfw::Key::I => Key::I,
                        glfw::Key::J => Key::J,
                        glfw::Key::K => Key::K,
                        glfw::Key::L => Key::L,
                        glfw::Key::M => Key::M,
                        glfw::Key::N => Key::N,
                        glfw::Key::O => Key::O,
                        glfw::Key::P => Key::P,
                        glfw::Key::Q => Key::Q,
                        glfw::Key::R => Key::R,
                        glfw::Key::S => Key::S,
                        glfw::Key::T => Key::T,
                        glfw::Key::U => Key::U,
                        glfw::Key::V => Key::V,
                        glfw::Key::W => Key::W,
                        glfw::Key::X => Key::X,
                        glfw::Key::Y => Key::Y,
                        glfw::Key::Z => Key::Z,
                        glfw::Key::LeftBracket => Key::LeftBracket,
                        glfw::Key::Backslash => Key::Backslash,
                        glfw::Key::RightBracket => Key::RightBracket,
                        glfw::Key::GraveAccent => Key::GraveAccent,
                        glfw::Key::World1 => Key::World1,
                        glfw::Key::World2 => Key::World2,
                        glfw::Key::Escape => Key::Escape,
                        glfw::Key::Enter => Key::Enter,
                        glfw::Key::Tab => Key::Tab,
                        glfw::Key::Backspace => Key::Backspace,
                        glfw::Key::Insert => Key::Insert,
                        glfw::Key::Delete => Key::Delete,
                        glfw::Key::Right => Key::Right,
                        glfw::Key::Left => Key::Left,
                        glfw::Key::Down => Key::Down,
                        glfw::Key::Up => Key::Up,
                        glfw::Key::PageUp => Key::PageUp,
                        glfw::Key::PageDown => Key::PageDown,
                        glfw::Key::Home => Key::Home,
                        glfw::Key::End => Key::End,
                        glfw::Key::CapsLock => Key::CapsLock,
                        glfw::Key::ScrollLock => Key::ScrollLock,
                        glfw::Key::NumLock => Key::NumLock,
                        glfw::Key::PrintScreen => Key::PrintScreen,
                        glfw::Key::Pause => Key::Pause,
                        glfw::Key::F1 => Key::F1,
                        glfw::Key::F2 => Key::F2,
                        glfw::Key::F3 => Key::F3,
                        glfw::Key::F4 => Key::F4,
                        glfw::Key::F5 => Key::F5,
                        glfw::Key::F6 => Key::F6,
                        glfw::Key::F7 => Key::F7,
                        glfw::Key::F8 => Key::F8,
                        glfw::Key::F9 => Key::F9,
                        glfw::Key::F10 => Key::F10,
                        glfw::Key::F11 => Key::F11,
                        glfw::Key::F12 => Key::F12,
                        glfw::Key::F13 => Key::F13,
                        glfw::Key::F14 => Key::F14,
                        glfw::Key::F15 => Key::F15,
                        glfw::Key::F16 => Key::F16,
                        glfw::Key::F17 => Key::F17,
                        glfw::Key::F18 => Key::F18,
                        glfw::Key::F19 => Key::F19,
                        glfw::Key::F20 => Key::F20,
                        glfw::Key::F21 => Key::F21,
                        glfw::Key::F22 => Key::F22,
                        glfw::Key::F23 => Key::F23,
                        glfw::Key::F24 => Key::F24,
                        glfw::Key::F25 => Key::F25,
                        glfw::Key::Kp0 => Key::Kp0,
                        glfw::Key::Kp1 => Key::Kp1,
                        glfw::Key::Kp2 => Key::Kp2,
                        glfw::Key::Kp3 => Key::Kp3,
                        glfw::Key::Kp4 => Key::Kp4,
                        glfw::Key::Kp5 => Key::Kp5,
                        glfw::Key::Kp6 => Key::Kp6,
                        glfw::Key::Kp7 => Key::Kp7,
                        glfw::Key::Kp8 => Key::Kp8,
                        glfw::Key::Kp9 => Key::Kp9,
                        glfw::Key::KpDecimal => Key::KpDecimal,
                        glfw::Key::KpDivide => Key::KpDivide,
                        glfw::Key::KpMultiply => Key::KpMultiply,
                        glfw::Key::KpSubtract => Key::KpSubtract,
                        glfw::Key::KpAdd => Key::KpAdd,
                        glfw::Key::KpEnter => Key::KpEnter,
                        glfw::Key::KpEqual => Key::KpEqual,
                        glfw::Key::LeftShift => Key::LeftShift,
                        glfw::Key::LeftControl => Key::LeftControl,
                        glfw::Key::LeftAlt => Key::LeftAlt,
                        glfw::Key::LeftSuper => Key::LeftSuper,
                        glfw::Key::RightShift => Key::RightShift,
                        glfw::Key::RightControl => Key::RightControl,
                        glfw::Key::RightAlt => Key::RightAlt,
                        glfw::Key::RightSuper => Key::RightSuper,
                        glfw::Key::Menu => Key::Menu,
                        glfw::Key::Unknown => Key::Unknown,
                    };

                    let modifier = match modifiers {
                        glfw::Modifiers::Shift => Modifier::Shift,
                        glfw::Modifiers::Control => Modifier::Control,
                        glfw::Modifiers::Alt => Modifier::Alt,
                        glfw::Modifiers::Super => Modifier::Super,
                        glfw::Modifiers::NumLock => Modifier::NumLock,
                        glfw::Modifiers::CapsLock => Modifier::CapsLock,
                        _ => Modifier::Unknown,
                    };

                    input_controller.update(key, Input {
                        key: key,
                        action: action,
                        modifier: modifier,
                    });
                },
                _ => ()
            };
        }
    }

    pub fn on_update(&mut self) {
        self.window.swap_buffers();
    }

    pub fn terminate() {
        glfw::terminate();
    }
}
