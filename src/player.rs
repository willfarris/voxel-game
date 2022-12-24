pub(crate) mod camera;

use cgmath::{Vector3, InnerSpace, Matrix4};

use camera::Camera;

use super::world::World;
use crate::physics::collision::{self, Collider, Rect3};
use crate::physics::collision::rect_vs_rect;
use crate::physics::vectormath::{Y_VECTOR, q_rsqrt};

const GRAVITY: Vector3<f32> = Vector3 {x: 0.0, y: -9.81 * 2.0, z: 0.0};

pub(crate) struct Player {
    pub(crate) camera: Camera,
    pub position: Vector3<f32>,
    velocity: Vector3<f32>,
    acceleration: Vector3<f32>,

    move_speed: f32,
    grounded: bool,
    walking: bool,
    height: f32,

    collision_box: Rect3,

    //pub inventory: Inventory,
}

impl Player {
    pub fn new(position: Vector3<f32>, forward: Vector3<f32>) -> Self {
        Self {
            camera: Camera::new(position, forward),
            position,
            velocity: Vector3::new(0f32, 0f32, 0f32),
            acceleration: Vector3::new(0f32, 0f32, 0f32),
            move_speed: 3.0,
            grounded: false,
            walking: false,
            height: 1.6,

            collision_box: Rect3::new([-0.25, 0.0, -0.25].into(), [0.5, 1.6, 0.5].into()),

            //inventory: Inventory::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {

        if !self.grounded {
            self.acceleration.y = GRAVITY.y;
            //self.velocity.y += self.acceleration.y * delta_time;
        }

        if !self.walking {
            self.velocity.x *= 1.0 - 10.0 * delta_time;
            self.velocity.z *= 1.0 - 10.0 * delta_time;
        }

        self.velocity += self.acceleration * delta_time;

        let forward = Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z).normalize();
        let delta = delta_time * Vector3 {
            x: (self.move_speed * self.camera.right.x * self.velocity.x as f32) + (self.move_speed * forward.x * self.velocity.z as f32),
            y: self.velocity.y as f32,
            z: (self.move_speed * self.camera.right.z * self.velocity.x as f32) + (self.move_speed * forward.z * self.velocity.z as f32),
        };

        /*
        self.position.x += delta.x;
        let mut player_bounding_box = self.bounding_box();
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let x_overlap = if player_bounding_box.pos.x > block_bounding_box.pos.x {
                            (block_bounding_box.pos.x + 1.0) - player_bounding_box.pos.x 
                        } else {
                            -1.0 * (player_bounding_box.pos.x + player_bounding_box.size.x - block_bounding_box.pos.x)
                        };
                        self.position.x += x_overlap;
                        player_bounding_box.pos.x += x_overlap;
                    }
                }
            }
        }
        for i in 0..entities.len() {
            let x_overlap = self.check_overlap_x(&entities[i]);
            self.position.x += x_overlap;
            player_bounding_box.pos.x += x_overlap;
        }

        self.position.y += delta.y;
        player_bounding_box = self.bounding_box();
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let y_overlap = if player_bounding_box.pos.y > block_bounding_box.pos.y {
                            (block_bounding_box.pos.y + 1.0) - player_bounding_box.pos.y 
                        } else {
                            -1.0 * (player_bounding_box.pos.y + player_bounding_box.size.y - block_bounding_box.pos.y)
                        };

                        self.position.y += y_overlap;
                        player_bounding_box.pos.y += y_overlap;
                        if y_overlap.abs() > 0.0 {
                            self.velocity.y = 0f32;
                            if y_overlap > 0.0 {
                                self.grounded = true;
                            }
                        }
                    }
                }
            }
        }
        for i in 0..entities.len() {
            let y_overlap = self.check_overlap_y(&entities[i]);
            self.position.y += y_overlap;
            player_bounding_box.pos.y += y_overlap;
            if y_overlap.abs() > 0.0 {
                self.velocity.y = 0f32;
                if y_overlap > 0.0 {
                    self.grounded = true;
                }
            }
        }

        self.position.z += delta.z;
        player_bounding_box = self.bounding_box();
        for block_x in (self.position.x.floor() as isize - 1) ..= (self.position.x.floor() as isize + 1) {
            for block_y in (self.position.y.floor() as isize - 1) ..= (self.position.y.floor() as isize + 2) {
                for block_z in (self.position.z.floor() as isize - 1) ..= (self.position.z.floor() as isize + 1) {
                    if !BLOCKS[world.block_at_global_pos(Vector3::new(block_x, block_y, block_z))].solid {
                        continue;
                    }
                    let block_bounding_box = collision::Rect3 {
                        pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                        size: Vector3::new(1.0, 1.0, 1.0)
                    };
                    if rect_vs_rect(&player_bounding_box, &block_bounding_box) {
                        let z_overlap = if player_bounding_box.pos.z > block_bounding_box.pos.z {
                            (block_bounding_box.pos.z + 1.0) - player_bounding_box.pos.z 
                        } else {
                            -1.0 * (player_bounding_box.pos.z + player_bounding_box.size.z - block_bounding_box.pos.z)
                        };
                        self.position.z += z_overlap;
                        player_bounding_box.pos.z += z_overlap;
                    }
                }
            }
        }
        for i in 0..entities.len() {
            let z_overlap = self.check_overlap_z(&entities[i]);
            self.position.z += z_overlap;
            player_bounding_box.pos.z += z_overlap;
        }
        */
        self.camera.translate(self.position + self.height * Y_VECTOR);
    }

    pub fn move_direction(&mut self, direction: Vector3<f32>) {
        self.walking = true;
        self.velocity.x += direction.x;
        self.velocity.z += direction.z;
        self.velocity.x *= q_rsqrt(self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z);
        self.velocity.z *= q_rsqrt(self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z);
    }

    pub fn jump(&mut self) {
        if self.grounded {
            self.velocity.y += 7f32;
            self.grounded = false;
        }
    }

    pub fn stop_move(&mut self) {
        self.walking = false;
    }

    pub fn camera_view_matrix(&self) -> Matrix4<f32> {
        self.camera.view_matrix()
    }
}

impl Collider for Player {
    /*fn check_collision(&mut self, delta: Vector3<f32>, other: &impl Collider) -> Vector3<f32> {
        let mut overlap: Vector3<f32> = [0.0, 0.0, 0.0].into();

        let other_bounding_box = other.bounding_box();
        let mut self_bounding_box = self.bounding_box();

        
        self_bounding_box.pos.x += delta.x;
        if rect_vs_rect(&self_bounding_box, &other_bounding_box) {
            let x_overlap = if self_bounding_box.pos.x > other_bounding_box.pos.x {
                (other_bounding_box.pos.x + 1.0) - self_bounding_box.pos.x 
            } else {
                -1.0 * (self_bounding_box.pos.x + self_bounding_box.size.x - other_bounding_box.pos.x)
            };
            self_bounding_box.pos.x += x_overlap;
            overlap.x += x_overlap;
        }

        /*
        self_bounding_box.pos.y += delta.y;
        if rect_vs_rect(&self_bounding_box, &other_bounding_box) {
            let y_overlap = if self_bounding_box.pos.y > other_bounding_box.pos.y {
                (other_bounding_box.pos.y + 1.0) - self_bounding_box.pos.y 
            } else {
                -1.0 * (self_bounding_box.pos.y + self_bounding_box.size.y - other_bounding_box.pos.y)
            };
            self_bounding_box.pos.y += y_overlap;
            overlap.y += y_overlap;
        }
        */

        self_bounding_box.pos.z += delta.z;
        if rect_vs_rect(&self_bounding_box, &other_bounding_box) {
            let z_overlap = if self_bounding_box.pos.z > other_bounding_box.pos.z {
                (other_bounding_box.pos.z + 1.0) - self_bounding_box.pos.z
            } else {
                -1.0 * (self_bounding_box.pos.z + self_bounding_box.size.z - other_bounding_box.pos.z)
            };
            self_bounding_box.pos.z += z_overlap;
            overlap.z += z_overlap;
        }

        overlap
    }*/

    fn bounding_box(&self) -> collision::Rect3 {
        let mut bounding_box = self.collision_box.clone();
        bounding_box.pos += self.position;
        bounding_box
    }
}