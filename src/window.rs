pub mod window {
    pub struct Window {
        pub context: glutin::WindowedContext<glutin::NotCurrent>,
        pub events: glutin::EventsLoop,
    }

    impl Window {
        pub fn new(title: &str, width: f64, height: f64) -> Window {
            let el = glutin::EventsLoop::new();
            let wb = glutin::WindowBuilder::new()
                .with_title(title)
                .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
            let windowed_context = glutin::ContextBuilder::new()
                .build_windowed(wb, &el)
                .unwrap();

            Window { context: windowed_context, events: el }
        }
    }
}
