use wgpu::util::DeviceExt;

const VERTICES: &[super::Vertex] = &[
    super::Vertex {
        position: [-1.0, -1.0, 1.0], // vertex a
        color: [0.5, 0.5, 0.5],
    },
    super::Vertex {
        position: [1.0, -1.0, 1.0], // vertex b
        color: [0.0, 0.0, 1.0],
    },
    super::Vertex {
        position: [1.0, 1.0, 1.0], // vertex c
        color: [0.0, 1.0, 0.0],
    },
    super::Vertex {
        position: [-1.0, 1.0, 1.0], // vertex d
        color: [0.0, 1.0, 1.0],
    },
    super::Vertex {
        position: [-1.0, -1.0, -1.0], // vertex e
        color: [1.0, 0.0, 0.0],
    },
    super::Vertex {
        position: [1.0, -1.0, -1.0], // vertex f
        color: [1.0, 0.0, 1.0],
    },
    super::Vertex {
        position: [1.0, 1.0, -1.0], // vertex g
        color: [1.0, 1.0, 0.0],
    },
    super::Vertex {
        position: [-1.0, 1.0, -1.0], // vertex h
        color: [1.0, 1.0, 1.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // front
    1, 5, 6, 6, 2, 1, // right
    4, 7, 6, 6, 5, 4, // back
    0, 3, 7, 7, 4, 0, // left
    3, 2, 6, 6, 7, 3, // top
    0, 4, 5, 5, 1, 0, // bottom
];

pub(crate) struct TileState {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
}

impl TileState {
    pub(crate) fn new(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout - TileState"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline - TileState"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[super::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
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
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer - TileState"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer - TileState"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    pub(crate) fn indices_len() -> u32 {
        INDICES.len() as u32
    }
}
