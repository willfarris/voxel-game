
mod graphics;
mod macros;
mod mesh_object;
mod world;
mod player;
mod physics;

#[cfg(feature = "android-lib")]
#[macro_use] extern crate log;
#[cfg(feature = "android-lib")]
extern crate android_log;
#[cfg(feature = "android-lib")]
extern crate jni;
#[cfg(feature = "android-lib")]
mod java_interface;

use cgmath::Vector3;
use physics::{vectormath::{Z_VECTOR, Vec3Direction, self}, collision::{Collider, check_world_collision_axis}};
use player::{Player, camera::perspective_matrix};
use world::World;
use graphics::resources::GLRenderable;
use mesh_object::MeshObject;

pub use physics::vectormath::q_rsqrt;

use crate::world::block::BLOCKS;

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
    player: Option<Player>,
    entities: Vec<MeshObject>,
    world: Option<World>,
    color: (f32, f32, f32),
    elapsed_time: f32,
    play_state: PlayState,

    width: i32,
    height: i32,
}

impl Engine {
    pub fn new() -> Self {
        let player = Some(Player::new(Vector3::new(0.0, 30.0, 0.0), Z_VECTOR));

        let mut entities = Vec::new();
        let test_mesh = MeshObject::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.1, 0.1, 0.1),
            mesh_object::DEFAULT_CUBE.to_vec(),
            include_str!("../shaders/cube.vert"),
            include_str!("../shaders/cube.frag"),
            include_bytes!("../assets/cube_test.png"),
        );
        entities.push(test_mesh);

        let world_texture_bitmap = include_bytes!("../assets/terrain.png");
        let mut world = Some(World::new(
            include_str!("../shaders/cube.vert"),
            include_str!("../shaders/cube.frag"),
            world_texture_bitmap,
        ));
        
        if let Some(w) = world.as_mut() {
            w.gen_terrain(5, 69);
        }

        Self {
            player,
            entities,
            world,
            color: (0.05, 0.15, 0.35),
            elapsed_time: 0.0,
            play_state: PlayState::Running,

            width: 0,
            height: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.play_state == PlayState::Running {
            /*for i in 0..self.entities.len() {
                let entity = &mut self.entities[i];
                entity.update(delta_time);
            }*/

            if let Some(player) = &mut self.player{
                let world = self.world.as_ref().unwrap();
                player.update_physics(delta_time);

                let movement_delta = player.movement_delta();

                player.position.x += movement_delta.x;
                let overlap_x = check_world_collision_axis(Vec3Direction::X, player, world);
                player.correct_position_axis(Vec3Direction::X, overlap_x);

                player.position.y += movement_delta.y;
                let overlap_y = check_world_collision_axis(Vec3Direction::Y, player, world);
                player.correct_position_axis(Vec3Direction::Y, overlap_y);
                
                player.position.z += movement_delta.z;
                let overlap_z = check_world_collision_axis(Vec3Direction::Z, player, world);
                player.correct_position_axis(Vec3Direction::Z, overlap_z);

                //println!("Player at: ({}, {}, {})", player.position.x, player.position.y, player.position.z);
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

        for entity in &mut self.entities {
            entity.init_gl_resources();
        }

        if let Some(world) = &mut self.world {
            world.build_all_chunk_mesh(5, Vector3::new(0.0, 0.0, 0.0));
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
    }

    pub fn draw(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width, self.height);
            gl::ClearColor(self.color.0, self.color.1, self.color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if let Some(player) = &self.player {
            let perspective_matrix = perspective_matrix(self.width, self.height);
            let view_matrix = player.camera_view_matrix();

            for entity in &self.entities {
                entity.draw(perspective_matrix, view_matrix, self.elapsed_time);
            }

            if let Some(world) = &self.world {
                world.draw(perspective_matrix, view_matrix, self.elapsed_time);
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

    pub fn set_color(&mut self, red: f32, green: f32, blue: f32) {
        self.color = (red, green, blue);
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
                            if let Some((world_pos, world_index)) = vectormath::dda(self.world.as_ref().unwrap(), &player.camera.position, &player.camera.forward, 6.0) {
                                let block_id = self.world.as_ref().unwrap().block_at_world_pos(&world_index);
                                player.add_to_inventory(block_id);
                                println!("{:?}, {}", world_index, BLOCKS[block_id].name);
                                self.entities[0].set_position(world_pos);
                                //self.world.unwrap().destroy_at_global_pos(world_index);
                            }
                        }
                        if left_hand {
                            /*if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                                self.terrain.interact_at_global_pos(world_index);
                            }*/
                        }
                    }
                }
            }
        }
    }


}
