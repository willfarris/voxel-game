use std::{mem::size_of, ffi::c_void};

use gl::types::GLsizeiptr;

use crate::offset_of;

use super::vertex::{Vertex3D, VertexBufferContents};


pub struct VertexBufferObject {
    id: u32,
    buffer: Box<dyn VertexBufferContents + Send + Sync>,
}

impl VertexBufferObject {
    pub fn create_buffer(buffer: Box<dyn VertexBufferContents + Send + Sync>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id,
            buffer,
        }
    }

    pub fn update(&mut self, new_contents: Box<dyn VertexBufferContents + Send + Sync>) {
        self.bind();
        let stride = self.buffer.get_stride();
        let buffer_size = (self.buffer.get_length() * stride) as GLsizeiptr;
        let data = self.buffer.get_raw_start_ptr();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, buffer_size, data, gl::STATIC_DRAW);
        }
        self.unbind();
        self.buffer = new_contents;
    }

    pub fn get_length(&self) -> usize {
        self.buffer.get_length()
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn setup_for_current_vao(&self) {
        self.bind();
        self.buffer.setup_for_current_vbo();
        self.unbind();
    }
}