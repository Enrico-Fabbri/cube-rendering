pub struct State {
    pub window_manager: super::window::WindowManager,
    pub wgpu_manager: super::wgpu::WgpuManager,
    pub camera_manager: super::camera::CameraManager,
    pub bundle_manager: super::bundles::BundleManager,
}

impl State {
    pub async fn new() -> Self {
        let window_manager = super::window::WindowManager::new();

        let wgpu_manager = super::wgpu::WgpuManager::new(&window_manager.window).await;

        let camera_manager =
            super::camera::CameraManager::new(&wgpu_manager.device, &wgpu_manager.config);

        let mut bundle_manager =
            super::bundles::BundleManager::new(&wgpu_manager.device, &wgpu_manager.config);

        crate::world::cubes::Cubes::new(
            &wgpu_manager.device,
            &wgpu_manager.config,
            &camera_manager.camera_bind_group_layout,
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        )
        .finish_bundle(
            &mut bundle_manager,
            &wgpu_manager.device,
            &wgpu_manager.config,
            &camera_manager.camera_bind_group,
            Some(wgpu::RenderBundleDepthStencil {
                format: wgpu::TextureFormat::Depth32Float,
                depth_read_only: false,
                stencil_read_only: false,
            }),
        );

        Self {
            window_manager,
            wgpu_manager,
            bundle_manager,
            camera_manager,
        }
    }

    pub fn run(mut self) {
        self.window_manager
            .event_loop
            .run(move |event, _, control_flow| match event {
                winit::event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window_manager.window.id() => {
                    if !self.wgpu_manager.input(event, &mut self.camera_manager) {
                        match event {
                            winit::event::WindowEvent::CloseRequested
                            | winit::event::WindowEvent::KeyboardInput {
                                input:
                                    winit::event::KeyboardInput {
                                        state: winit::event::ElementState::Pressed,
                                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = winit::event_loop::ControlFlow::Exit,
                            winit::event::WindowEvent::Resized(physical_size) => {
                                self.wgpu_manager.resize(
                                    *physical_size,
                                    &mut self.bundle_manager,
                                    &self.window_manager.window,
                                );
                            }
                            winit::event::WindowEvent::ScaleFactorChanged {
                                new_inner_size,
                                ..
                            } => {
                                self.wgpu_manager.resize(
                                    **new_inner_size,
                                    &mut self.bundle_manager,
                                    &self.window_manager.window,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                winit::event::Event::RedrawRequested(window_id)
                    if window_id == self.window_manager.window.id() =>
                {
                    self.wgpu_manager.update(&mut self.camera_manager);
                    match self.wgpu_manager.render(
                        self.bundle_manager.get_bundles(),
                        self.bundle_manager.get_depth_texture_view(),
                    ) {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => self.wgpu_manager.resize(
                            self.wgpu_manager.size,
                            &mut self.bundle_manager,
                            &self.window_manager.window,
                        ),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = winit::event_loop::ControlFlow::Exit
                        }
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    self.window_manager.window.request_redraw();
                }
                _ => {}
            });
    }
}
