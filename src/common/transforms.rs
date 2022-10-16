use glam::*;

#[allow(dead_code)]
pub(crate) struct Transform {
    pub(crate) scale: Vec3,
    pub(crate) position: Vec3,
    pub(crate) rotation: Vec3,
    pub(crate) speed: Vec3,
}

impl Transform {
    pub(crate) const IDENTITY: Self = Self {
        scale: Vec3::ZERO,
        position: Vec3::ZERO,
        rotation: Vec3::ZERO,
        speed: Vec3::ZERO,
    };

    #[allow(dead_code)]
    pub(crate) fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Self::IDENTITY
        }
    }

    pub(crate) fn from_position(position: Vec3) -> Self {
        Self {
            position,
            ..Self::IDENTITY
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_rotation(rotation: Vec3) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_speed(speed: Vec3) -> Self {
        Self {
            speed,
            ..Self::IDENTITY
        }
    }

    pub(crate) fn transform_matrix(&self) -> Mat4 {
        transform_matrix(self.scale, self.position, self.rotation)
    }
}

pub(crate) fn scale_matrix(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4 {
        x_axis: Vec4::new(x, 0.0, 0.0, 0.0),
        y_axis: Vec4::new(0.0, y, 0.0, 0.0),
        z_axis: Vec4::new(0.0, 0.0, z, 0.0),
        w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
    }
}

pub(crate) fn translation_matrix(x: f32, y: f32, z: f32) -> Mat4 {
    Mat4 {
        x_axis: Vec4::new(1.0, 0.0, 0.0, x),
        y_axis: Vec4::new(0.0, 1.0, 0.0, y),
        z_axis: Vec4::new(0.0, 0.0, 1.0, z),
        w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
    }
}

pub(crate) fn rotation_matrix_x(angle: f32) -> Mat4 {
    Mat4::from_rotation_x(angle)
}

pub(crate) fn rotation_matrix_y(angle: f32) -> Mat4 {
    Mat4::from_rotation_y(angle)
}

pub(crate) fn rotation_matrix_z(angle: f32) -> Mat4 {
    Mat4::from_rotation_z(angle)
}

pub(crate) fn transform_matrix(scale: Vec3, translation: Vec3, rotation: Vec3) -> Mat4 {
    translation_matrix(translation.x, translation.y, translation.z)
        * rotation_matrix_z(rotation.z)
        * rotation_matrix_y(rotation.y)
        * rotation_matrix_x(rotation.x)
        * scale_matrix(scale.x, scale.y, scale.z)
}
