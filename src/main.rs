extern crate sdl2;

mod graphic;

use graphic::canvas;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut canvas = canvas::Canvas::new("test", 800, 600);

    'running: loop {
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        canvas.window.gl_swap_window();
        for event in canvas.event.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}