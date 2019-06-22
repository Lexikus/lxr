extern crate sdl2;
extern crate gl;

use sdl2::video::GLProfile;

pub struct Canvas {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub window: sdl2::video::Window,
    pub context: sdl2::Sdl,
    pub event: sdl2::EventPump,
}

impl Canvas {
    pub fn new(title: &str, width: u32, height: u32) -> Canvas {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(title, width, height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        let mut event_pump = sdl_context.event_pump().unwrap();

        Canvas {
            title: title.to_owned(),
            width: width,
            height: height,
            window: window,
            context: sdl_context,
            event: event_pump,
        }
    }
}
