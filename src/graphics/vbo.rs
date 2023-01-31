use std::{mem::size_of, ffi::c_void};

use gl::types::GLsizeiptr;

use crate::offset_of;

use super::vertex::Vertex3D;

pub(crate) trait VertexBufferObject {
    fn get_id(&self) -> u32;
    fn setup_for_current_vao(&self);

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.get_id());
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

struct VertexBuffer<T: Sized + Send + Sync> {
    id: u32,
    buffer: Vec<T>,
}

impl VertexBufferObject for VertexBuffer<Vertex3D> {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn setup_for_current_vao(&self) {
        let stride = size_of::<Vertex3D>();
        let size = (self.buffer.len() * stride) as GLsizeiptr;
        let data = &self.buffer[0] as *const Vertex3D as *const c_void;
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            // Vertex3D-specific attributes
            // 0 - position
            // 1 - normal
            // 2 - texture coords
            // 3 - vertex type

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
        }
    }
}