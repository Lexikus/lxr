extern crate glfw;
extern crate gl;

const OPENGL_MAJOR_VERSION: u32 = 4;
const OPENGL_MINOR_VERSION: u32 = 0;

use glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

pub struct Canvas {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub should_close: bool,
    window: glfw::Window,
    glfw: glfw::Glfw,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Canvas {

        // Initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(OPENGL_MAJOR_VERSION, OPENGL_MINOR_VERSION));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // GLFW Window creation
        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        Canvas {
            title: title.to_owned(),
            width: width,
            height: height,
            should_close: false,
            window: window,
            glfw: glfw,
            events: events,
        }
    }

    // Loading all OpenGL functions
    pub fn load_graphic_library_functions(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }

    // Swap window buffer
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    // Poll events
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    // Process internal events. For instance, closing the window and
    // resizing the window.
    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    unsafe {
                        gl::Viewport(0, 0, width, height)
                    }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                    self.should_close = true;
                }
                _ => {}
            }
        }
    }
}
