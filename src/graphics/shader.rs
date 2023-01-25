use cgmath::{Array, Matrix, Matrix2, Matrix3, Matrix4, Vector2, Vector3};
pub(crate) use std::ffi::CStr;
use std::{ffi::CString, ptr};

#[derive(Clone, Copy, Debug)]
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_str: &str, fragment_str: &str) -> Result<Self, String> {
        let mut shader_program: Shader = Shader { id: 0u32 };
        unsafe {
            // vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_str.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // check for shader compile errors
            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = [0u8; 512];
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                return Err(String::from(std::str::from_utf8(&info_log).unwrap()));
            }

            // fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_str.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            // check for shader compile errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                return Err(String::from(std::str::from_utf8(&info_log).unwrap()));
            }

            // link shaders
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);
            // check for linking errors
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetProgramInfoLog(
                    program_id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                return Err(String::from(std::str::from_utf8(&info_log).unwrap()));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program.id = program_id;
        }
        Ok(shader_program)
    }

    pub fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    pub fn set_mat3(&self, name: &CStr, mat: &Matrix3<f32>) {
        unsafe {
            gl::UniformMatrix3fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    pub fn set_mat2(&self, name: &CStr, mat: &Matrix2<f32>) {
        unsafe {
            gl::UniformMatrix2fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    pub fn set_vec3(&self, name: &CStr, vec: &Vector3<f32>) {
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                vec.as_ptr(),
            );
        }
    }

    pub fn set_vec2(&self, name: &CStr, vec: &Vector2<f32>) {
        unsafe {
            gl::Uniform2fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                vec.as_ptr(),
            );
        }
    }

    pub fn set_float(&self, name: &CStr, float: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), float);
        }
    }

    pub fn set_texture(&self, name: &CStr, index: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), index);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
