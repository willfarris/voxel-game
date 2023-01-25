use cgmath::prelude::*;
pub(crate) use cgmath::{Vector2, Vector3};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub vtype: i32,
}

impl Default for Vertex3D {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            vtype: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex2D {
    pub position: Vector2<f32>,
    pub tex_coords: Vector2<f32>,
}
