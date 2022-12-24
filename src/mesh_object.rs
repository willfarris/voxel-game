use std::{mem::size_of, ffi::c_void, collections::HashMap, hash::Hash};

use cgmath::Vector2;
pub(crate) use cgmath::{Vector3, Matrix4, Quaternion, Rotation3, Deg};
use gl::types::GLsizeiptr;
use std::ptr;

use crate::{graphics::{vertex::Vertex3D, shader::Shader, resources::GLRenderable, texture::Texture}, c_str, offset_of};

pub(crate) const DEFAULT_CUBE: [Vertex3D; 36] = [
    // Facing positive-X
    Vertex3D { position: Vector3::new( 0.5, -0.5,  0.5), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },  // Front-bottom-right
    Vertex3D { position: Vector3::new( 0.5, -0.5, -0.5), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new( 0.5,  0.5,  0.5), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right

    Vertex3D { position: Vector3::new( 0.5,  0.5,  0.5), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
    Vertex3D { position: Vector3::new( 0.5, -0.5, -0.5), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new( 0.5,  0.5, -0.5), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },  // Back-top-right

    // Facing negative-X
    Vertex3D { position: Vector3::new(-0.5,  0.5,  0.5), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 }, // Front-top-left
    Vertex3D { position: Vector3::new(-0.5,  0.5, -0.5), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
    Vertex3D { position: Vector3::new(-0.5, -0.5,  0.5), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
    
    Vertex3D { position: Vector3::new(-0.5, -0.5,  0.5), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
    Vertex3D { position: Vector3::new(-0.5,  0.5, -0.5), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
    Vertex3D { position: Vector3::new(-0.5, -0.5, -0.5), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-left

    // Facing positive-Y
    Vertex3D { position: Vector3::new( 0.5,  0.5,  0.5), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new( 0.5,  0.5, -0.5), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
    Vertex3D { position: Vector3::new(-0.5,  0.5,  0.5), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left

    Vertex3D { position: Vector3::new(-0.5,  0.5,  0.5), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    Vertex3D { position: Vector3::new( 0.5,  0.5, -0.5), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
    Vertex3D { position: Vector3::new(-0.5,  0.5, -0.5), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-top-left
    
    // Facing negative-Y
    Vertex3D { position: Vector3::new( 0.5, -0.5,  0.5), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-bottom-right
    Vertex3D { position: Vector3::new(-0.5, -0.5,  0.5), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new( 0.5, -0.5, -0.5), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

    Vertex3D { position: Vector3::new(-0.5, -0.5,  0.5), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new(-0.5, -0.5, -0.5), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new( 0.5, -0.5, -0.5), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

    // Facing positive-Z
    Vertex3D { position: Vector3::new( 0.5,  0.5,  0.5), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new(-0.5,  0.5,  0.5), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    Vertex3D { position: Vector3::new(-0.5, -0.5,  0.5), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left

    Vertex3D { position: Vector3::new( 0.5,  0.5,  0.5), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new(-0.5, -0.5,  0.5), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new( 0.5, -0.5,  0.5), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Front-bottom-right

    // Facing negative-Z
    Vertex3D { position: Vector3::new( 0.5, -0.5, -0.5), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(-0.5, -0.5, -0.5), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new(-0.5,  0.5, -0.5), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex3D { position: Vector3::new( 0.5, -0.5, -0.5), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(-0.5,  0.5, -0.5), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex3D { position: Vector3::new( 0.5, 0.5, -0.5), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right

];

pub struct MeshObject {
    // Persists across OpenGL context creation
    vertices: Vec<Vertex3D>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    shader_src: (&'static str, &'static str),
    texture_bitmap: &'static [u8],

    // Recreated during init_gl_resources
    texture: Option<Texture>,
    shader: Option<Shader>,
    vao: Option<u32>,
    vbo: Option<u32>,
}

impl MeshObject {
    pub fn new(
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: Vector3<f32>,
        vertices: Vec<Vertex3D>,
        vertex_shader_src: &'static str,
        fragment_shader_src: &'static str,
        texture_bitmap: &'static [u8]
    ) -> MeshObject {
        MeshObject {
            position,
            rotation,
            scale,
            vertices,
            shader_src: (vertex_shader_src, fragment_shader_src),
            texture_bitmap,

            texture: None,
            shader: None,
            vao: None,
            vbo: None,
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }
}

impl GLRenderable for MeshObject {
    fn init_gl_resources(&mut self) {
        let mut vao = 0;
        let mut vbo = 0;

        let texture = Texture::from_dynamic_image_bytes(self.texture_bitmap, image::ImageFormat::Png);
        let shader = match Shader::new(self.shader_src.0, self.shader_src.1) {
            Ok(s) => s,
            Err(why) => {
                #[cfg(target_os = "android")] {
                    debug!("Could not create Mesh3D shader: {}", why);
                }
                panic!("Could not create Mesh3D shader: {}", why)
            }
        };
        #[cfg(target_os = "android")] {
            debug!("Created Mesh3D shader");
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            #[cfg(target_os = "android")] {
                debug!("Generated vertex arrays and buffers");
            }

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let size = (self.vertices.len() * size_of::<Vertex3D>()) as GLsizeiptr;
            let data = &self.vertices[0] as *const Vertex3D as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let stride = size_of::<Vertex3D>() as i32;
            
            // vertex Positions
            let position_location = gl::GetAttribLocation(shader.id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, position) as *const c_void);
            
            // vertex normals
            let normal_location = gl::GetAttribLocation(shader.id, c_str!("normal").as_ptr()) as u32;
            gl::EnableVertexAttribArray(normal_location);
            gl::VertexAttribPointer(normal_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, normal) as *const c_void);
            
            // vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(shader.id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, tex_coords) as *const c_void);
            
            // vertex type
            let vertex_type_location = gl::GetAttribLocation(shader.id, c_str!("vtype").as_ptr()) as u32;
            gl::EnableVertexAttribArray(vertex_type_location);
            gl::VertexAttribPointer(vertex_type_location, 1, gl::INT, gl::FALSE, stride, offset_of!(Vertex3D, vtype) as *const c_void);
        
        }

        self.texture = Some(texture);
        self.shader = Some(shader);
        self.vao = Some(vao);
        self.vbo = Some(vbo);

    }

    fn draw(&self, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, elapsed_time: f32) {
        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation = Quaternion::from_angle_x(Deg(self.rotation.x)) * Quaternion::from_angle_y(Deg(self.rotation.y)) * Quaternion::from_angle_z(Deg(self.rotation.z));
        let rotation_matrix = Matrix4::from(rotation);
        let translation_matrix = Matrix4::from_translation(self.position);
        let model_matrix = translation_matrix * rotation_matrix * scale_matrix;
        
        if let Some(shader) = &self.shader {
            unsafe {
                // use shader and set uniforms and texture
                shader.use_program();
                shader.set_mat4(c_str!("perspective_matrix"), &perspective_matrix);
                shader.set_mat4(c_str!("view_matrix"), &view_matrix);
                shader.set_mat4(c_str!("model_matrix"), &model_matrix);
                shader.set_float(c_str!("time"), elapsed_time);
                shader.set_texture(c_str!("texture_map"), 0);
            }

            // Draw vertices with DrawArrays
            unsafe {
                if let Some(t) = &self.texture {
                    gl::BindTexture(gl::TEXTURE_2D, t.id);
                }
                if let Some(vao) = self.vao {
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
                }
                gl::BindVertexArray(0);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }
}