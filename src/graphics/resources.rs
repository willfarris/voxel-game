use std::collections::{HashMap, VecDeque};

use super::{
    framebuffer::Framebuffer, shader::Shader, texture::Texture, uniform::Uniform,
    vao::VertexAttributeObject, vbo::VertexBufferObject, vertex::VertexBufferContents,
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
        self.textures.retain(|_name, texture| {
            texture.delete();
            false
        });

        self.shaders.retain(|_name, shader| {
            shader.delete();
            false
        });

        self.vaos.retain(|_name, vao| {
            vao.delete();
            false
        });

        self.framebuffers.retain(|_name, framebuffer| {
            framebuffer.delete();
            false
        });

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

    pub fn add_vao(
        &mut self,
        name: String,
        buffer_contents: Box<dyn VertexBufferContents + Send + Sync>,
    ) {
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

    pub fn create_or_update_vao(
        &mut self,
        buffer_name: String,
        buffer_contents: Box<dyn VertexBufferContents + Send + Sync>,
    ) {
        if let Some(vao) = self.vaos.get_mut(&buffer_name) {
            vao.update_buffer(buffer_contents);
        } else {
            self.add_vao(buffer_name, buffer_contents);
        }
    }

    pub fn update_vao_buffer(
        &mut self,
        name: String,
        buffer_contents: Box<dyn VertexBufferContents + Send + Sync>,
    ) {
        self.vao_update_queue.push_front((name, buffer_contents));
    }

    pub fn process_vao_buffer_updates(&mut self, num_per_frame: usize) {
        for _ in 0..num_per_frame {
            if let Some((buffer_name, new_contents)) = self.vao_update_queue.pop_front() {
                self.create_or_update_vao(buffer_name, new_contents);
            }
        }
    }
}

pub trait GLRenderable {
    fn init_gl_resources(&self, gl_resources: &mut GLResources);
    fn draw(&self, gl_resources: &GLResources, uniforms: &[(&str, Box<dyn Uniform>)]);
}
