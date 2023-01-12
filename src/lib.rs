
mod graphics;
mod macros;
mod terrain;
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

use std::{sync::{Mutex, Arc, RwLock}, time::{Instant, Duration}};

use cgmath::{Vector3, Zero, Vector2};
use entity::EntityTrait;
use noise::Perlin;
use physics::{vectormath::{Z_VECTOR, Vec3Direction, self}, collision::{Collider, check_world_collision_axis, check_collision_axis}, physics_update::PhysicsUpdate};
use player::{Player, camera::perspective_matrix};
use terrain::{Terrain, block::BLOCKS, chunk::{Chunk, CHUNK_WIDTH}, ChunkIndex, generation::NoiseConfig, BlockWorldPos};
use graphics::{resources::{GLRenderable, GLResources}, mesh::block_drop_vertices};

pub use physics::vectormath::q_rsqrt;

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

pub struct EngineLock {
    engine: Mutex<Engine>,
}

impl Default for EngineLock {
    fn default() -> Self {
        Self {
            engine: Mutex::new(Engine::default()),
        }
    }
}

pub struct Engine {
    player: Arc<RwLock<Box<Player>>>,
    terrain: Arc<RwLock<Terrain>>,
    entities: Vec<Box<dyn EntityTrait>>,

    elapsed_time: f32,
    play_state: PlayState,
    input_queue: Vec<PlayerInput>,
    noise_config: Arc<RwLock<NoiseConfig>>,

    width: i32,
    height: i32,
    gl_resources: Arc<RwLock<GLResources>>,
}

impl Default for Engine {
    fn default() -> Self {
        let player = Box::new(Player::new(Vector3::new(0.0, 30.0, 0.0), Z_VECTOR));
        let terrain = Terrain::new();
        
        let noise_scale = 0.02;
        let noise_offset = Vector2::new(
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
        );
        let perlin = Perlin::new();
        let noise_config = NoiseConfig {
            perlin,
            noise_scale,
            noise_offset,
        };

        Self {
            player: Arc::new(RwLock::new(player)),
            terrain: Arc::new(RwLock::new(terrain)),
            entities: Vec::new(),

            elapsed_time: 0.0,
            play_state: PlayState::Paused,
            input_queue: Vec::new(),
            noise_config: Arc::new(RwLock::new(noise_config)),

            width: 0,
            height: 0,
            gl_resources: Arc::new(RwLock::new(GLResources::new())),
        }
    }
}

impl Engine {
    pub fn update(&mut self, delta_time: f32) {
        if self.play_state == PlayState::Running {

            {
                let mut player = self.player.write().unwrap();
                let terrain = self.terrain.read().unwrap();

                player.update_physics(delta_time);

                let movement_delta = player.movement_delta();

                player.position.x += movement_delta.x;
                let overlap_x = check_world_collision_axis(Vec3Direction::X, player.bounding_box(), &terrain);
                player.correct_position_axis(Vec3Direction::X, overlap_x);

                player.position.y += movement_delta.y;
                let overlap_y = check_world_collision_axis(Vec3Direction::Y, player.bounding_box(), &terrain);
                player.correct_position_axis(Vec3Direction::Y, overlap_y);
                
                player.position.z += movement_delta.z;
                let overlap_z = check_world_collision_axis(Vec3Direction::Z, player.bounding_box(), &terrain);
                player.correct_position_axis(Vec3Direction::Z, overlap_z);
            }

            for entity in &mut self.entities {
                let terrain = self.terrain.read().unwrap();
                entity.update_physics(delta_time);

                let movement_delta = entity.movement_delta();

                entity.translate_relative(Vector3::new(movement_delta.x, 0.0, 0.0));
                let overlap_x = check_world_collision_axis(Vec3Direction::X, entity.bounding_box(), &terrain);
                entity.correct_position_axis(Vec3Direction::X, overlap_x);

                entity.translate_relative(Vector3::new(0.0, movement_delta.y, 0.0));
                let overlap_y = check_world_collision_axis(Vec3Direction::Y, entity.bounding_box(), &terrain);
                entity.correct_position_axis(Vec3Direction::Y, overlap_y);
                
                entity.translate_relative(Vector3::new(0.0, 0.0, movement_delta.z));
                let overlap_z = check_world_collision_axis(Vec3Direction::Z, entity.bounding_box(), &terrain);
                entity.correct_position_axis(Vec3Direction::Z, overlap_z);
            }

            self.elapsed_time += delta_time;
            
            {
                let mut player = self.player.write().unwrap();
                let mut terrain = self.terrain.write().unwrap();
                let mut gl_resources = self.gl_resources.write().unwrap();

                while !self.input_queue.is_empty() {
                    let input = self.input_queue.remove(0);
                    match input {
                        PlayerInput::Look(dx, dy) => {
                            player.camera.rotate_on_x_axis(dx);
                            player.camera.rotate_on_y_axis(dy);
                        },
                        PlayerInput::Walk(dx, dy, dz) => {
                            player.move_direction(Vector3::new(dx, dy, dz));
                        },
                        PlayerInput::Jump => {
                            player.jump();
                        },
                        PlayerInput::Stop => {
                            player.stop_move();
                        }
                        PlayerInput::Inventory(selected) => {
                            player.select_inventory(selected);
                        },
                        PlayerInput::Interact(left_hand, right_hand) => {
                            if right_hand {
                                if let Some((_world_pos, world_index)) = vectormath::dda(&terrain, &player.camera.position, &player.camera.forward, 6.0) {
                                    if let Some(drop) = terrain.destroy_at_global_pos(&world_index, &mut gl_resources) {
                                        let boxed_drop = Box::new(drop);
                                        let verts = block_drop_vertices(&BLOCKS[boxed_drop.block_id]);
                                        let name = format!("item_{}", boxed_drop.block_id);
                                        gl_resources.update_buffer(name, verts);
                                        self.entities.push(boxed_drop);
                                    }
                                }
                            }
                            if left_hand {
                                if let Some((world_pos, world_index)) = vectormath::dda(&terrain, &player.camera.position, &player.camera.forward, 6.0) {
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
                                    terrain.place_block(1, &(world_index + offset), &mut gl_resources);
                                    
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn start_terrain_thread(&mut self) {
        #[cfg(feature = "android-lib")] {
            debug!("Starting terrain thread");
        }

        // Create initial terrain around the player, block the main thread so the player doesn't go through the ground
        let chunk_render_distance = 4;
        {
            let terrain = self.terrain.clone();
            let gl_resources = self.gl_resources.clone();
            let noise_config = self.noise_config.clone();
            terrain.write().unwrap().init_worldgen(&Vector3::new(0.0, 0.0, 0.0), chunk_render_distance*2, &mut gl_resources.write().unwrap(), &noise_config.read().unwrap());
        }
        self.resume();

        let terrain_gen = self.terrain.clone();
        let player_gen = self.player.clone();
        let noise_config_gen = self.noise_config.clone();
        let gl_resources_gen = self.gl_resources.clone();
        std::thread::spawn(move || {
            loop {
                // Get the list of chunks which need generation
                let player_chunk = {
                    let player = player_gen.read().unwrap();
                    let player_position = player.position;
                    ChunkIndex::new(
                        player_position.x.floor() as isize / CHUNK_WIDTH as isize,
                        player_position.z.floor() as isize / CHUNK_WIDTH as isize,
                    )
                };

                let chunk_update_list = {
                    let chunks_to_generate = terrain_gen.read().unwrap().get_indices_to_generate(chunk_render_distance, 200, &player_chunk);
                    chunks_to_generate
                };

                // Sleep the thread for a bit if no chunks need to generate
                if chunk_update_list.is_empty() {
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                }
                
                // Generate data for the new chunks that are in range
                for chunk_index in chunk_update_list.iter() {
                    let mut chunk = Box::new(Chunk::new());
                    Terrain::gen_surface_terrain(chunk_index, &mut chunk, &noise_config_gen.read().unwrap());
                    {
                        let mut terrain = terrain_gen.write().unwrap();
                        terrain.insert_chunk(*chunk_index, chunk);
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }

                for chunk_index in chunk_update_list.iter() {
                    let mut terrain = terrain_gen.write().unwrap();
                    let mut gl_resources = gl_resources_gen.write().unwrap();

                    let x_pos = chunk_index + ChunkIndex::new(1, 0);
                    let x_neg = chunk_index + ChunkIndex::new(-1, 0);
                    let z_pos = chunk_index + ChunkIndex::new(0, 1);
                    let z_neg = chunk_index + ChunkIndex::new(0, -1);

                    terrain.update_single_chunk_mesh(chunk_index, &mut gl_resources);
                    terrain.update_single_chunk_mesh(&x_pos, &mut gl_resources);
                    terrain.update_single_chunk_mesh(&x_neg, &mut gl_resources);
                    terrain.update_single_chunk_mesh(&z_pos, &mut gl_resources);
                    terrain.update_single_chunk_mesh(&z_neg, &mut gl_resources);
                }
            }
        });
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

        {
            self.terrain.write().unwrap().init_gl_resources(&mut self.gl_resources.write().unwrap());
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
    }

    pub fn reset_gl_resources(&mut self) {
        self.gl_resources.write().unwrap().invalidate_resources();
    }

    pub fn draw(&mut self) {
        let player = self.player.read().unwrap();
        let terrain = self.terrain.read().unwrap();
        

        unsafe {
            gl::Viewport(0, 0, self.width, self.height);
            gl::ClearColor(0.2, 0.2, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        {
            self.gl_resources.write().unwrap().process_buffer_updates(2);
        }

        let gl_resources = self.gl_resources.read().unwrap();

        let perspective_matrix = perspective_matrix(self.width, self.height);
        let view_matrix = player.camera_view_matrix();

        terrain.draw(&gl_resources, perspective_matrix, view_matrix, self.elapsed_time);

        for entity in &self.entities {
            entity.draw(&gl_resources, perspective_matrix, view_matrix, self.elapsed_time);
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

    pub fn player_input(&mut self, movement: PlayerInput) {
        if self.play_state == PlayState::Running {
            self.input_queue.push(movement);
        }
    }

}
