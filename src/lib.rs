
mod graphics;
mod macros;
mod world;
mod player;
mod physics;
mod item;
mod entity;

#[cfg(feature = "android-lib")]
#[macro_use] extern crate log;
#[cfg(feature = "android-lib")]
extern crate android_log;
#[cfg(feature = "android-lib")]
extern crate jni;
#[cfg(feature = "android-lib")]
mod java_interface;

use cgmath::Vector3;
use entity::EntityTrait;
use physics::{vectormath::{Z_VECTOR, Vec3Direction, self}, collision::{Collider, check_world_collision_axis, check_collision_axis}, physics_update::PhysicsUpdate};
use player::{Player, camera::perspective_matrix};
use world::{World, block::BLOCKS};
use graphics::{resources::{GLRenderable, GLResources}, mesh::block_drop_vertices};

pub use physics::vectormath::q_rsqrt;

#[derive(PartialEq, Eq, Debug)]
enum PlayState {
    Running,
    Paused,
}

#[derive(Debug)]
pub enum PlayerMovement {
    Look(f32, f32),
    Walk(f32, f32, f32),
    Inventory(usize),
    Interact(bool, bool),
    Jump,
    Stop,
}

pub struct Engine {
    player: Option<Box<Player>>,
    world: Option<World>,
    entities: Vec<Box<dyn EntityTrait>>,

    elapsed_time: f32,
    play_state: PlayState,

    width: i32,
    height: i32,
    gl_resources: GLResources,
}

impl Engine {
    pub fn new() -> Self {
        let player = Some(Box::new(Player::new(Vector3::new(0.0, 30.0, 0.0), Z_VECTOR)));
        let entities = Vec::new();
        let mut world = Some(World::new());
        if let Some(w) = world.as_mut() {
            w.gen_terrain(2, 69);
        }

        Self {
            player,
            world,
            entities,

            elapsed_time: 0.0,
            play_state: PlayState::Running,

            width: 0,
            height: 0,
            gl_resources: GLResources::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.play_state == PlayState::Running {

            let world = self.world.as_ref().unwrap();

            if let Some(player) = &mut self.player{
                player.update_physics(delta_time);

                let movement_delta = player.movement_delta();

                player.position.x += movement_delta.x;
                let overlap_x = check_world_collision_axis(Vec3Direction::X, player.bounding_box(), world);
                player.correct_position_axis(Vec3Direction::X, overlap_x);

                player.position.y += movement_delta.y;
                let overlap_y = check_world_collision_axis(Vec3Direction::Y, player.bounding_box(), world);
                player.correct_position_axis(Vec3Direction::Y, overlap_y);
                
                player.position.z += movement_delta.z;
                let overlap_z = check_world_collision_axis(Vec3Direction::Z, player.bounding_box(), world);
                player.correct_position_axis(Vec3Direction::Z, overlap_z);
            }

            for entity in &mut self.entities {
                entity.update_physics(delta_time);

                let movement_delta = entity.movement_delta();

                entity.translate_relative(Vector3::new(movement_delta.x, 0.0, 0.0));
                let overlap_x = check_world_collision_axis(Vec3Direction::X, entity.bounding_box(), world);
                entity.correct_position_axis(Vec3Direction::X, overlap_x);

                entity.translate_relative(Vector3::new(0.0, movement_delta.y, 0.0));
                let overlap_y = check_world_collision_axis(Vec3Direction::Y, entity.bounding_box(), world);
                entity.correct_position_axis(Vec3Direction::Y, overlap_y);
                
                entity.translate_relative(Vector3::new(0.0, 0.0, movement_delta.z));
                let overlap_z = check_world_collision_axis(Vec3Direction::Z, entity.bounding_box(), world);
                entity.correct_position_axis(Vec3Direction::Z, overlap_z);
            }

            self.elapsed_time += delta_time;
        }
    }

    pub fn init_gl(&mut self, width: i32, height: i32) {

        #[cfg(target_os = "android")] {
            gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
            debug!("Loaded GL pointer");
        }

        self.width = width;
        self.height = height;
        let mut framebuffer_id = 0;
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            
            gl::FrontFace(gl::CW);
    
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut framebuffer_id);
        }

        if let Some(world) = &mut self.world {
            world.init_gl_resources(&mut self.gl_resources);
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
    }

    pub fn reset_gl_resources(&mut self) {
        self.gl_resources.invalidate_resources();
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::Viewport(0, 0, self.width, self.height);
            gl::ClearColor(0.2, 0.2, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.gl_resources.process_buffer_updates();

        if let Some(player) = &self.player {
            let perspective_matrix = perspective_matrix(self.width, self.height);
            let view_matrix = player.camera_view_matrix();

            if let Some(world) = &mut self.world {
                world.draw(&mut self.gl_resources, perspective_matrix, view_matrix, self.elapsed_time);
            }

            for entity in &self.entities {
                entity.draw(&mut self.gl_resources, perspective_matrix, view_matrix, self.elapsed_time);
            }
        }
    }

    pub fn pause(&mut self) {
        self.play_state = PlayState::Paused;
        #[cfg(feature = "android-lib")] {
            debug!("Paused");
        }
    }

    pub fn resume(&mut self) {
        self.play_state = PlayState::Running;
        #[cfg(feature = "android-lib")] {
            debug!("Running");
        }
    }

    pub fn is_paused(&self) -> bool {
        self.play_state == PlayState::Paused
    }

    pub fn player_movement(&mut self, movement: PlayerMovement) {
        if self.play_state == PlayState::Running {
            if let Some(player) = &mut self.player {
                match movement {
                    PlayerMovement::Look(dx, dy) => {
                        player.camera.rotate_on_x_axis(f32::from(dx));
                        player.camera.rotate_on_y_axis(f32::from(dy));
                    },
                    PlayerMovement::Walk(dx, dy, dz) => {
                        player.move_direction(Vector3::new(dx, dy, dz));
                    },
                    PlayerMovement::Jump => {
                        player.jump();
                    },
                    PlayerMovement::Stop => {
                        player.stop_move();
                    }
                    PlayerMovement::Inventory(selected) => {
                        player.select_inventory(selected);
                    },
                    PlayerMovement::Interact(left_hand, right_hand) => {
                        if right_hand {
                            if let Some((_world_pos, world_index)) = vectormath::dda(self.world.as_ref().unwrap(), &player.camera.position, &player.camera.forward, 6.0) {
                                if let Some(drop) = self.world.as_mut().unwrap().destroy_at_global_pos(&world_index, &mut self.gl_resources) {
                                    let boxed_drop = Box::new(drop);
                                    let verts = block_drop_vertices(&BLOCKS[boxed_drop.block_id]);
                                    let name = format!("item_{}", boxed_drop.block_id);
                                    self.gl_resources.update_buffer(name, verts);
                                    self.entities.push(boxed_drop);
                                }
                            }
                        }
                        if left_hand {
                            if let Some((_world_pos, world_index)) = vectormath::dda(&self.world.as_ref().unwrap(), &player.camera.position, &player.camera.forward, 6.0) {
                                let mut diff = Vector3::new(
                                    _world_pos.x - world_index.x as f32,
                                    _world_pos.y - world_index.y as f32,
                                    _world_pos.z - world_index.z as f32,
                                );

                                if diff.x == 0.0 {
                                    diff.x = -1.0;
                                } else if diff.x == 1.0 {
                                    diff.x = 1.0;
                                } else {
                                    diff.x = 0.0;
                                }

                                if diff.y == 0.0 {
                                    diff.y = -1.0;
                                } else if diff.y == 1.0 {
                                    diff.y = 1.0;
                                } else {
                                    diff.y = 0.0;
                                }

                                if diff.z == 0.0 {
                                    diff.z = -1.0;
                                } else if diff.z == 1.0 {
                                    diff.z = 1.0;
                                } else {
                                    diff.z = 0.0;
                                }

                                let offset = Vector3::new(
                                    diff.x as isize,
                                    diff.y as isize,
                                    diff.z as isize,
                                );
                                self.world.as_mut().unwrap().place_block(1, &(world_index + offset), &mut self.gl_resources);
                                
                            }
                        }
                    }
                }
            }
        }
    }


}
