use super::vbo::VertexBufferObject;

pub(crate) struct VertexAttributeObject {
    id: u32,
    buffer: Option<Box<dyn VertexBufferObject + Sync + Send>>,
}

impl VertexAttributeObject {
    pub fn with_buffers(vertex_buffer: Box<dyn VertexBufferObject + Sync + Send>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        let mut vao = Self {
            id,
            buffer: None,
        };
        vao.bind();
        
        let buffer = vao.buffer.as_ref().unwrap();
        buffer.bind();
        buffer.setup_for_current_vao();
        buffer.unbind();
        vao.unbind();

        vao.buffer = Some(vertex_buffer);

        vao
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id)
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}