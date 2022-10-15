pub(crate) struct StoneHearthState {
    pub init: super::init::InitWgpu,
    pub tile_state: crate::world::tile::TileState,
}

impl StoneHearthState {
    pub(crate) async fn new(stonehearth_window: &super::window::StoneHearthWindow) -> Self {
        let init = super::init::InitWgpu::init_wgpu(&stonehearth_window.window).await;

        let tile_shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Tiles Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../../assets/shaders/tile.wgsl").into(),
                ),
            });

        let tile_state =
            crate::world::tile::TileState::new(&init.device, &tile_shader, &init.config);

        Self { init, tile_state }
    }

    pub(crate) fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);
        }
    }
    pub(crate) fn input(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&self) {}

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.init.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // This is what @location(0) in the fragment shader targets
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.tile_state.pipeline);
            render_pass.set_vertex_buffer(0, self.tile_state.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.tile_state.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..crate::world::tile::TileState::indices_len(), 0, 0..1);
        }

        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
