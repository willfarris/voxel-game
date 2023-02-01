mod entity;
mod graphics;
mod item;
mod macros;
mod physics;
mod player;
mod terrain;

#[cfg(feature = "android-lib")]
#[macro_use]
extern crate log;
#[cfg(feature = "android-lib")]
extern crate android_log;
#[cfg(feature = "android-lib")]
extern crate jni;
#[cfg(feature = "android-lib")]
mod java_interface;

use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration, ffi::CStr,
};

use cgmath::{Vector2, Vector3, Zero, InnerSpace};
use entity::EntityTrait;
use graphics::{
    buffer::BufferObject,
    framebuffer::Framebuffer,
    mesh::{block_drop_vertices, FULLSCREEN_QUAD},
    resources::{GLRenderable, GLResources},
    texture::{Texture, TextureFormat}, source::{GBUFFER_FRAG_SRC, SCREENQUAD_VERT_SRC}, depthbuffer::Depthbuffer, vbo::{VertexBufferObject}, vao::VertexAttributeObject,
};
use noise::Perlin;
use physics::{
    collision::{check_world_collision_axis, Collider},
    physics_update::PhysicsUpdate,
    vectormath::{self, Vec3Direction, Z_VECTOR},
};
use player::{camera::perspective_matrix, Player};
use terrain::{
    block::BLOCKS,
    chunk::{Chunk, CHUNK_WIDTH},
    generation::{terraingen, TerrainGenConfig},
    ChunkIndex, Terrain,
};

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
    noise_config: Arc<RwLock<TerrainGenConfig>>,

    width: i32,
    height: i32,
    render_distance: isize,
    gl_resources: Arc<RwLock<GLResources>>,
}

impl Default for Engine {
    fn default() -> Self {
        let player = Box::new(Player::new(Vector3::new(0.0, 64.0, 0.0), Z_VECTOR));
        let terrain = Terrain::new();
        let noise_config = TerrainGenConfig::default();

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
            render_distance: 8,
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
                entity.update_physics(delta_time);

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

                while !self.input_queue.is_empty() {
                    let input = self.input_queue.remove(0);
                    match input {
                        PlayerInput::Look(dx, dy) => {
                            player.camera.rotate_on_x_axis(dx);
                            player.camera.rotate_on_y_axis(dy);
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
                                    if let Some(drop) = terrain
                                        .destroy_at_global_pos(&world_index, &mut gl_resources)
                                    {
                                        let boxed_drop = Box::new(drop);
                                        let verts = Box::new(block_drop_vertices(&BLOCKS[boxed_drop.block_id]));
                                        let vbo = VertexBufferObject::create_buffer(verts);
                                        let vao = VertexAttributeObject::with_buffer(vbo);
                                        let name = format!("item_{}", boxed_drop.block_id);
                                        
                                        gl_resources.vaos.insert(name, vao);
                                        self.entities.push(boxed_drop);
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
                                    terrain.place_block(
                                        1,
                                        &(world_index + offset),
                                        &mut gl_resources,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn start_terrain_thread(&mut self) {
        #[cfg(feature = "android-lib")]
        {
            debug!("Starting terrain thread");
        }

        let render_distance = self.render_distance;

        // Create initial terrain around the player, block the main thread so the player doesn't go through the ground
        {
            let terrain = self.terrain.clone();
            let gl_resources = self.gl_resources.clone();
            let noise_config = self.noise_config.clone();
            terrain.write().unwrap().init_worldgen(
                &Vector3::new(0.0, 0.0, 0.0),
                self.render_distance,
                &mut gl_resources.write().unwrap(),
                &noise_config.read().unwrap(),
            );
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
                    let chunks_to_generate = terrain_gen.read().unwrap().get_indices_to_generate(
                        render_distance,
                        200,
                        &player_chunk,
                    );
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
                    terraingen::generate_surface(
                        chunk_index,
                        &mut chunk,
                        &noise_config_gen.read().unwrap(),
                    );
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
        #[cfg(target_os = "android")]
        {
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

        let screenquad_vbo = VertexBufferObject::create_buffer(Box::new(Vec::from(FULLSCREEN_QUAD)));
        let screenquad_vao = VertexAttributeObject::with_buffer(screenquad_vbo);
        
        let gbuffer_program = graphics::shader::Shader::new(SCREENQUAD_VERT_SRC, GBUFFER_FRAG_SRC).unwrap();

        gbuffer_program.use_program();
        let mut ssao_kernel = [Vector3::zero(); 64];
        for (i, s) in &mut ssao_kernel.iter_mut().enumerate() {
            let sample: Vector3<f32> = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>(),
            ).normalize() * rand::random();
            let sample_name_rust = format!("samples[{}]", i);
            let sample_name = std::ffi::CString::new(sample_name_rust.as_str()).unwrap();
            gbuffer_program.set_vec3(&sample_name, &sample);
            *s = sample;
        }

        let mut ssao_noise = [Vector3::zero(); 16];
        for pixel in ssao_noise.iter_mut() {
            *pixel = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                0.0
            );
        }
        let ssao_noise_texture = Texture::from_vector3_array(&ssao_noise, 4, 4);

        let gbuffer_position = Texture::empty(self.width, self.height, TextureFormat::Float);
        let gbuffer_normal = Texture::empty(self.width, self.height, TextureFormat::Float);
        let gbuffer_albedo = Texture::empty(self.width, self.height, TextureFormat::Color);
        let gbuffer_depthbuffer = Depthbuffer::new(self.width, self.height);
        let gbuffer_textures = vec![("position", gbuffer_position), ("normal", gbuffer_normal), ("albedo", gbuffer_albedo)];
        let gbuffer = Framebuffer::with_textures(gbuffer_textures, Some(gbuffer_depthbuffer));

        {
            let mut gl_resources = self.gl_resources.write().unwrap();
            gl_resources.vaos.insert("screenquad".to_string(), screenquad_vao);
            gl_resources.framebuffers.insert("gbuffer", gbuffer);
            gl_resources.textures.insert("ssao_noise", ssao_noise_texture);
            gl_resources.shaders.insert("gbuffer", gbuffer_program);
        }

        {
            self.terrain
                .write()
                .unwrap()
                .init_gl_resources(&mut self.gl_resources.write().unwrap());
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn reset_gl_resources(&mut self) {
        //self.gl_resources.write().unwrap().invalidate_resources();
    }

    pub fn draw(&mut self) {
        let player = self.player.read().unwrap();
        let terrain = self.terrain.read().unwrap();

        {
            self.gl_resources.write().unwrap().process_vao_buffer_updates(1);
        }

        let gl_resources = self.gl_resources.read().unwrap();

        let gbuffer_fbo = gl_resources.framebuffers.get("gbuffer").unwrap();
        gbuffer_fbo.bind();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let perspective_matrix =
            perspective_matrix(self.width, self.height, self.render_distance as f32);
        let view_matrix = player.camera_view_matrix();

        terrain.draw(
            &gl_resources,
            perspective_matrix,
            view_matrix,
            self.elapsed_time,
        );

        for entity in &self.entities {
            entity.draw(
                &gl_resources,
                perspective_matrix,
                view_matrix,
                self.elapsed_time,
            );
        }

        gbuffer_fbo.unbind();


        unsafe {
            gl::ClearColor(0.4, 0.6, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let gbuffer_shader = gl_resources.shaders.get("gbuffer").unwrap();
        let ssao_noise_texture = gl_resources.textures.get("ssao_noise").unwrap();
        let screenquad = gl_resources.vaos.get("screenquad").unwrap();
        
        gbuffer_fbo.bind_render_textures_to_current_fb(vec!["position", "normal", "albedo"]);

        ssao_noise_texture.use_as_framebuffer_texture(3);

        gbuffer_shader.use_program();
        gbuffer_shader.set_texture(unsafe {c_str!("position")}, 0);
        gbuffer_shader.set_texture(unsafe {c_str!("normal")}, 1);
        gbuffer_shader.set_texture(unsafe {c_str!("albedo")}, 2);
        gbuffer_shader.set_texture(unsafe {c_str!("ssao_noise")}, 3);

        gbuffer_shader.set_mat4(unsafe {c_str!("projection")}, &perspective_matrix);
        gbuffer_shader.set_vec2(unsafe {c_str!("resolution")}, &Vector2::new(self.width as f32, self.height as f32));
        gbuffer_shader.set_float(unsafe {c_str!("time")}, self.elapsed_time);

        screenquad.draw();

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
