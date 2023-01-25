use std::{ffi::c_void, mem::size_of, ptr};

use gl::{self, types::GLsizeiptr};

use crate::offset_of;

use super::vertex::Vertex3D;

pub struct BufferObject<T> {
    buffer_gl_object: u32,
    attribute_gl_object: u32,
    buffer_contents: Vec<T>,
    needs_update: bool,
}

impl<T> BufferObject<T> {
    pub fn new(buffer_contents: Vec<T>) -> Self {
        let (vao, vbo) = Self::crate_objects(&buffer_contents);

        Self {
            buffer_gl_object: vbo,
            attribute_gl_object: vao,
            buffer_contents,
            needs_update: false,
        }
    }

    fn crate_objects(buffer_contents: &Vec<T>) -> (u32, u32) {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let size = (buffer_contents.len() * size_of::<T>()) as GLsizeiptr;
            let data = &buffer_contents[0] as *const T as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let stride = size_of::<Vertex3D>() as i32;

            // vertex Positions
            let position_location = 0; //gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(
                position_location,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
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
                stride,
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
                stride,
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
                stride,
                offset_of!(Vertex3D, vtype) as *const c_void,
            );
        }

        (vao, vbo)
    }

    pub fn update_buffer(&mut self, new_contents: Vec<T>) {
        if new_contents.is_empty() {
            return;
        }
        self.buffer_contents = new_contents;
        unsafe {
            gl::BindVertexArray(self.attribute_gl_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_gl_object);

            let size = (self.buffer_contents.len() * size_of::<T>()) as GLsizeiptr;
            let data = &self.buffer_contents[0] as *const T as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
        }
        self.needs_update = false;
    }

    pub fn recreate_buffer(&mut self) {
        let (vao, vbo) = Self::crate_objects(&self.buffer_contents);
        self.attribute_gl_object = vao;
        self.buffer_gl_object = vbo;
        self.needs_update = false;
    }

    pub fn invalidate(&mut self) {
        self.needs_update = true;
        self.attribute_gl_object = 0;
        self.buffer_gl_object = 0;
    }

    pub fn is_valid(&self) -> bool {
        !self.needs_update
    }
}

impl BufferObject<Vertex3D> {
    pub fn bind_vertex_array(&self) {
        unsafe {
            gl::BindVertexArray(self.attribute_gl_object);
        }
    }

    pub fn draw_vertex_buffer(&self) {
        if self.buffer_contents.is_empty() {
            return;
        }
        unsafe {
            self.bind_vertex_array();
            gl::DrawArrays(gl::TRIANGLES, 0, self.buffer_contents.len() as i32);
        }
    }
}
