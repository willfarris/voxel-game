use super::{vbo::VertexBufferObject, vertex::VertexBufferContents};

pub struct VertexAttributeObject {
    id: u32,
    buffer: VertexBufferObject,
}

impl VertexAttributeObject {
    pub fn with_buffer(buffer: VertexBufferObject) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        let vao = Self {
            id,
            buffer,
        };

        vao.setup_vbo();

        vao
    }

    pub fn update_buffer(&mut self, new_contents: Box<dyn VertexBufferContents + Send + Sync>) {
        self.buffer.update(new_contents);
    }

    pub fn setup_vbo(&self) {
        self.bind();
        self.buffer.setup_for_current_vao();
        self.unbind();
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id)
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        self.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.buffer.get_length() as i32);
        }
        self.unbind();
    }

    pub fn delete(&mut self) {
        self.bind();
        self.buffer.delete();
        self.unbind();
        unsafe {
            gl::DeleteVertexArrays(1, &self.id)
        }
    }
}