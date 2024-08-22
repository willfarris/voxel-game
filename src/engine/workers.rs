use std::{sync::{Arc, RwLock}, time::Duration};
use crate::{graphics::resources::GLResources, terrain::{chunk::{self, Chunk, ChunkUpdate}, generation::{terraingen, TerrainGenConfig}, Terrain}};


pub trait EngineWorker {
    fn start_thread(&self, gl_resources: Arc<RwLock<GLResources>>);
}

impl EngineWorker for Arc<RwLock<Terrain>> {
    fn start_thread(&self, gl_resources: Arc<RwLock<GLResources>>) {
        let terrain = self.clone();
        
        std::thread::spawn(move || {
            let terrain_config  = {
                terrain.read().unwrap().terrain_config()
            };
            let gen_queue = {
                terrain.read().unwrap().chunk_generation_queue.clone()
            };
            loop {
                {
                    if let Some(chunk_index) = gen_queue.pop() {
                        println!("Generating {:?}", chunk_index);
                        let mut chunk = Box::new(Chunk::new());
                        chunk.next_update = ChunkUpdate::Generated;
                        terraingen::generate_surface(
                            &chunk_index,
                            &mut chunk,
                            &terrain_config,
                        );
                        {
                            let mut terrain = terrain.write().unwrap();
                            terrain.insert_chunk(chunk_index, Arc::new(RwLock::new(chunk)));
                        }
                    }
                }

                /* TODO:
                 * Get chunks which need updates
                 * for chunk in needs_update:
                 *   ... physics update ...
                 *   ... create cube w/ side dimensions CHUNK_WIDTH+1
                 *   ... calculate lighting in chunk ...
                 *   for side in {-x, +x, -z, +z}:
                 *      let side_index = chunk index at side
                 *      for block on side face:
                 *          light_update_map[side_index].push((side_index, light val from larger cube))
                 *      lighting_update_queue.push(side)
                 *    lighting_update_queue.push(chunk)
                 *   
                 *   for index in lighting_update_queue
                 *     ... flood fill? ...
                 *     ... rebuild chunk mesh data ...
                 */

                {
                    let mut terrain = terrain.write().unwrap();
                    terrain.update_meshes(&mut gl_resources.write().unwrap());
                }

                std::thread::sleep(Duration::from_millis(1));
            }
        });
    }
}