pub struct Window {
    pub title: String,
    pub width: f64,
    pub height: f64,
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

        Window {
            title: title.to_owned(),
            width: width,
            height: height,
            context: windowed_context,
            events: el
        }
    }
}
