use cgmath::{Vector2, Vector3};

use crate::graphics::mesh::block_drop_vertices;
use crate::graphics::skybox::Skybox;
use crate::item::drop::ItemDrop;
use crate::physics::collision::{check_world_collision_axis, Collider};
use crate::physics::physics_update::PhysicsUpdate;
use crate::physics::vectormath::{self, Vec3Direction};
use crate::terrain::block::BLOCKS;
use crate::terrain::generation::TerrainGenConfig;
use crate::terrain::ChunkIndex;
use crate::{entity::EntityTrait, player::Player, terrain::Terrain};
use crate::{graphics::resources::GLResources, physics::vectormath::Z_VECTOR};
use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

mod graphics;
mod save;
mod workers;

#[derive(PartialEq, Eq, Debug)]
enum PlayState {
    Running,
    Paused,
}

#[derive(Debug)]
pub enum PlayerInput {
    Look(f32, f32),
    Walk(f32, f32, f32),
    Inventory(usize),
    Interact(bool, bool),
    Jump,
    Stop,
}

pub struct Engine {
    player: Arc<RwLock<Box<Player>>>,
    terrain: Arc<RwLock<Terrain>>,
    entities: Vec<Box<dyn EntityTrait>>,
    skybox: Skybox,

    elapsed_time: Duration,
    last_update: Instant,

    play_state: PlayState,
    input_queue: Vec<PlayerInput>,
    terrain_config: Arc<RwLock<TerrainGenConfig>>,

    width: i32,
    height: i32,
    render_distance: isize,
    gl_resources: Arc<RwLock<GLResources>>,
}

impl Default for Engine {
    fn default() -> Self {
        let player = Box::new(Player::new(Vector3::new(0.0, 64.0, 0.0), Z_VECTOR));
        let terrain = Terrain::new();
        let mut terrain_config = TerrainGenConfig::default();
        terrain_config.load_features(include_str!("../assets/features/world_features.json"));

        Self {
            player: Arc::new(RwLock::new(player)),
            terrain: Arc::new(RwLock::new(terrain)),
            entities: Vec::new(),
            skybox: Skybox,

            elapsed_time: Duration::ZERO,
            last_update: Instant::now(),

            play_state: PlayState::Paused,
            input_queue: Vec::new(),
            terrain_config: Arc::new(RwLock::new(terrain_config)),

            width: 0,
            height: 0,
            render_distance: 8,
            gl_resources: Arc::new(RwLock::new(GLResources::new())),
        }
    }
}

impl Engine {
    pub fn update(&mut self) {
        if self.play_state == PlayState::Running {
            let now = std::time::Instant::now();
            let delta_time = now - self.last_update;
            self.last_update = now;
            self.elapsed_time += delta_time;

            {
                let mut player = self.player.write().unwrap();
                let terrain = self.terrain.read().unwrap();

                player.update_physics(delta_time.as_secs_f32());

                let movement_delta = player.movement_delta();

                player.position.x += movement_delta.x;
                let overlap_x =
                    check_world_collision_axis(Vec3Direction::X, player.bounding_box(), &terrain);
                player.correct_position_axis(Vec3Direction::X, overlap_x);

                player.position.y += movement_delta.y;
                let overlap_y =
                    check_world_collision_axis(Vec3Direction::Y, player.bounding_box(), &terrain);
                player.correct_position_axis(Vec3Direction::Y, overlap_y);

                player.position.z += movement_delta.z;
                let overlap_z =
                    check_world_collision_axis(Vec3Direction::Z, player.bounding_box(), &terrain);
                player.correct_position_axis(Vec3Direction::Z, overlap_z);
            }

            for entity in &mut self.entities {
                let terrain = self.terrain.read().unwrap();
                entity.update_physics(delta_time.as_secs_f32());

                let movement_delta = entity.movement_delta();

                entity.translate_relative(Vector3::new(movement_delta.x, 0.0, 0.0));
                let overlap_x =
                    check_world_collision_axis(Vec3Direction::X, entity.bounding_box(), &terrain);
                entity.correct_position_axis(Vec3Direction::X, overlap_x);

                entity.translate_relative(Vector3::new(0.0, movement_delta.y, 0.0));
                let overlap_y =
                    check_world_collision_axis(Vec3Direction::Y, entity.bounding_box(), &terrain);
                entity.correct_position_axis(Vec3Direction::Y, overlap_y);

                entity.translate_relative(Vector3::new(0.0, 0.0, movement_delta.z));
                let overlap_z =
                    check_world_collision_axis(Vec3Direction::Z, entity.bounding_box(), &terrain);
                entity.correct_position_axis(Vec3Direction::Z, overlap_z);
            }

            self.elapsed_time += delta_time;

            {
                let mut player = self.player.write().unwrap();
                let mut terrain = self.terrain.write().unwrap();
                let mut gl_resources = self.gl_resources.write().unwrap();

                let player_chunk_index = ChunkIndex::new(
                    player.position.x as isize / 16,
                    player.position.z as isize / 16,
                );
                terrain.update_visible_chunks_near(self.render_distance, &player_chunk_index);

                while !self.input_queue.is_empty() {
                    let input = self.input_queue.remove(0);
                    match input {
                        PlayerInput::Look(dx, dy) => {
                            player.look_direction(Vector2::new(dx, dy));
                        }
                        PlayerInput::Walk(dx, dy, dz) => {
                            player.move_direction(Vector3::new(dx, dy, dz));
                        }
                        PlayerInput::Jump => {
                            player.jump();
                        }
                        PlayerInput::Stop => {
                            player.stop_move();
                        }
                        PlayerInput::Inventory(selected) => {
                            player.select_inventory(selected);
                        }
                        PlayerInput::Interact(left_hand, right_hand) => {
                            if right_hand {
                                if let Some((_world_pos, world_index)) = vectormath::dda(
                                    &terrain,
                                    &player.camera.position,
                                    &player.camera.forward,
                                    6.0,
                                ) {
                                    let dropped = terrain.set_block(0, &world_index);
                                    if dropped != 0 {
                                        let drop_world_pos = Vector3::new(
                                            world_index.x as f32 + 0.5,
                                            world_index.y as f32 + 0.5,
                                            world_index.z as f32 + 0.5,
                                        );
                                        let new_drop =
                                            Box::new(ItemDrop::new(dropped, drop_world_pos));

                                        let verts = Box::new(block_drop_vertices(
                                            &BLOCKS[new_drop.block_id],
                                        ));
                                        let name = format!("item_{}", new_drop.block_id);

                                        gl_resources.update_vao_buffer(name, verts);
                                        self.entities.push(new_drop);
                                    }
                                }
                            }
                            if left_hand {
                                if let Some((world_pos, world_index)) = vectormath::dda(
                                    &terrain,
                                    &player.camera.position,
                                    &player.camera.forward,
                                    6.0,
                                ) {
                                    let mut diff = Vector3::new(
                                        world_pos.x - world_index.x as f32,
                                        world_pos.y - world_index.y as f32,
                                        world_pos.z - world_index.z as f32,
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
                                    terrain.set_block(1, &(world_index + offset));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn pause(&mut self) {
        self.play_state = PlayState::Paused;
        #[cfg(feature = "android-lib")]
        {
            debug!("Paused");
        }
    }

    pub fn resume(&mut self) {
        self.play_state = PlayState::Running;
        self.last_update = Instant::now();
        #[cfg(feature = "android-lib")]
        {
            debug!("Running");
        }
    }

    pub fn is_paused(&self) -> bool {
        self.play_state == PlayState::Paused
    }

    pub fn player_input(&mut self, movement: PlayerInput) {
        if self.play_state == PlayState::Running {
            self.input_queue.push(movement);
        }
    }
}
