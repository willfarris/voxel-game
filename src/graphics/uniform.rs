use std::ffi::CString;

use cgmath::{Vector3, Vector2, Matrix4};

use super::shader::Shader;

pub trait Uniform {
    fn set_as_uniform(&self, shader: &Shader, name: &'static str);
}

impl Uniform for Matrix4<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &'static str) {
        let c_name = CString::new(name).unwrap();
        shader.set_mat4(c_name.as_c_str(), self);
    }
}

impl Uniform for Vector2<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &'static str) {
        let c_name = CString::new(name).unwrap();
        shader.set_vec2(c_name.as_c_str(), self);
    }
}

impl Uniform for Vector3<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &'static str) {
        let c_name = CString::new(name).unwrap();
        shader.set_vec3(c_name.as_c_str(), self);
    }
}