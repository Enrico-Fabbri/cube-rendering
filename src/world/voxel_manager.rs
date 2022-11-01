use wgpu::util::DeviceExt;

pub struct VoxelManger {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub voxels: Vec<super::voxel::Voxel>,
    pub instances_model_buffer: wgpu::Buffer,
    pub instances_render_buffer: wgpu::Buffer,
}

impl VoxelManger {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        depth_stencil: Option<wgpu::DepthStencilState>,
        voxel_number: u32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader - Voxel Manager"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../assets/shaders/cube.wgsl").into()),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer - Voxel Manager"),
            contents: bytemuck::cast_slice(super::voxel::face::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer - Voxel Manager"),
            contents: bytemuck::cast_slice(super::voxel::face::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout - Voxel Manager"),
            bind_group_layouts: &[camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline - Voxel Manager"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    super::voxel::face::Vertex::desc(),
                    super::voxel::face::FaceInstanceModelRaw::desc(),
                    super::voxel::face::FaceInstanceRenderRaw::desc(),
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let voxels = Self::gen_voxels((voxel_number as f32).sqrt().ceil() as u32);

        let instances_model_data = voxels
            .iter()
            .map(|v| v.get_data().0)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let instances_render_data = voxels
            .iter()
            .map(|v| v.get_data().1)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let instances_model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Model Buffer - Voxel Manager"),
            contents: bytemuck::cast_slice(&instances_model_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let instances_render_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Render Buffer - Voxel Manager"),
                contents: bytemuck::cast_slice(&instances_render_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            voxels,
            instances_model_buffer,
            instances_render_buffer,
        }
    }

    fn gen_voxels(voxel_number: u32) -> Vec<super::voxel::Voxel> {
        (0..voxel_number)
            .flat_map(|z| {
                (0..voxel_number)
                    .map(move |x| super::voxel::Voxel::new(&cgmath::vec3(x as f32, 0.0, z as f32)))
            })
            .collect::<Vec<_>>()
    }

    pub fn update_buffers(mut self, device: &wgpu::Device) -> Self {
        self.instances_render_buffer.destroy();
        self.instances_model_buffer.destroy();

        let instances_model_data = self
            .voxels
            .iter()
            .map(|v| v.get_data().0)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        self.instances_model_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Model Buffer - Voxel Manager"),
                contents: bytemuck::cast_slice(&instances_model_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let instances_render_data = self
            .voxels
            .iter()
            .map(|v| v.get_data().1)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        self.instances_render_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Render Buffer - Voxel Manager"),
                contents: bytemuck::cast_slice(&instances_render_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        self
    }

    pub fn update_map(mut self) -> Self {
        for (index, n) in self
            .get_neighbours()
            .iter()
            .enumerate()
            .take(self.voxels.len())
        {
            self.voxels[index].set_faces(
                Some(n[0]),
                Some(n[1]),
                Some(n[2]),
                Some(n[3]),
                Some(n[4]),
                Some(n[5]),
            );

            self.voxels[index].update_instance_data();
        }

        self
    }

    fn get_neighbours(&self) -> Vec<Vec<bool>> {
        let mut ns = Vec::new();
        for v in self.voxels.iter() {
            ns.push(self.get_neighbour(v));
        }
        ns
    }

    fn get_neighbour(&self, v: &super::voxel::Voxel) -> Vec<bool> {
        let front = self
            .voxels
            .iter()
            .any(|voxel| voxel.position.z == v.position.z + 1.0);
        let back = self
            .voxels
            .iter()
            .any(|voxel| voxel.position.z == v.position.z - 1.0);
        let left = self
            .voxels
            .iter()
            .any(|voxel| voxel.position.x == v.position.x - 1.0);
        let right = self
            .voxels
            .iter()
            .any(|voxel| voxel.position.x == v.position.x + 1.0);
        let up = self
            .voxels
            .iter()
            .any(|voxel| voxel.position.y == v.position.y + 1.0);
        let down = self
            .voxels
            .iter()
            .any(|voxel| voxel.position.y == v.position.y - 1.0);
        vec![front, back, left, right, up, down]
    }

    pub fn finish_bundle(
        &self,
        bundle_manager: &mut crate::common::bundles::BundleManager,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group: &wgpu::BindGroup,
        depth_stencil: Option<wgpu::RenderBundleDepthStencil>,
    ) {
        let mut render_bundle_encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("Render Bundle Encoder - Voxel Manager"),
                color_formats: &[Some(config.format)],
                depth_stencil,
                sample_count: 1,
                multiview: None,
            });

        render_bundle_encoder.set_pipeline(&self.pipeline);

        render_bundle_encoder.set_bind_group(0, camera_bind_group, &[]);

        render_bundle_encoder.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_bundle_encoder.set_vertex_buffer(1, self.instances_model_buffer.slice(..));
        render_bundle_encoder.set_vertex_buffer(2, self.instances_render_buffer.slice(..));

        render_bundle_encoder
            .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_bundle_encoder.draw_indexed(
            0..(super::voxel::face::INDICES.len() as u32),
            0,
            0..(self.voxels.len() as u32 * 6),
        );

        let render_bundle = render_bundle_encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Bundle - Voxel Manager"),
        });

        bundle_manager.push_bundle(render_bundle);
    }
}
