use std::{collections::HashMap, hash::Hash};

use cgmath::{Matrix4, Vector3};

use super::{
    buffer::BufferObject,
    framebuffer::Framebuffer,
    shader::Shader,
    texture::Texture,
    vertex::{Vertex2D, Vertex3D, VertexBufferContents}, vao::VertexAttributeObject, vbo::VertexBufferObject,
};

pub struct GLResources {
    pub(crate) textures: HashMap<&'static str, Texture>,
    pub(crate) shaders: HashMap<&'static str, Shader>,
    pub(crate) vaos: HashMap<String, VertexAttributeObject>,
    pub(crate) framebuffers: HashMap<&'static str, Framebuffer>,
    
    pub(crate) vao_update_queue: Vec<(String, Box<dyn VertexBufferContents + Send + Sync>)>,
}

impl GLResources {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            shaders: HashMap::new(),
            vaos: HashMap::new(),
            framebuffers: HashMap::new(),
            
            vao_update_queue: Vec::new(),
        }
    }

    pub fn process_vao_buffer_updates(&mut self, num_per_frame: usize) {
        for _ in 0..num_per_frame {
            if let Some((buffer_name, new_contents)) = self.vao_update_queue.pop() {
                if let Some(vao) = self.vaos.get_mut(&buffer_name) {
                    vao.update_buffer(new_contents);
                } else {
                    let vbo = VertexBufferObject::create_buffer(new_contents);
                    let vao = VertexAttributeObject::with_buffer(vbo);
                    self.vaos.insert(buffer_name, vao);
                }
            }
        }
    }
    
}

pub trait GLRenderable {
    fn init_gl_resources(&self, gl_resources: &mut GLResources);
    fn draw(
        &self,
        gl_resources: &GLResources,
        perspective_matrix: Matrix4<f32>,
        view_matrix: Matrix4<f32>,
        elapsed_time: f32,
    );
}
