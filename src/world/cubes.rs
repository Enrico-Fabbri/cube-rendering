use cgmath::{InnerSpace, Rotation3, Zero};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    //color: [f32; 3],
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
                /*wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },*/
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0, 0.0],
    }, // A
    Vertex {
        position: [0.5, 0.0, 0.0],
    }, // B
    Vertex {
        position: [0.5, 0.5, 0.0],
    }, // C
    Vertex {
        position: [0.0, 0.5, 0.0],
    }, // D
    Vertex {
        position: [0.0, 0.5, 0.5],
    }, // E
    Vertex {
        position: [0.0, 0.0, 0.5],
    }, // F
    Vertex {
        position: [0.5, 0.0, 0.5],
    }, // G
    Vertex {
        position: [0.5, 0.5, 0.5],
    }, // H
];

const INDICES: &[u16] = &[
    0, 1, 2, // 1
    0, 2, 3, // 2
    5, 0, 3, // 3
    5, 3, 4, // 4
    6, 5, 4, // 5
    6, 4, 7, // 6
    1, 6, 7, // 7
    1, 7, 2, // 8
    5, 6, 1, // 9
    5, 1, 0, // 10
    7, 4, 2, // 11
    2, 4, 3, // 12
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CubeInstanceRaw {
    model: [[f32; 4]; 4],
    color: [f32; 3],
}

impl CubeInstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<CubeInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct CubeInstance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub color: cgmath::Vector3<f32>,
}

impl CubeInstance {
    pub fn to_raw(&self) -> CubeInstanceRaw {
        CubeInstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            color: (self.color).into(),
        }
    }
}

const NUM_INSTANCES_PER_ROW: u32 = 100;

pub struct Cubes {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instances: Vec<CubeInstance>,
    pub instance_buffer: wgpu::Buffer,
}

impl Cubes {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        depth_stencil: Option<wgpu::DepthStencilState>,
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
                buffers: &[Vertex::desc(), CubeInstanceRaw::desc()],
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

        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let position = cgmath::Vector3 {
                        x: x as f32 * 0.5,
                        y: 0.0,
                        z: z as f32 * 0.5,
                    };

                    let rotation = if position.is_zero() {
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0))
                    };

                    let color: cgmath::Vector3<f32> = cgmath::Vector3 {
                        x: 0.142_763_85,
                        y: 0.201_978_01,
                        z: 0.069_961_436,
                    };

                    CubeInstance {
                        position,
                        rotation,
                        color,
                    }
                })
            })
            .collect::<Vec<_>>();

        let instance_data = instances
            .iter()
            .map(CubeInstance::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer - Cubes"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            instances,
            instance_buffer,
        }
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
                label: Some("Render Bundle Encoder - Cubes"),
                color_formats: &[Some(config.format)],
                depth_stencil,
                sample_count: 1,
                multiview: None,
            });

        render_bundle_encoder.set_pipeline(&self.pipeline);
        render_bundle_encoder.set_bind_group(0, camera_bind_group, &[]);
        render_bundle_encoder.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_bundle_encoder.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_bundle_encoder
            .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_bundle_encoder.draw_indexed(
            0..(INDICES.len() as u32),
            0,
            0..self.instances.len() as _,
        );

        let render_bundle = render_bundle_encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some("Render Bundle - Cubes"),
        });

        bundle_manager.push_bundle(render_bundle);
    }
}
