use cgmath::Vector3;

use crate::graphics::skybox::Skybox;
use crate::physics::collision::{check_world_collision_axis, Collider};
use crate::physics::physics_update::PhysicsUpdate;
use crate::physics::vectormath::{self, Vec3Direction};
pub use crate::player::PlayerInput;
use crate::terrain::chunk::CHUNK_WIDTH;
use crate::terrain::generation::TerrainGenConfig;
use crate::terrain::{ChunkIndex, TerrainEvent};
use crate::{entity::EntityTrait, player::Player, terrain::Terrain};
use crate::{graphics::resources::GLResources, physics::vectormath::Z_VECTOR};

use std::sync::atomic::{AtomicI32, AtomicIsize};
use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use self::workers::EngineWorker;

mod graphics;
mod save;
mod workers;

#[derive(PartialEq, Eq, Debug)]
enum PlayState {
    Running,
    Paused,
}

pub enum EngineEvent {
    UserInput(PlayerInput),     // UserInput(input: PlayerInput)
    EngineState(bool),          // EngineState(is_paused: bool)
}

struct EngineState {
    pub(crate) play_state: PlayState,
    pub(crate) elapsed_time: Duration,
    pub(crate) last_update: Instant,
}

impl Default for EngineState {
    fn default() -> Self {
        Self {
            play_state: PlayState::Paused,
            elapsed_time: Duration::ZERO,
            last_update: Instant::now(),
        }
    }
}

pub struct Engine {
    player: Arc<RwLock<Box<Player>>>,
    terrain: Arc<RwLock<Terrain>>,
    terrain_config: Arc<RwLock<TerrainGenConfig>>,
    entities: Vec<Box<dyn EntityTrait>>,
    skybox: Arc<RwLock<Skybox>>,

    event_queue: Arc<RwLock<Vec<EngineEvent>>>,
    engine_state: Arc<RwLock<EngineState>>,
    
    width: AtomicI32,
    height: AtomicI32,
    render_distance: AtomicIsize,
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
            skybox: Arc::new(RwLock::new(Skybox)),

            
            event_queue: Arc::new(RwLock::new(Vec::new())),
            terrain_config: Arc::new(RwLock::new(terrain_config)),
            engine_state: Arc::new(RwLock::new(EngineState::default())),
            

            width: 0.into(),
            height: 0.into(),
            render_distance: 8.into(),
            gl_resources: Arc::new(RwLock::new(GLResources::new())),
        }
    }
}

impl Engine {

    pub fn init_engine(&mut self) {
        {
            self.terrain.write().unwrap().init_worldgen(
                &Vector3::new(0.0, 0.0, 0.0),
                4,
                &self.terrain_config.read().unwrap(),
            );
        }
        self.terrain.start_thread(self.gl_resources.clone(), self.terrain_config.clone());
    }

    pub fn start_gameloop(&mut self) {
        let player = self.player.clone();
        let terrain = self.terrain.clone();
        let engine_state = self.engine_state.clone();
        let event_queue = self.event_queue.clone();

        std::thread::spawn(move || {
            loop {
                if engine_state.read().unwrap().play_state == PlayState::Running {

                    /*******************************************
                     * Lock engine components during game tick *
                     *******************************************/

                    let mut player_rw = player.write().unwrap();
                    println!("player position: {:?}", player_rw.position);
                    let mut terrain_rw = terrain.write().unwrap();

                    /****************************************
                     * Calculate delta_time since last tick *
                     ****************************************/
                    let delta_time = {
                        let mut engine_state_rw = engine_state.write().unwrap();
                        let now = std::time::Instant::now();
                        let delta_time = now - engine_state_rw.last_update;
                        engine_state_rw.last_update = now;
                        engine_state_rw.elapsed_time += delta_time;
                        delta_time
                    };


                    /***************************
                     * Process incoming inputs *
                     ***************************/
                
                    while let Some(event) = event_queue.write().unwrap().pop() {
                        match event {
                            EngineEvent::UserInput(player_input) => match player_input {
                                PlayerInput::Interact(left_hand, right_hand) => {
                                    let (camera_position, camera_forward) = player_rw.camera_pos_and_dir();
                                    if right_hand {
                                        if let Some((_world_pos, world_index)) = vectormath::dda(
                                            &terrain_rw,
                                            &camera_position,
                                            &camera_forward,
                                            6.0,
                                        ) {
                                            /*let dropped = terrain.set_block(0, &world_index);
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
                                            }*/
                                            terrain_rw.event(TerrainEvent::ModifyBlock(world_index, 0));
                                        }
                                    }
                                    if left_hand {
                                            if let Some((world_pos, world_index)) = vectormath::dda(
                                                &terrain_rw,
                                                &player_rw.camera.position,
                                                &player_rw.camera.forward,
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
                                                //terrain.set_block(1, &(world_index + offset));
                                                terrain_rw.event(TerrainEvent::ModifyBlock(world_index + offset, 1));
                                            }
                                        }
                                    }
                                _ => player_rw.input(player_input),
                            },
                            EngineEvent::EngineState(is_paused) => engine_state.write().unwrap().play_state = if is_paused { PlayState::Paused } else { PlayState::Running },
                        }
                    }

                    /*******************************************
                     * Tick all objects in the array in order: *
                     * 1. Terrain                              *
                     * 2. Player                               *
                     * 3. Entities                             *
                     *******************************************/
                    
                    /******************
                     * Update terrain *
                     ******************/

                    let player_chunk_index = ChunkIndex {
                        x: (player_rw.position.x as f32 / CHUNK_WIDTH as f32).floor() as isize,
                        y: (player_rw.position.z as f32 / CHUNK_WIDTH as f32).floor() as isize,
                    };
                    terrain_rw.event(TerrainEvent::LoadingZones(vec![player_chunk_index]));
                    terrain_rw.tick();
                    //terrain.set_active_chunks(vec![player_chunk_index]);
                    //terrain.update_visible_chunks_near(self.render_distance, &player_chunk_index);


                    /*****************
                     * Update player *
                     *****************/

                    player_rw.update_physics(delta_time.as_secs_f32());

                    let movement_delta = player_rw.movement_delta();

                    player_rw.position.x += movement_delta.x;
                    let overlap_x =
                        check_world_collision_axis(Vec3Direction::X, player_rw.bounding_box(), &terrain_rw);
                    player_rw.correct_position_axis(Vec3Direction::X, overlap_x);

                    player_rw.position.y += movement_delta.y;
                    let overlap_y =
                        check_world_collision_axis(Vec3Direction::Y, player_rw.bounding_box(), &terrain_rw);
                    player_rw.correct_position_axis(Vec3Direction::Y, overlap_y);

                    player_rw.position.z += movement_delta.z;
                    let overlap_z =
                        check_world_collision_axis(Vec3Direction::Z, player_rw.bounding_box(), &terrain_rw);
                    player_rw.correct_position_axis(Vec3Direction::Z, overlap_z);


                    /*******************
                     * Update entities *
                     *******************/
                    
                /*
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
                */
                }

                //TODO: Replace the below statement with logic to target ~20tps
                std::thread::sleep(Duration::from_millis(5));
            }
        });
    }

    pub fn pause(&mut self) {
        self.engine_state.write().unwrap().play_state = PlayState::Paused;
        #[cfg(feature = "android-lib")]
        {
            debug!("Paused");
        }
    }

    pub fn resume(&mut self) {
        let mut engine_state = self.engine_state.write().unwrap();
        engine_state.play_state = PlayState::Running;
        engine_state.last_update = Instant::now();
        #[cfg(feature = "android-lib")]
        {
            debug!("Running");
        }
    }

    pub fn is_paused(&self) -> bool {
        self.engine_state.read().unwrap().play_state == PlayState::Paused
    }

    pub fn engine_event(&mut self, event: EngineEvent) {
        self.event_queue.write().unwrap().push(event);
    }

    /*pub fn player_input(&mut self, movement: PlayerInput) {
        if self.play_state == PlayState::Running {
            self.input_queue.push(movement);
        }
    }*/
}
