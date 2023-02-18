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
    time::{Duration, Instant},
};

use cgmath::{Vector2, Vector3, Zero, InnerSpace, Matrix4};
use entity::EntityTrait;
use graphics::{
    framebuffer::Framebuffer,
    mesh::{block_drop_vertices, FULLSCREEN_QUAD},
    resources::{GLRenderable, GLResources},
    texture::{Texture, TextureFormat}, source::{SCREENQUAD_VERT_SRC, TERRAIN_BITMAP, TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC, POSTPROCESS_FRAG_SRC, SSAO_FRAG_SRC}, depthbuffer::Depthbuffer, shader::Shader, uniform::Uniform, skybox::Skybox,
};
use image::ImageFormat;

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

use crate::graphics::source::{SKYBOX_BITMAP, SKYBOX_VERT_SRC, SKYBOX_FRAG_SRC};

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

                let player_chunk_index = ChunkIndex::new(player.position.x as isize / 16, player.position.z as isize / 16);
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
                                    if let Some(drop) = terrain
                                        .destroy_at_global_pos(&world_index, &mut gl_resources)
                                    {
                                        let boxed_drop = Box::new(drop);
                                        let verts = Box::new(block_drop_vertices(&BLOCKS[boxed_drop.block_id]));
                                        let name = format!("item_{}", boxed_drop.block_id);

                                        gl_resources.update_vao_buffer(name, verts);
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
            let terrain_config = self.terrain_config.clone();
            terrain.write().unwrap().init_worldgen(
                &Vector3::new(0.0, 0.0, 0.0),
                self.render_distance,
                &mut gl_resources.write().unwrap(),
                &terrain_config.read().unwrap(),
            );
        }
        self.resume();

        let terrain_gen = self.terrain.clone();
        let player_gen = self.player.clone();
        let terrain_config_gen = self.terrain_config.clone();
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
                    let placement_queue = terraingen::generate_surface(
                        chunk_index,
                        &mut chunk,
                        &terrain_config_gen.read().unwrap(),
                    );
                    {
                        let mut terrain = terrain_gen.write().unwrap();
                        terrain.insert_chunk(*chunk_index, chunk);
                        terrain.place_features(placement_queue);
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }

                for chunk_index in chunk_update_list.iter() {
                    let terrain = terrain_gen.read().unwrap();
                    let mut gl_resources = gl_resources_gen.write().unwrap();
                    terrain.update_chunk_mesh(chunk_index, &mut gl_resources)
                }
            }
        });
    }

    pub fn init_gl(&mut self, width: i32, height: i32) {
        self.pause();
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
        
        let ssao_program = graphics::shader::Shader::new(SCREENQUAD_VERT_SRC, SSAO_FRAG_SRC).unwrap();

        ssao_program.use_program();
        let mut ssao_kernel = [Vector3::zero(); 64];
        for (i, s) in &mut ssao_kernel.iter_mut().enumerate() {
            let sample: Vector3<f32> = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>(),
            ).normalize() * rand::random();
            let sample_name_rust = format!("samples[{}]", i);
            let sample_name = std::ffi::CString::new(sample_name_rust.as_str()).unwrap();
            ssao_program.set_vec3(&sample_name, &sample);
            *s = sample;
        }

        const SSAO_NOISE_SIZE: usize = 4;
        ssao_program.set_float(unsafe {c_str!("ssao_noise_size")}, SSAO_NOISE_SIZE as f32);
        let mut ssao_noise = [Vector3::zero(); SSAO_NOISE_SIZE*SSAO_NOISE_SIZE];
        for pixel in ssao_noise.iter_mut() {
            *pixel = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                0.0
            );
        }
        let ssao_noise_texture = Texture::from_vector3_array(&ssao_noise, SSAO_NOISE_SIZE as i32, SSAO_NOISE_SIZE as i32);

        let gbuffer_position = Texture::empty(self.width, self.height, TextureFormat::Float);
        let gbuffer_normal = Texture::empty(self.width, self.height, TextureFormat::Float);
        let gbuffer_albedo = Texture::empty(self.width, self.height, TextureFormat::Color);
        let gbuffer_depthbuffer = Depthbuffer::new(self.width, self.height);
        let gbuffer_textures = vec![("position", gbuffer_position), ("normal", gbuffer_normal), ("albedo", gbuffer_albedo)];
        let gbuffer = Framebuffer::with_textures(gbuffer_textures, Some(gbuffer_depthbuffer));

        let ssao_output_texture = Texture::empty(self.width, self.height, TextureFormat::SingleChannel);
        let ssao_output_framebuffer = Framebuffer::with_textures(vec![("ssao", ssao_output_texture)], None);

        let postprocess_program = Shader::new(SCREENQUAD_VERT_SRC, POSTPROCESS_FRAG_SRC).unwrap();
        postprocess_program.use_program();
        postprocess_program.set_float(unsafe {c_str!("ssao_noise_size")}, SSAO_NOISE_SIZE as f32);

        let terrain_texture = Texture::from_dynamic_image_bytes(TERRAIN_BITMAP, ImageFormat::Png);
        let terrain_program = Shader::new(TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC).unwrap();

        let skybox_texture = Texture::from_dynamic_image_bytes(SKYBOX_BITMAP, ImageFormat::Png);
        let skybox_program = Shader::new(SKYBOX_VERT_SRC, SKYBOX_FRAG_SRC).unwrap();

        {
            let mut gl_resources = self.gl_resources.write().unwrap();
            gl_resources.add_vao("screenquad".to_string(), Box::new(Vec::from(FULLSCREEN_QUAD)));
            
            gl_resources.add_framebuffer("gbuffer", gbuffer);
            gl_resources.add_framebuffer("ssao", ssao_output_framebuffer);
            
            gl_resources.add_texture("ssao_noise", ssao_noise_texture);
            gl_resources.add_texture("terrain", terrain_texture);
            gl_resources.add_texture("skybox", skybox_texture);

            gl_resources.add_shader("ssao", ssao_program);
            gl_resources.add_shader("terrain", terrain_program);
            gl_resources.add_shader("postprocess", postprocess_program);
            gl_resources.add_shader("skybox", skybox_program);

            self.skybox.init_gl_resources(&mut gl_resources);


            self.terrain
                .write()
                .unwrap()
                .init_gl_resources(&mut gl_resources);

            for entity in self.entities.iter() {
                entity.init_gl_resources(&mut gl_resources);
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.resume();
    }

    pub fn reset_gl_resources(&mut self) {
        self.gl_resources.write().unwrap().invalidate_resources();
    }

    pub fn draw(&mut self) {
        let player = self.player.read().unwrap();
        let terrain = self.terrain.read().unwrap();

        {
            self.gl_resources.write().unwrap().process_vao_buffer_updates(2);
        }

        let gl_resources = self.gl_resources.read().unwrap();

        let gbuffer_fbo = gl_resources.get_framebuffer("gbuffer").unwrap();
        gbuffer_fbo.bind();

        unsafe {
            gl::Viewport(0, 0, self.width, self.height);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let perspective_matrix =
            perspective_matrix(self.width, self.height, self.render_distance as f32);
        let view_matrix = player.camera_view_matrix();

        let geometry_uniforms: Vec<(&str, Box<dyn Uniform>)> = vec![("perspective_matrix", Box::new(perspective_matrix)), ("view_matrix", Box::new(view_matrix)), ("time", Box::new(self.elapsed_time.as_secs_f32()))];

        terrain.draw(
            &gl_resources,
            &geometry_uniforms,
        );

        for entity in &self.entities {
            entity.draw(
                &gl_resources,
                &geometry_uniforms,
            );
        }

        gbuffer_fbo.unbind();

        let screenquad = gl_resources.get_vao("screenquad").unwrap();

        unsafe {
            //gl::ClearColor(0.4, 0.6, 1.0, 1.0);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let postprocess_shader = gl_resources.get_shader("postprocess").unwrap();
        postprocess_shader.use_program();
        
        //ssao_fbo.bind_render_textures_to_current_fb(vec![("ssao", 0)]);
        gbuffer_fbo.bind_render_textures_to_current_fb(vec![("albedo", 1), ("position", 2), ("normal", 3)]);

        postprocess_shader.set_texture(unsafe {c_str!("ssao")}, 0);
        postprocess_shader.set_texture(unsafe {c_str!("albedo")}, 1);
        postprocess_shader.set_texture(unsafe {c_str!("position")}, 2);
        postprocess_shader.set_texture(unsafe {c_str!("normal")},3);
        postprocess_shader.set_vec2(unsafe {c_str!("resolution")}, &Vector2::new(self.width as f32, self.height as f32));

        screenquad.draw();

        gbuffer_fbo.blit_depth_to_fbo(0, self.width, self.height);

        let skybox_model_matrix = Matrix4::from_translation(player.camera.position) * Matrix4::from_scale(self.render_distance as f32 * 16.0 * 2.0);
        let geometry_uniforms: Vec<(&str, Box<dyn Uniform>)> = vec![
            ("model_matrix", Box::new(skybox_model_matrix)),
            ("perspective_matrix", Box::new(perspective_matrix)),
            ("view_matrix", Box::new(view_matrix))
        ];
        self.skybox.draw(&gl_resources, &geometry_uniforms);

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
