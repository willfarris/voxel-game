use std::time::Duration;

use cgmath::Vector3;

use crate::terrain::{
    chunk::{Chunk, CHUNK_WIDTH},
    generation::terraingen,
    ChunkIndex,
};

use super::Engine;

impl Engine {
    pub fn start_workers(&mut self) {
        self.terrain_thread();
        self.lighting_thread();
    }

    fn terrain_thread(&mut self) {
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
            }
        });
    }

    fn lighting_thread(&mut self) {
        let terrain_light = self.terrain.clone();
        let player_light = self.player.clone();

        std::thread::spawn(move || loop {
            let player_chunk = {
                let player = player_light.read().unwrap();
                let player_position = player.position;
                ChunkIndex::new(
                    player_position.x.floor() as isize / CHUNK_WIDTH as isize,
                    player_position.z.floor() as isize / CHUNK_WIDTH as isize,
                )
            };
            let pending_light_updates = {
                let mut terrain = terrain_light.write().unwrap();
                terrain.pending_light_updates()
            };

            for chunk_index in pending_light_updates {
                let chunk = { terrain_light.write().unwrap().copy_chunk(&chunk_index) };
                if let Some(mut chunk) = chunk {
                    chunk.update_lighting();
                    terrain_light
                        .write()
                        .unwrap()
                        .insert_chunk(chunk_index, chunk);
                }
            }

            std::thread::sleep(Duration::from_millis(100));
        });
    }
}
