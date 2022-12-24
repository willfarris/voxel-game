pub(crate) use cgmath::{Vector2, Vector3};
use cgmath::prelude::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub vtype: i32,
}

impl Vertex3D {
    pub(crate) fn postion_only(position: Vector3<f32>, tex_coords: Vector2<f32>) -> Self {
        let mut default = Self::default();
        default.position = position;
        default.tex_coords = tex_coords;
        default
    }
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