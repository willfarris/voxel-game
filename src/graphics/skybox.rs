use cgmath::{Vector2, Vector3};
use image::ImageFormat;

use crate::c_str;

use super::{
    resources::GLRenderable,
    shader::Shader,
    source::{SKYBOX_BITMAP, SKYBOX_FRAG_SRC, SKYBOX_VERT_SRC},
    texture::Texture,
    vertex::Vertex3D,
};

const HALF: f32 = 0.5;
const THIRD: f32 = 1.0 / 3.0;

pub(crate) const SKYBOX_VERTS: [Vertex3D; 36] = [
    // Facing positive-X
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, 0.50),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(HALF, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, -0.50),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, 0.50),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, 0.50),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, -0.50),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, -0.5),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-right
    // Facing negative-X
    Vertex3D {
        position: Vector3::new(0.50, 0.50, 0.50),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(0.50, 0.50, -0.50),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(HALF, 1.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(0.5, -0.50, 0.5),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(0.50, -0.50, 0.50),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(0.50, 0.50, -0.50),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(HALF, 1.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(0.50, -0.50, -0.50),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-left
    // Facing positive-Y
    Vertex3D {
        position: Vector3::new(0.50, -0.50, 0.50),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(1.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(0.50, -0.50, -0.50),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-right
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, 0.50),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(HALF, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, 0.50),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(HALF, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(0.50, -0.50, -0.50),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-right
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, -0.50),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(HALF, 0.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-left
    // Facing negative-Y
    Vertex3D {
        position: Vector3::new(0.50, 0.50, 0.50),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(1.0, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, 0.50),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(0.50, 0.50, -0.50),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, 0.50),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, -0.50),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(HALF, 1.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-left
    Vertex3D {
        position: Vector3::new(0.50, 0.50, -0.50),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-right
    // Facing positive-Z
    Vertex3D {
        position: Vector3::new(0.50, 0.50, -0.50),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(0.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, -0.50),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(HALF, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, -0.50),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(HALF, 0.0),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(0.50, 0.50, -0.50),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(0.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, -0.50),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(HALF, 0.0),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(0.50, -0.50, -0.50),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
        lighting: 0.0,
    }, // Front-bottom-right
    // Facing negative-Z
    Vertex3D {
        position: Vector3::new(0.50, -0.50, 0.50),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(1.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, -0.50, 0.50),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(HALF, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-left
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, 0.50),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(0.50, -0.50, 0.50),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(1.0, THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(-0.50, 0.50, 0.50),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(HALF, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(0.50, 0.50, 0.50),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(1.0, 2.0 * THIRD),
        vtype: 0,
        lighting: 0.0,
    }, // Back-top-right
];

pub(crate) struct Skybox;

impl GLRenderable for Skybox {
    fn init_gl_resources(&self, gl_resources: &mut super::resources::GLResources) {
        let skybox_texture = Texture::from_dynamic_image_bytes(SKYBOX_BITMAP, ImageFormat::Png);
        let skybox_program = Shader::new(SKYBOX_VERT_SRC, SKYBOX_FRAG_SRC).unwrap();
        let skybox_vertices: Box<Vec<Vertex3D>> = Box::new(SKYBOX_VERTS.into());

        gl_resources.add_texture("skybox", skybox_texture);
        gl_resources.add_shader("skybox", skybox_program);
        gl_resources.create_or_update_vao("skybox".to_string(), skybox_vertices);
    }

    fn draw(
        &self,
        gl_resources: &super::resources::GLResources,
        uniforms: &[(&str, Box<dyn super::uniform::Uniform>)],
    ) {
        let shader = gl_resources.get_shader("skybox").unwrap();
        shader.use_program();
        for (name, uniform) in uniforms {
            uniform.set_as_uniform(shader, name);
        }

        let skybox_texture = gl_resources.get_texture("skybox").unwrap();
        skybox_texture.use_as_framebuffer_texture(0);
        shader.set_texture(unsafe { c_str!("skybox_texture") }, 0);

        if let Some(vao) = gl_resources.get_vao("skybox") {
            vao.draw();
        }
    }
}
