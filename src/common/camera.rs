use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

pub struct CameraManager {
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_controller: CameraController,
}

impl CameraManager {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let camera = super::camera::Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer - Camera"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("Bind Group Layout - Camera"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group - Camera"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let camera_controller = CameraController::new(0.2);

        Self {
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group_layout,
            camera_bind_group,
            camera_controller,
        }
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_rotation_left_pressed: bool,
    is_rotation_right_pressed: bool,
    is_zoom_in_pressed: bool,
    is_zoom_out_pressed: bool,
    reset: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_rotation_left_pressed: false,
            is_rotation_right_pressed: false,
            is_zoom_in_pressed: false,
            is_zoom_out_pressed: false,
            reset: false,
        }
    }

    pub fn process_events(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == winit::event::ElementState::Pressed;
                match keycode {
                    winit::event::VirtualKeyCode::W | winit::event::VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::A | winit::event::VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::S | winit::event::VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::D | winit::event::VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::Q => {
                        self.is_rotation_left_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::E => {
                        self.is_rotation_right_pressed = is_pressed;
                        true
                    }
                    winit::event::VirtualKeyCode::O => {
                        self.reset = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            winit::event::WindowEvent::MouseInput { button, state, .. } => {
                let is_pressed = *state == winit::event::ElementState::Pressed;
                match button {
                    winit::event::MouseButton::Left => {
                        self.is_zoom_in_pressed = is_pressed;
                        true
                    }
                    winit::event::MouseButton::Right => {
                        self.is_zoom_out_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::Angle;
        use cgmath::InnerSpace;

        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        let right = forward_norm.cross(camera.up);
        let right_norm = right.normalize();

        if self.is_forward_pressed {
            camera.eye.x += forward_norm.x * self.speed;
            camera.eye.z += forward_norm.z * self.speed;
            camera.target.x += forward_norm.x * self.speed;
            camera.target.z += forward_norm.z * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye.x -= forward_norm.x * self.speed;
            camera.eye.z -= forward_norm.z * self.speed;
            camera.target.x -= forward_norm.x * self.speed;
            camera.target.z -= forward_norm.z * self.speed;
        }
        if self.is_right_pressed {
            camera.eye += right_norm * self.speed;
            camera.target += right_norm * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= right_norm * self.speed;
            camera.target -= right_norm * self.speed;
        }
        if self.is_zoom_in_pressed && camera.eye.y >= camera.target.y + 0.5 {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_zoom_out_pressed && camera.eye.y <= 10.0 {
            camera.eye -= forward_norm * self.speed;
        }
        if self.is_rotation_right_pressed {
            let angle = cgmath::Deg(10.0 * self.speed);
            let sin = cgmath::Deg::sin(angle);
            let cos = cgmath::Deg::cos(angle);

            camera.eye.x -= camera.target.x;
            camera.eye.z -= camera.target.z;

            let new_x = camera.eye.x * cos - camera.eye.z * sin;
            let new_z = camera.eye.x * sin + camera.eye.z * cos;

            camera.eye.x = new_x + camera.target.x;
            camera.eye.z = new_z + camera.target.z;
        }
        if self.is_rotation_left_pressed {
            let angle = -cgmath::Deg(10.0 * self.speed);
            let sin = cgmath::Deg::sin(angle);
            let cos = cgmath::Deg::cos(angle);

            camera.eye.x -= camera.target.x;
            camera.eye.z -= camera.target.z;

            let new_x = camera.eye.x * cos - camera.eye.z * sin;
            let new_z = camera.eye.x * sin + camera.eye.z * cos;

            camera.eye.x = new_x + camera.target.x;
            camera.eye.z = new_z + camera.target.z;
        }
        if self.reset {
            camera.eye.x = camera.target.x - 5.0;
            camera.eye.z = camera.target.z;
        }
    }
}
