#![allow(dead_code)]

extern crate gl;
extern crate glfw;
const OPENGL_MAJOR_VERSION: u32 = 4;
const OPENGL_MINOR_VERSION: u32 = 0;

use glfw::Context;

pub struct Canvas {
    pub title: String,
    pub width: u32,
    pub height: u32,
    window: glfw::Window,
    glfw: glfw::Glfw,
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

        let (mut window, _receiver) =
            match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
                Some((window, receiver)) => (window, receiver),
                None => return None,
            };

        glfw.make_context_current(Some(&window));

        gl::load_with(|s| window.get_proc_address(s));

        Some(Canvas {
            title: title.to_owned(),
            width: width,
            height: height,
            window: window,
            glfw: glfw,
        })
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
    }

    pub fn terminate() {
        glfw::terminate();
    }
}
