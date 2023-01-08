use std::collections::HashMap;

use cgmath::Matrix4;

use super::{shader::Shader, texture::Texture, buffer::BufferObject, vertex::Vertex3D};

pub struct GLResources {
    textures: HashMap<&'static str, Texture>,
    shaders: HashMap<&'static str, Shader>,
    buffers: HashMap<String, BufferObject<Vertex3D>>,

    buffer_update_queue: Vec<(String, Vec<Vertex3D>)>,
}

impl GLResources {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            shaders: HashMap::new(),
            buffers: HashMap::new(),

            buffer_update_queue: Vec::new(),
        }
    }

    pub fn create_shader(&mut self, key: &'static str, vert_shader_src: &str, frag_shader_src: &str) {
        let shader = Shader::new(vert_shader_src, frag_shader_src).unwrap();
        self.shaders.insert(key, shader);
    }

    pub fn create_texture(&mut self, key: &'static str, bitmap: &[u8]) {
        let texture = Texture::from_dynamic_image_bytes(bitmap, image::ImageFormat::Png);
        self.textures.insert(key, texture);
    }

    pub fn get_texture(&self, key: &str) -> Option<Texture> {
        if let Some(texture) = self.textures.get(key) {
            Some(texture.clone())
        } else {
           None
        }   
    }

    pub fn get_shader(&self, key: &str) -> Option<Shader> {
        if let Some(shader) = self.shaders.get(key) {
            Some(shader.clone())
        } else {
            None
        }
    }

    pub fn get_buffer(&mut self, name: String) -> Option<&mut BufferObject<Vertex3D>> {
        if let Some(buffer) = self.buffers.get_mut(name.as_str()) {
            if buffer.is_valid() {
                Some(buffer)
            } else {
                buffer.recreate_buffer();
                Some(buffer)
            }
        } else {
            None
        }
    }

    pub fn create_buffer_from_verts(&mut self, name: String, buffer_contents: Vec<Vertex3D>) {
        let buffer = BufferObject::new(buffer_contents);
        self.buffers.insert(name.clone(), buffer);
    }

    pub fn update_buffer(&mut self, name: String, new_contents: Vec<Vertex3D>) {
        self.buffer_update_queue.push((name, new_contents));
    }

    pub fn process_buffer_updates(&mut self) {
        while let Some((buffer_name, new_contents)) = self.buffer_update_queue.pop() {
            if let Some(buffer) = self.buffers.get_mut(&buffer_name) {
                buffer.update_buffer(new_contents);
            }  else {
                self.create_buffer_from_verts(buffer_name, new_contents);
            }
        }
        assert_eq!(self.buffer_update_queue.len(), 0);
    }

    pub fn invalidate_resources(&mut self) {
        self.shaders.clear();
        self.textures.clear();
        for (_buffer_name, buffer) in &mut self.buffers {
            buffer.invalidate();
        }
    }

}

pub trait GLRenderable {
    fn init_gl_resources(&self, gl_resources: &mut GLResources);
    fn draw(&self, gl_resources: &mut GLResources, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, elapsed_time: f32);
}

