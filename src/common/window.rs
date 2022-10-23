use winit::platform::windows::WindowBuilderExtWindows;

pub struct WindowManager {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
}

impl WindowManager {
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title("Stonehearth 2")
            .with_min_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .with_max_inner_size(winit::dpi::PhysicalSize::new(1920, 1080))
            .with_theme(Some(winit::window::Theme::Dark))
            .build(&event_loop)
            .unwrap();
        Self { event_loop, window }
    }
}
