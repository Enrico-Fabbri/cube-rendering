pub mod face;

use cgmath::Rotation3;

use self::face::{FaceInstance, FaceInstanceModelRaw, FaceInstanceRenderRaw};

pub const NUM_INSTANCES_PER_CUBE: u32 = 6;

pub struct Voxel {
    pub instances: Vec<FaceInstance>,
    pub position: cgmath::Vector3<f32>,
    instance_model_data: Vec<FaceInstanceModelRaw>,
    instance_render_data: Vec<FaceInstanceRenderRaw>,
}

impl Voxel {
    pub fn new(position: &cgmath::Vector3<f32>) -> Self {
        let instances = Self::gen_instances(position);
        let instance_data = instances
            .iter()
            .map(FaceInstance::to_raw)
            .collect::<Vec<(_, _)>>();
        let instance_model_data = instance_data.iter().map(|data| data.0).collect::<Vec<_>>();
        let instance_render_data = instance_data.iter().map(|data| data.1).collect::<Vec<_>>();

        Self {
            instances,
            position: *position,
            instance_model_data,
            instance_render_data,
        }
    }

    pub fn update_instance_data(&mut self) {
        let instance_data = self
            .instances
            .iter()
            .map(FaceInstance::to_raw)
            .collect::<Vec<(_, _)>>();

        self.instance_model_data = instance_data.iter().map(|data| data.0).collect::<Vec<_>>();
        self.instance_render_data = instance_data.iter().map(|data| data.1).collect::<Vec<_>>();
    }

    fn gen_instances(position: &cgmath::Vector3<f32>) -> Vec<FaceInstance> {
        (0..NUM_INSTANCES_PER_CUBE)
            .map(|i| Self::gen_instance(i, position))
            .collect::<Vec<_>>()
    }

    fn gen_instance(index: u32, position: &cgmath::Vector3<f32>) -> FaceInstance {
        let x = position.x;
        let y = position.y;
        let z = position.z;

        match index {
            0 => {
                let position = cgmath::Vector3 { x, y, z };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_z(),
                    cgmath::Deg(0.0),
                );

                FaceInstance {
                    position,
                    rotation,
                    render: true,
                }
            }
            1 => {
                let position = cgmath::Vector3 { x, y, z: z + 1.0 };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_y(),
                    cgmath::Deg(90.0),
                );

                FaceInstance {
                    position,
                    rotation,
                    render: true,
                }
            }
            2 => {
                let position = cgmath::Vector3 {
                    x: x + 1.0,
                    y,
                    z: z + 1.0,
                };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_y(),
                    cgmath::Deg(180.0),
                );

                FaceInstance {
                    position,
                    rotation,
                    render: true,
                }
            }
            3 => {
                let position = cgmath::Vector3 { x: x + 1.0, y, z };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_y(),
                    cgmath::Deg(270.0),
                );

                FaceInstance {
                    position,
                    rotation,
                    render: true,
                }
            }
            4 => {
                let position = cgmath::Vector3 { x, y: y + 1.0, z };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_x(),
                    cgmath::Deg(90.0),
                );

                FaceInstance {
                    position,
                    rotation,
                    render: true,
                }
            }
            5 => {
                let position = cgmath::Vector3 {
                    x: x + 1.0,
                    y,
                    z: z + 1.0,
                };

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_x(),
                    cgmath::Deg(270.0),
                );

                FaceInstance {
                    position,
                    rotation,
                    render: true,
                }
            }
            _ => panic!("Index not Implemented in gen_instance of Cube struct"),
        }
    }

    pub fn set_faces(
        &mut self,
        front: Option<bool>,
        back: Option<bool>,
        left: Option<bool>,
        right: Option<bool>,
        up: Option<bool>,
        down: Option<bool>,
    ) {
        if let Some(f) = front {
            self.instances[2].render = !f;
        }
        if let Some(b) = back {
            self.instances[0].render = !b;
        }
        if let Some(l) = left {
            self.instances[1].render = !l;
        }
        if let Some(r) = right {
            self.instances[3].render = !r;
        }
        if let Some(u) = up {
            self.instances[4].render = !u;
        }
        if let Some(d) = down {
            self.instances[5].render = !d;
        }
    }

    pub fn get_data(&self) -> (Vec<FaceInstanceModelRaw>, Vec<FaceInstanceRenderRaw>) {
        (
            self.instance_model_data.clone(),
            self.instance_render_data.clone(),
        )
    }
}
