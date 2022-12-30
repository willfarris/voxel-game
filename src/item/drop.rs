use cgmath::Zero;
pub(crate) use cgmath::{Vector3, Matrix4, Quaternion, Rotation3, Deg};

use crate::{graphics::{resources::{GLRenderable, GLResources}, texture::Texture, buffer::BufferObject, mesh::block_drop_vertices, source::{TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC, TERRAIN_BITMAP}}, c_str, physics::{collision::{Rect3, Collider}, vectormath::Vec3Direction, physics_update::PhysicsUpdate}, player::GRAVITY, world::block::BLOCKS, EntityTrait};

pub struct ItemDrop {
    // Persists across OpenGL context creation
    pub block_id: usize,
    pub position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,

    // Physics properties
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>,
    movement_delta: Vector3<f32>,
    collider: Rect3,
    grounded: bool,
}

impl ItemDrop {
    pub fn new(
        block_id: usize,
        position: Vector3<f32>,
    ) -> ItemDrop {
        ItemDrop {
            block_id,
            position,
            rotation: Vector3::zero(),
            scale: Vector3::new(0.25, 0.25, 0.25),

            collider: Rect3::new(Vector3::zero(), Vector3::new(1.0, 1.0, 1.0)),
            velocity: Vector3::new(0.0, 3.0, 0.0),
            acceleration: Vector3::zero(),
            movement_delta: Vector3::zero(),
            grounded: false,
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

impl GLRenderable for ItemDrop {
    fn init_gl_resources(&self, gl_resources: &mut GLResources) {
        if gl_resources.get_shader("terrain").is_none() {
            gl_resources.create_shader("terrain", TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC);
        }
        if gl_resources.get_texture("terrain").is_none() {
            gl_resources.create_texture("terrain", TERRAIN_BITMAP);
        }

        let verts = block_drop_vertices(&BLOCKS[self.block_id]);
        let name = format!("item_{}", self.block_id);
        gl_resources.update_buffer(name, verts);
    }

    fn draw(&self, gl_resources: &mut GLResources, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, elapsed_time: f32) {
        let scale_matrix = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation = Quaternion::from_angle_x(Deg(self.rotation.x)) * Quaternion::from_angle_y(Deg(self.rotation.y)) * Quaternion::from_angle_z(Deg(self.rotation.z));
        let rotation_matrix = Matrix4::from(rotation);
        let translation_matrix = Matrix4::from_translation(self.position);
        let model_matrix = translation_matrix * rotation_matrix * scale_matrix;

        let shader = gl_resources.get_shader("terrain").unwrap();
        let texture = gl_resources.get_texture("terrain").unwrap();

        texture.bind();

        shader.use_program();
        shader.set_mat4(unsafe {c_str!("perspective_matrix")}, &perspective_matrix);
        shader.set_mat4(unsafe {c_str!("view_matrix")}, &view_matrix);
        shader.set_mat4(unsafe {c_str!("model_matrix")}, &model_matrix);
        shader.set_float(unsafe {c_str!("time")}, elapsed_time);
        shader.set_texture(unsafe {c_str!("texture_map")}, 0);

        let name = format!("item_{}", self.block_id);
        if let Some(vbo) = gl_resources.get_buffer(name) {
            vbo.draw_vertex_buffer();
        }

    }
}

impl Collider for ItemDrop {
    fn bounding_box(&self) -> Rect3 {
        let mut col_corrected = self.collider.clone();
        col_corrected.pos += self.position;
        col_corrected.size.x *= self.scale.x;
        col_corrected.size.y *= self.scale.y;
        col_corrected.size.z *= self.scale.z;
        col_corrected
    }

    fn movement_delta(&self) -> Vector3<f32> {
        self.movement_delta
    }

    fn correct_position_axis(&mut self, axis: Vec3Direction, overlap: f32) {
        match axis {
            Vec3Direction::X => {
                self.position.x += overlap;
            },
            Vec3Direction::Y => {
                self.position.y += overlap;
                if overlap.abs() > 0.0 {
                    self.velocity.y = 0f32;
                    if overlap > 0.0 {
                        self.grounded = true;
                    }
                }
            },
            Vec3Direction::Z => {
                self.position.z += overlap;
            },
        }
    }

    fn has_collider(&self) -> bool {
        true
    }
}

impl PhysicsUpdate for ItemDrop {
    fn update_physics(&mut self, delta_time: f32) {
        if !self.grounded {
            self.acceleration.y = GRAVITY.y;
        }
        self.velocity += self.acceleration * delta_time;

        self.movement_delta = delta_time * Vector3 {
            x: 0.0,
            y: self.velocity.y as f32,
            z: 0.0,
        };
    }

    fn translate_relative(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }
}

impl EntityTrait for ItemDrop {}