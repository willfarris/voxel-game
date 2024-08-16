use std::{sync::{Arc, RwLock}, time::Duration};
use crate::{graphics::resources::GLResources, terrain::{chunk::Chunk, generation::{terraingen, TerrainGenConfig}, Terrain}};


pub trait EngineWorker {
    fn start_thread(&self, gl_resources: Arc<RwLock<GLResources>>);
}

impl EngineWorker for Arc<RwLock<Terrain>> {
    fn start_thread(&self, gl_resources: Arc<RwLock<GLResources>>) {
        let terrain = self.clone();
        std::thread::spawn(move || {
            loop {
                {
                    let gen_queue = { terrain.write().unwrap().needs_regen() };
                    for chunk_index in gen_queue {
                        let mut chunk = Box::new(Chunk::new());
                        let placement_queue = terraingen::generate_surface(
                            &chunk_index,
                            &mut chunk,
                            &terrain.read().unwrap().terrain_config(),
                        );
                        {
                            let mut terrain = terrain.write().unwrap();
                            terrain.insert_chunk(chunk_index, Arc::new(RwLock::new(chunk)));
                            terrain.queue_features(placement_queue);
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
                    terrain.place_features();
                    terrain.update_meshes(&mut gl_resources.write().unwrap());
                }

                std::thread::sleep(Duration::from_millis(1));
            }
        });
    }
}