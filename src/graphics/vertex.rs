use std::ffi::c_void;

use cgmath::prelude::*;
pub(crate) use cgmath::{Vector2, Vector3};
use gl::types::GLsizeiptr;

use crate::offset_of;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub vtype: i32,
    pub lighting: f32,
}

impl Default for Vertex3D {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            vtype: 0,
            lighting: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex2D {
    pub position: Vector2<f32>,
    pub tex_coords: Vector2<f32>,
}
pub trait VertexBufferContents {
    fn setup_for_current_vbo(&self);
    fn get_length(&self) -> usize;
    fn get_raw_start_ptr(&self) -> *const c_void;
    fn get_stride(&self) -> usize;
}


impl VertexBufferContents for Vec<Vertex2D> {
    fn setup_for_current_vbo(&self) {
        let stride = std::mem::size_of::<Vertex2D>();
        let size = (self.len() * stride) as GLsizeiptr;
        let data = &self[0] as *const Vertex2D as *const c_void;
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            // Vertex3D-specific attributes
            // 0 - position
            // 1 - texture coords

            // vertex Positions
            let position_location = 0;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(
                position_location,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, position) as *const c_void,
            );

            // vertex texture coords
            let tex_coords_location = 1;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(
                tex_coords_location,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, tex_coords) as *const c_void,
            );
        }
    }

    fn get_length(&self) -> usize {
        self.len()
    }

    fn get_raw_start_ptr(&self) -> *const c_void {
        &self[0] as *const Vertex2D as *const c_void
    }

    fn get_stride(&self) -> usize {
        std::mem::size_of::<Vertex2D>()
    }
}

impl VertexBufferContents for Vec<Vertex3D> {
    fn setup_for_current_vbo(&self) {
        let stride = std::mem::size_of::<Vertex3D>();
        let size = (self.len() * stride) as GLsizeiptr;
        let data = &self[0] as *const Vertex3D as *const c_void;
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            // Vertex3D-specific attributes
            // 0 - position
            // 1 - normal
            // 2 - texture coords
            // 3 - vertex type
            // 4 - vertex lighting

            // vertex Positions
            let position_location = 0; //gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(
                position_location,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, position) as *const c_void,
            );

            // vertex normals
            let normal_location = 1; //gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("normal").as_ptr()) as u32;
            gl::EnableVertexAttribArray(normal_location);
            gl::VertexAttribPointer(
                normal_location,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, normal) as *const c_void,
            );

            // vertex texture coords
            let tex_coords_location = 2; //gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(
                tex_coords_location,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, tex_coords) as *const c_void,
            );

            // vertex type
            let vertex_type_location = 3; //gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("vtype").as_ptr()) as u32;
            gl::EnableVertexAttribArray(vertex_type_location);
            gl::VertexAttribPointer(
                vertex_type_location,
                1,
                gl::INT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, vtype) as *const c_void,
            );

            // lighting
            let lighting_location = 4; //gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("vtype").as_ptr()) as u32;
            gl::EnableVertexAttribArray(lighting_location);
            gl::VertexAttribPointer(
                lighting_location,
                1,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                offset_of!(Vertex3D, lighting) as *const c_void,
            );
        }
        
    }

    fn get_length(&self) -> usize {
        self.len()
    }

    fn get_raw_start_ptr(&self) -> *const c_void {
        &self[0] as *const Vertex3D as *const c_void
    }

    fn get_stride(&self) -> usize {
        std::mem::size_of::<Vertex3D>()
    }
}
