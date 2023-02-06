pub(crate) use cgmath::{InnerSpace, Matrix4, Vector3, Vector4};

use crate::physics::vectormath::{quaternion_rotate, Y_VECTOR};

pub struct Camera {
    pub position: Vector3<f32>,
    pub forward: Vector3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, direction: Vector3<f32>) -> Self {
        let n_direction = direction.normalize();
        let mut s = Self {
            position,
            forward: n_direction,
            right: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 0.0, 0.0),
        };
        s.calculate_normals();
        s
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let p: Vector3<f32> = Vector3::new(
            -self.position.x * self.right.x
                - self.position.y * self.right.y
                - self.position.z * self.right.z,
            -self.position.x * self.up.x
                - self.position.y * self.up.y
                - self.position.z * self.up.z,
            -self.position.x * self.forward.x
                - self.position.y * self.forward.y
                - self.position.z * self.forward.z,
        );

        Matrix4::from_cols(
            Vector4::new(self.right.x, self.up.x, self.forward.x, 0.0),
            Vector4::new(self.right.y, self.up.y, self.forward.y, 0.0),
            Vector4::new(self.right.z, self.up.z, self.forward.z, 0.0),
            Vector4::new(p.x, p.y, p.z, 1.0),
        )
    }

    fn calculate_normals(&mut self) {
        self.forward = self.forward.normalize();
        self.right = Y_VECTOR.cross(self.forward).normalize();
        self.up = self.forward.cross(self.right).normalize();
    }

    pub fn translate(&mut self, new_position: Vector3<f32>) {
        self.position = new_position;
    }

    pub fn rotate_on_y_axis(&mut self, angle: f32) {
        self.forward = quaternion_rotate(self.forward, angle, self.up);
        self.calculate_normals();
    }

    pub fn rotate_on_x_axis(&mut self, angle: f32) {
        self.forward = quaternion_rotate(self.forward, angle, self.right);
        self.calculate_normals();
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.forward
    }
}

pub fn perspective_matrix(width: i32, height: i32, render_distance_chunks: f32) -> Matrix4<f32> {
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = std::f32::consts::PI / 2.0;
    let zfar = 16.0 * render_distance_chunks;
    let znear = 0.01;

    let f = 1.0 / (fov / 2.0).tan();

    Matrix4::from_cols(
        Vector4::new(f * aspect_ratio, 0.0, 0.0, 0.0),
        Vector4::new(0.0, f, 0.0, 0.0),
        Vector4::new(0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0),
        Vector4::new(0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0),
    )
}
