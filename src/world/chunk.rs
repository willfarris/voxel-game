use std::{ptr, collections::HashMap, mem::size_of, ffi::c_void};

use cgmath::{Vector3, Matrix4, Vector2};
use gl::types::GLsizeiptr;

use crate::{graphics::{resources::GLRenderable, texture::Texture, shader::Shader, vertex::Vertex3D}};

pub(crate) const CHUNK_SIZE: usize = 16;

const CUBE_FACES: [[Vertex3D; 6]; 10] = [
    
    // Facing positive-X
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },  // Front-bottom-right
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
    
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },  // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 }, // Front-top-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
        
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-bottom-right
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
    
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
    ],

    // Diagonal (0, 0) -> (1, 1)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
    ],

    // Diagonal (1, 1) -> (0, 0)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
    ],

    // Diagonal (0, 1) -> (1, 0)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
    ],

    // Diagonal (1, 0) -> (0, 1)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0 },
    ],
];

pub(crate) fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex3D>, texmap_offset: &(f32, f32), vertex_type: i32) {

    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v];
        vertex.position.x += position[0];
        vertex.position.y += position[1];
        vertex.position.z += position[2];

        vertex.tex_coords.x = vertex.tex_coords.x * 0.0625 + 0.0625 * texmap_offset.0 as f32;
        vertex.tex_coords.y = vertex.tex_coords.y * 0.0625 + 0.0625 * texmap_offset.1 as f32;

        vertex.vtype = vertex_type as i32;

        vertices.push(vertex);
    }
}

pub struct Chunk {
    // Persists across GL contexts
    pub blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub metadata: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],

    // Persists until chunk update
    vertices: Option<Vec<Vertex3D>>,

    // Rebuild when GL context is recreated
    shader: Option<Shader>,
    texture: Option<Texture>,
    vao: Option<u32>,
    vbo: Option<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            metadata: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],

            vertices: None,
            shader: None,
            texture: None,
            vao: None,
            vbo: None,
        }
    }

    pub fn rebuild_mesh(&mut self, vertices: Vec<Vertex3D>, shader: Option<Shader>, texture: Option<Texture>) {
        self.vertices = Some(vertices);

        let vertices = self.vertices.as_ref().unwrap();
        if vertices.len() == 0 {
            return;
        }

        self.shader = shader;
        self.texture = texture;

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let size = (vertices.len() * size_of::<Vertex3D>()) as GLsizeiptr;
            let data = &vertices[0] as *const Vertex3D as *const c_void;
            gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

            let stride = size_of::<Vertex3D>() as i32;
            
            // vertex Positions
            let position_location = gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("position").as_ptr()) as u32;
            gl::EnableVertexAttribArray(position_location);
            gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, position) as *const c_void);
            
            // vertex normals
            let normal_location = gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("normal").as_ptr()) as u32;
            gl::EnableVertexAttribArray(normal_location);
            gl::VertexAttribPointer(normal_location, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, normal) as *const c_void);
            
            // vertex texture coords
            let tex_coords_location = gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("tex_coords").as_ptr()) as u32;
            gl::EnableVertexAttribArray(tex_coords_location);
            gl::VertexAttribPointer(tex_coords_location, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex3D, tex_coords) as *const c_void);
            
            // vertex type
            let vertex_type_location = gl::GetAttribLocation(self.shader.as_ref().unwrap().id, c_str!("vtype").as_ptr()) as u32;
            gl::EnableVertexAttribArray(vertex_type_location);
            gl::VertexAttribPointer(vertex_type_location, 1, gl::INT, gl::FALSE, stride, offset_of!(Vertex3D, vtype) as *const c_void);
        
        }

        self.vao = Some(vao);
        self.vbo = Some(vbo);
    }

    pub fn draw(&self, position: Vector3<f32>, perspective_matrix: cgmath::Matrix4<f32>, view_matrix: cgmath::Matrix4<f32>, elapsed_time: f32) {
        if self.vertices.is_none() {
            return;
        }

        let model_matrix = Matrix4::from_translation(position);
        
        if let Some(shader) = &self.shader {
            unsafe {
                shader.use_program();
                shader.set_mat4(c_str!("perspective_matrix"), &perspective_matrix);
                shader.set_mat4(c_str!("view_matrix"), &view_matrix);
                shader.set_mat4(c_str!("model_matrix"), &model_matrix);
                shader.set_float(c_str!("time"), elapsed_time);
                shader.set_texture(c_str!("texture_map"), 0);
            }
        }

        unsafe {
            if let Some(texture) = self.texture {
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }
            if let Some(vao) = self.vao {
                gl::BindVertexArray(vao);
            }
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.as_ref().unwrap().len() as i32);
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn block_at_chunk_pos(&self, position: &Vector3<usize>) -> usize {
        self.blocks[position.x][position.y][position.z] as usize
    }
}