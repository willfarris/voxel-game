use super::vertex::VertexBufferContents;
use gl::types::GLsizeiptr;

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
        Self { id, buffer }
    }

    pub fn update(&mut self, new_contents: Box<dyn VertexBufferContents + Send + Sync>) {
        self.buffer = new_contents;
        self.bind();
        let stride = self.buffer.get_stride();
        let buffer_size = (self.buffer.get_length() * stride) as GLsizeiptr;
        let data = self.buffer.get_raw_start_ptr();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, buffer_size, data, gl::STATIC_DRAW);
        }
        self.unbind();
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

    pub fn delete(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
