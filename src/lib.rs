
mod graphics;
mod macros;
mod mesh_object;
mod world;
mod player;
mod physics;

#[cfg(target_os = "android")]
#[macro_use] extern crate log;
#[cfg(target_os = "android")]
extern crate android_log;
#[cfg(target_os = "android")]
extern crate jni;
#[cfg(target_os = "android")]
mod java_interface;

use cgmath::Vector3;
use physics::vectormath::Z_VECTOR;
use player::{Player, camera::perspective_matrix};
use world::World;
use graphics::resources::GLRenderable;
use mesh_object::MeshObject;

pub use physics::vectormath::q_rsqrt;

#[derive(PartialEq, Eq)]
pub enum PlayState {
    Running,
    Paused,
}

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
        //let camera = Camera::new(Vector3::new(0.0, 5.0, -10.0), Vector3::new(0.0, 0.0, 1.0));
        let player = Some(Player::new(Vector3::new(0.0, 30.0, 0.0), Z_VECTOR));

        let mut entities = Vec::new();
        let test_mesh = MeshObject::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
            mesh_object::DEFAULT_CUBE.to_vec(),
            include_str!("../shaders/cube.vert"),
            include_str!("../shaders/cube.frag"),
            include_bytes!("../assets/cube_test.png"),
        );
        entities.push(test_mesh);

        //let world_vertex_src = include_str!("../shaders/world.vert");
        //let world_frag_src = include_str!("../shaders/world.frag");
        let world_texture_bitmap = include_bytes!("../assets/terrain.png");
        let mut world = Some(World::new(
            include_str!("../shaders/cube.vert"),
            include_str!("../shaders/cube.frag"),
            world_texture_bitmap,
        ));
        #[cfg(target_os = "android")] {
            debug!("Placing blocks");
        }
        if let Some(w) = world.as_mut() {
            w.gen_terrain(5, 69);
        }
        #[cfg(target_os = "android")] {
            debug!("World built");
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

        /*debug!("Creating vbo...");
        let mut vertexbuffer: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut vertexbuffer as *mut u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertexbuffer);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of::<[f32; 9]>() as isize, (&TEST_TRIANGLE_VERTS as *const [f32; 9])  as *const c_void, gl::STATIC_DRAW);
        }*/

        for entity in &mut self.entities {
            entity.init_gl_resources();
        }

        if let Some(world) = &mut self.world {
            world.build_all_chunk_mesh(5, Vector3::new(0.0, 0.0, 0.0));
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        #[cfg(target_os = "android")] {
            debug!("GL setup done");
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
                        //self.player.inventory.selected = selected;
                    },
                    PlayerMovement::Interact(left_hand, right_hand) => {
                        /*if right_hand {
                            if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                                let block_id = self.terrain.block_at_global_pos(world_index);
                                self.player.inventory.add_to_inventory(block_id);
                                self.terrain.destroy_at_global_pos(world_index);
                            }
                        }
                        if left_hand {
                            if let Some((_, world_index)) = vectormath::dda(&self.terrain, &self.player.camera.position, &self.player.camera.forward, 6.0) {
                                self.terrain.interact_at_global_pos(world_index);
                            }
                        }*/
                    }
                }
            }
        }
    }


}
