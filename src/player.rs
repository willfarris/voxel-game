pub(crate) mod camera;
mod inventory;

use cgmath::{InnerSpace, Matrix4, Vector2, Vector3};

use camera::Camera;

use crate::engine::EngineEvent;
use crate::physics::collision::{self, Collider, Rect3};
use crate::physics::physics_update::PhysicsUpdate;
use crate::physics::vectormath::{q_rsqrt, Vec3Direction, Y_VECTOR};

use self::inventory::Inventory;

const GRAVITY_MODIFIER: f32 = 1.0;

pub(crate) const GRAVITY: Vector3<f32> = Vector3 {
    x: 0.0,
    y: -9.81 * 2.0,
    z: 0.0,
};

#[derive(Debug)]
pub enum PlayerInput {
    Look(f32, f32),
    Walk(f32, f32, f32),
    Inventory(usize),
    Interact(bool, bool),
    Jump,
    Sprint,
    Stop,
}

pub(crate) struct Player {
    pub(crate) camera: Camera,
    pub position: Vector3<f32>,
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>,
    movement_delta: Vector3<f32>,

    move_speed: f32,
    speed_multiplier: f32,
    pub grounded: bool,
    walking: bool,
    time_walking: f32,
    running: bool,
    height: f32,

    collision_box: Rect3,

    inventory: Inventory,
}

impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,

            velocity: Vector3::new(0f32, 0f32, 0f32),
            acceleration: Vector3::new(0f32, 0f32, 0f32),
            movement_delta: Vector3::new(0f32, 0f32, 0f32),

            move_speed: 4.0,
            speed_multiplier: 2.0,
            running: false,
            grounded: false,
            walking: false,
            time_walking: 0.0,
            height: 1.6,

            collision_box: Rect3::new([-0.25, 0.0, -0.25].into(), [0.5, 1.6, 0.5].into()),

            inventory: Inventory::new(),
        }
    }

    pub fn input(&mut self, input: PlayerInput) {
        match input {
            PlayerInput::Look(dx, dy) => {
                self.look_direction(Vector2::new(dx, dy));
            }
            PlayerInput::Walk(dx, dy, dz) => {
                self.move_direction(Vector3::new(dx, dy, dz));
            }
            PlayerInput::Jump => {
                self.jump();
            }
            PlayerInput::Stop => {
                self.stop_move();
            }
            PlayerInput::Inventory(selected) => {
                self.select_inventory(selected);
            }
            PlayerInput::Sprint => {
                self.running = true;
                self.speed_multiplier = 2.0;
            }
            _ => {}
        }
    }

    pub fn camera_pos_and_dir(&self) -> (Vector3<f32>, Vector3<f32>) {
        (self.camera.position, self.camera.forward)
    }

    fn move_direction(&mut self, direction: Vector3<f32>) {
        self.walking = true;
        self.velocity.x += direction.x;
        self.velocity.z += direction.z;
        self.velocity.x *=
            q_rsqrt(self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z);
        self.velocity.z *=
            q_rsqrt(self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z);
    }

    fn look_direction(&mut self, direction: Vector2<f32>) {
        self.camera.rotate_on_x_axis(direction.x);
        self.camera.rotate_on_y_axis(direction.y);
    }

    fn jump(&mut self) {
        if self.grounded {
            self.velocity.y += 7f32;
            self.grounded = false;
        }
    }

    pub fn stop_move(&mut self) {
        self.walking = false;
        self.time_walking %= 2.0 * std::f32::consts::PI / 10.0;
        self.running = false;
        self.speed_multiplier = 1.0;
    }

    pub fn camera_view_matrix(&self) -> Matrix4<f32> {
        self.camera.view_matrix()
    }

    pub fn select_inventory(&mut self, selected: usize) {
        self.inventory.set_selected(selected);
        self.inventory.print_inventory();
    }

    pub fn _add_to_inventory(&mut self, block_id: usize) {
        self.inventory.add_to_inventory(block_id);
    }
}

impl PhysicsUpdate for Player {
    fn update_physics(&mut self, delta_time: f32) {
        self.camera
            .translate(self.position + self.height * Y_VECTOR);
        if !self.grounded {
            self.acceleration.y = GRAVITY.y * GRAVITY_MODIFIER;
        }

        if !self.walking {
            self.velocity.x *= 1.0 - 10.0 * delta_time;
            self.velocity.z *= 1.0 - 10.0 * delta_time;
            //self.time_walking *= 1.0 - 10.0 * delta_time;
        } else {
            self.time_walking += delta_time;
        }

        self.camera.position += self.camera.up * 0.03 * (10.0 * self.time_walking).sin();

        self.velocity += self.acceleration * delta_time;

        let forward = Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z).normalize();
        let move_speed = if self.running {
            self.move_speed * self.speed_multiplier
        } else {
            self.move_speed
        };
        let delta = delta_time
            * Vector3 {
                x: (move_speed * self.camera.right.x * self.velocity.x)
                    + (move_speed * forward.x * self.velocity.z),
                y: self.velocity.y,
                z: (move_speed * self.camera.right.z * self.velocity.x)
                    + (move_speed * forward.z * self.velocity.z),
            };
        self.movement_delta = delta;
    }

    fn translate_relative(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }
}

impl Collider for Player {
    fn bounding_box(&self) -> collision::Rect3 {
        let mut bounding_box = self.collision_box.clone();
        bounding_box.pos += self.position;
        bounding_box
    }

    fn movement_delta(&self) -> Vector3<f32> {
        self.movement_delta
    }

    fn correct_position_axis(&mut self, axis: Vec3Direction, overlap: f32) {
        match axis {
            Vec3Direction::X => {
                self.position.x += overlap;
            }
            Vec3Direction::Y => {
                self.position.y += overlap;
                if overlap.abs() > 0.0 {
                    self.velocity.y = 0f32;
                    if overlap > 0.0 {
                        self.grounded = true;
                    }
                }
            }
            Vec3Direction::Z => {
                self.position.z += overlap;
            }
        }
    }

    fn has_collider(&self) -> bool {
        true
    }
}
