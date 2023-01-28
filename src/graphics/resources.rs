use std::collections::HashMap;

use cgmath::{Matrix4, Vector3};

use super::{
    buffer::BufferObject,
    framebuffer::Framebuffer,
    shader::Shader,
    texture::Texture,
    vertex::{Vertex2D, Vertex3D},
};

pub struct GLResources {
    textures: HashMap<&'static str, Texture>,
    shaders: HashMap<&'static str, Shader>,
    buffers: HashMap<String, BufferObject<Vertex3D>>,

    pub(crate) gbuffer: Option<Framebuffer>,
    pub(crate) screenquad: Option<BufferObject<Vertex3D>>,
    pub(crate) gbuffer_program: Option<Shader>,
    pub(crate) ssao_kernel: Option<[Vector3<f32>; 64]>,
    pub(crate) ssao_noise: Option<Texture>,

    buffer_update_queue: Vec<(String, Vec<Vertex3D>)>,
}

impl GLResources {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            shaders: HashMap::new(),
            buffers: HashMap::new(),

            gbuffer: None,
            screenquad: None,
            gbuffer_program: None,
            ssao_kernel: None,
            ssao_noise: None,

            buffer_update_queue: Vec::new(),
        }
    }

    pub fn create_shader(
        &mut self,
        key: &'static str,
        vert_shader_src: &str,
        frag_shader_src: &str,
    ) {
        let shader = Shader::new(vert_shader_src, frag_shader_src).unwrap();
        self.shaders.insert(key, shader);
    }

    pub fn create_texture(&mut self, key: &'static str, bitmap: &[u8]) {
        let texture = Texture::from_dynamic_image_bytes(bitmap, image::ImageFormat::Png);
        self.textures.insert(key, texture);
    }

    pub fn get_texture(&self, key: &str) -> Option<Texture> {
        self.textures.get(key).copied()
    }

    pub fn get_shader(&self, key: &str) -> Option<Shader> {
        self.shaders.get(key).copied()
    }

    pub fn get_buffer(&self, name: String) -> Option<&BufferObject<Vertex3D>> {
        if let Some(buffer) = self.buffers.get(name.as_str()) {
            if buffer.is_valid() {
                Some(buffer)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn create_buffer_from_verts(&mut self, name: String, buffer_contents: Vec<Vertex3D>) {
        let buffer = BufferObject::new(buffer_contents);
        self.buffers.insert(name, buffer);
    }

    pub fn update_buffer(&mut self, name: String, new_contents: Vec<Vertex3D>) {
        self.buffer_update_queue.push((name, new_contents));
    }

    pub fn process_buffer_updates(&mut self, num_per_frame: usize) {
        for _ in 0..num_per_frame {
            if let Some((buffer_name, new_contents)) = self.buffer_update_queue.pop() {
                if let Some(buffer) = self.buffers.get_mut(&buffer_name) {
                    buffer.update_buffer(new_contents);
                } else {
                    self.create_buffer_from_verts(buffer_name, new_contents);
                }
            }
        }
    }

    pub fn invalidate_resources(&mut self) {
        self.shaders.clear();
        self.textures.clear();
        for buffer in self.buffers.values_mut() {
            buffer.invalidate();
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
