mod window;


fn main() {
    let mut window = window::window::Window::new("test", 100.0, 100.0);

    window.events.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event: glutin::WindowEvent::CloseRequested, .. } => {
                glutin::ControlFlow::Break
            },
            _ => glutin::ControlFlow::Continue,
        }
    });
}