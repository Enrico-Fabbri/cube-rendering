pub(crate) struct StoneHearthWindow {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl StoneHearthWindow {
    pub(crate) fn new() -> Self {
        use winit::platform::windows::WindowBuilderExtWindows;

        let event_loop = winit::event_loop::EventLoop::new();
        let size = winit::dpi::PhysicalSize::new(800, 600);
        let window = winit::window::WindowBuilder::new()
            .with_min_inner_size(size)
            .with_inner_size(size)
            .with_title("StoneHearth 2")
            .with_theme(Some(winit::window::Theme::Dark))
            //.with_window_icon(window_icon)
            //.with_taskbar_icon(taskbar_icon)
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop,
            window,
            size,
        }
    }

    pub(crate) fn run(mut self, mut state: crate::state::StoneHearthState) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                winit::event::Event::WindowEvent {
                    window_id,
                    ref event,
                } if window_id == self.window.id() => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    winit::event::WindowEvent::Resized(new_size) => {
                        self.size = *new_size;
                        state.resize(new_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(*new_inner_size)
                    }
                    _ => {}
                },
                winit::event::Event::RedrawRequested(window_id)
                    if window_id == self.window.id() =>
                {
                    state.update();

                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(&self.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = winit::event_loop::ControlFlow::Exit
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            })
    }
}
