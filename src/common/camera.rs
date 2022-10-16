use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[allow(dead_code)]
pub(crate) struct Camera {
    pub(crate) data: CameraData,
    pub(crate) controller: CameraController,
}

impl Camera {
    pub(crate) fn new(data: CameraData, controller: CameraController) -> Self {
        Self { data, controller }
    }
}

#[allow(dead_code)]
pub(crate) struct CameraData {
    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl CameraData {
    pub(crate) fn new(device: &wgpu::Device, controller: &CameraController) -> Self {
        let model_view_projection_matrix =
            (controller.projection_matrix * controller.view_matrix * controller.model_matrix)
                .to_cols_array(); // FIXME: could not work if not transpsed
        let model_view_projection_matrix_ref = &model_view_projection_matrix;

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer - Camera"),
            contents: bytemuck::cast_slice(model_view_projection_matrix_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout - Camera"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group - Camera"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            uniform_buffer,
            bind_group_layout,
            bind_group,
        }
    }
}

#[allow(dead_code)]
pub(crate) struct CameraController {
    transform: super::transforms::Transform,
    projection_matrix: glam::Mat4,
    view_matrix: glam::Mat4,
    model_matrix: glam::Mat4,
}

impl CameraController {
    pub(crate) fn new(size: &winit::dpi::PhysicalSize<u32>, eye: glam::Vec3) -> Self {
        let transform = super::transforms::Transform::from_position(eye);
        Self {
            model_matrix: transform.transform_matrix(),
            projection_matrix: glam::Mat4::perspective_rh(
                45_f32.to_radians(),
                size.width as f32 / size.height as f32,
                0.0,
                100.0,
            ),
            view_matrix: glam::Mat4::look_at_rh(
                super::transforms::Transform::IDENTITY.position,
                glam::Vec3::new(0.0, 0.0, 0.0),
                glam::Vec3::Y.normalize(),
            ),
            transform,
        }
    }
}
