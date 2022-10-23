use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.12, 0.19, 0.033],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.12, 0.19, 0.033],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.12, 0.19, 0.033],
    },
];

const INDICES: &[u16] = &[0, 1, 2];

pub struct Cubes {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Cubes {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader - Cubes"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../assets/shaders/cubes.wgsl").into(),
            ),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer - Cubes"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer - Cubes"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout - Cubes"),
            bind_group_layouts: &[camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline - Cubes"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
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

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn finish_bundle(
        &self,
        bundle_manager: &mut crate::common::bundles::BundleManager,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group: &wgpu::BindGroup,
    ) {
        let mut render_bundle_encoder =
            device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                label: Some("Render Bundle Encoder - Cubes"),
                color_formats: &[Some(config.format)],
                depth_stencil: None,
                sample_count: 1,
                multiview: None,
            });

        render_bundle_encoder.set_pipeline(&self.pipeline);
        render_bundle_encoder.set_bind_group(0, camera_bind_group, &[]);
        render_bundle_encoder.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_bundle_encoder
            .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_bundle_encoder.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);

        let render_bundle = render_bundle_encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Bundle - Cubes"),
        });

        bundle_manager.push_bundle(render_bundle);
    }
}
