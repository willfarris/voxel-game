use std::{collections::{HashMap, VecDeque}, hash::Hash};

use cgmath::{Matrix4, Vector3};

use super::{
    buffer::BufferObject,
    framebuffer::{Framebuffer, self},
    shader::Shader,
    texture::Texture,
    vertex::{Vertex2D, Vertex3D, VertexBufferContents}, vao::VertexAttributeObject, vbo::VertexBufferObject,
};

pub struct GLResources {
    textures: HashMap<&'static str, Texture>,
    shaders: HashMap<&'static str, Shader>,
    vaos: HashMap<String, VertexAttributeObject>,
    framebuffers: HashMap<&'static str, Framebuffer>,
    
    vao_update_queue: VecDeque<(String, Box<dyn VertexBufferContents + Send + Sync>)>,
}

impl GLResources {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            shaders: HashMap::new(),
            vaos: HashMap::new(),
            framebuffers: HashMap::new(),
            
            vao_update_queue: VecDeque::new(),
        }
    }

    pub fn invalidate_resources(&mut self) {
        self.textures.clear();
        self.shaders.clear();
        self.vaos.clear();
        self.framebuffers.clear();
        self.vao_update_queue.clear();
    }

    pub fn add_shader(&mut self, name: &'static str, shader: Shader) {
        self.shaders.insert(name, shader);
    }

    pub fn add_texture(&mut self, name: &'static str, texture: Texture) {
        self.textures.insert(name, texture);
    }

    pub fn add_framebuffer(&mut self, name: &'static str, framebuffer: Framebuffer) {
        self.framebuffers.insert(name, framebuffer);
    }

    pub fn add_vao(&mut self, name: String, buffer_contents: Box<dyn VertexBufferContents + Send + Sync>) {
        let vbo = VertexBufferObject::create_buffer(buffer_contents);
        let vao = VertexAttributeObject::with_buffer(vbo);
        self.vaos.insert(name, vao);
    }

    pub fn get_shader(&self, name: &str) -> Option<&Shader> {
        self.shaders.get(name)
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn get_vao(&self, name: &str) -> Option<&VertexAttributeObject> {
        self.vaos.get(name)
    }

    pub fn get_framebuffer(&self, name: &str) -> Option<&Framebuffer> {
        self.framebuffers.get(name)
    }

    pub fn update_vao_buffer(&mut self, name: String, buffer_contents: Box<dyn VertexBufferContents + Send + Sync>) {
        self.vao_update_queue.push_back((name, buffer_contents));
    }

    pub fn process_vao_buffer_updates(&mut self, num_per_frame: usize) {
        for _ in 0..num_per_frame {
            if let Some((buffer_name, new_contents)) = self.vao_update_queue.pop_front() {
                if let Some(vao) = self.vaos.get_mut(&buffer_name) {
                    vao.update_buffer(new_contents);
                } else {
                    self.add_vao(buffer_name, new_contents);
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
