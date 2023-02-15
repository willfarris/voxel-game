use cgmath::{Vector3, Vector2};

use super::{vertex::Vertex3D, resources::GLRenderable};

pub(crate) const SKYBOX_VERTS: [Vertex3D; 36] = [
    // Facing positive-X
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 1.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Front-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 1.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 1.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 0.0),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Back-top-right
    // Facing negative-X
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 1.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 0.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 1.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 1.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 0.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 0.0),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-bottom-left
    
    // Facing positive-Y
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 1.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-top-right
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 1.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 1.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-top-right
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Back-top-left
    // Facing negative-Y
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 1.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Front-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 1.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 0.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 1.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 0.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Back-bottom-left
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 0.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-bottom-right
    
    // Facing positive-Z
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Front-top-left
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Front-top-right
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Front-bottom-left
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Front-bottom-right
    // Facing negative-Z
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 1.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 0.0, 1.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(0.0, 0.0),
        vtype: 0,
    }, // Back-bottom-left
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 1.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(1.0, 0.0, 1.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(1.0, 0.0),
        vtype: 0,
    }, // Back-bottom-right
    Vertex3D {
        position: Vector3::new(0.0, 1.0, 1.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(0.0, 1.0),
        vtype: 0,
    }, // Back-top-left
    Vertex3D {
        position: Vector3::new(1.0, 1.0, 1.0),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coords: Vector2::new(1.0, 1.0),
        vtype: 0,
    }, // Back-top-right
];

pub(crate) struct Skybox;

impl GLRenderable for Skybox {
    fn init_gl_resources(&self, gl_resources: &mut super::resources::GLResources) {
        let verts: Box<Vec<Vertex3D>> = Box::new(SKYBOX_VERTS.into());
        gl_resources.update_vao_buffer("skybox".to_string(), verts);
    }

    fn draw(
        &self,
        gl_resources: &super::resources::GLResources,
        uniforms: &[(&str, Box<dyn super::uniform::Uniform>)],
    ) {
        todo!()
    }
}