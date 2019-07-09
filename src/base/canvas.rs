#![allow(dead_code)]

extern crate gl;
extern crate glfw;
const OPENGL_MAJOR_VERSION: u32 = 4;
const OPENGL_MINOR_VERSION: u32 = 0;

use std::sync::mpsc::Receiver;

use glfw::Context;
use glfw::WindowEvent;
use glfw::Key;

// use crate::base::event_handler as eh;
// use crate::base::event_handler::EventType;

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
            events: receiver,
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

    pub fn on_update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&mut self.events) {
            match event {
                WindowEvent::Key(key, _scancode, _action, _modifiers) => {
                    match key {
                        Key::Escape => {
                            self.window.set_should_close(true);
                        },
                        _ => ()
                    }
                },
                _ => ()
            };
        }
    }

    pub fn terminate() {
        glfw::terminate();
    }
}
