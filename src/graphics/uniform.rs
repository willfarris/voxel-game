use std::ffi::CString;

use cgmath::{Vector3, Vector2, Matrix4, Matrix3, Matrix2};

use super::shader::Shader;

pub trait Uniform {
    fn set_as_uniform(&self, shader: &Shader, name: &str);
}

impl Uniform for Matrix4<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &str) {
        let c_name = CString::new(name).unwrap();
        shader.set_mat4(c_name.as_c_str(), self);
    }
}

impl Uniform for Matrix3<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &str) {
        let c_name = CString::new(name).unwrap();
        shader.set_mat3(c_name.as_c_str(), self);
    }
}

impl Uniform for Matrix2<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &str) {
        let c_name = CString::new(name).unwrap();
        shader.set_mat2(c_name.as_c_str(), self);
    }
}

impl Uniform for Vector3<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &str) {
        let c_name = CString::new(name).unwrap();
        shader.set_vec3(c_name.as_c_str(), self);
    }
}

impl Uniform for Vector2<f32> {
    fn set_as_uniform(&self, shader: &Shader, name: &str) {
        let c_name = CString::new(name).unwrap();
        shader.set_vec2(c_name.as_c_str(), self);
    }
}

impl Uniform for f32 {
    fn set_as_uniform(&self, shader: &Shader, name: &str) {
        let c_name = CString::new(name).unwrap();
        shader.set_float(c_name.as_c_str(), *self);
    }
}
