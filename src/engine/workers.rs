use std::{sync::{Arc, RwLock}, time::Duration};
use cgmath::{Vector2, Vector3, Zero};

use crate::{graphics::resources::GLResources, terrain::{chunk::{Chunk, ChunkUpdate, ChunkUpdateInner}, generation::terraingen, ChunkIndex, Terrain}};


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
            let active_chunk = {
                terrain.read().unwrap().active_chunk.clone()
            };
            let chunks = {
                terrain.read().unwrap().chunks.clone()
            };
            loop {
                {
                    let radius = 6;
                    
                    let chunk_index = active_chunk.read().unwrap().clone();
                    for x in -radius..=radius {
                        for z in -radius..=radius {
                            let chunk_index = chunk_index + Vector2::new(x, z);
                            let chunk = chunks.lock().unwrap().get(&chunk_index).cloned();
                            if chunk.is_none() {
                                println!("Generating {:?}", chunk_index);
                                let mut chunk = Box::new(Chunk::new());
                                chunk.next_update = ChunkUpdate::Generated;
                                terraingen::generate_surface(
                                    &chunk_index,
                                    &mut chunk,
                                    &terrain_config,
                                );
                                {
                                    let mut chunks = chunks.lock().unwrap();
                                    chunks.insert(chunk_index, Arc::new(RwLock::new(chunk)));
                                    let adjacent_chunks = [
                                        chunk_index + ChunkIndex::new(1, 0),  //x_pos
                                        chunk_index + ChunkIndex::new(-1, 0), //x_neg
                                        chunk_index + ChunkIndex::new(0, 1),  //z_pos
                                        chunk_index + ChunkIndex::new(0, -1), //z_neg
                                    ];
                                    for index in adjacent_chunks {
                                        if let Some(adjacent_chunk) = chunks.get_mut(&index) {
                                            let mut adjacent_chunk = adjacent_chunk.write().unwrap();
                                            match adjacent_chunk.next_update {
                                                ChunkUpdate::NoUpdate => {
                                                    adjacent_chunk.next_update = ChunkUpdate::NeighborChanged(ChunkUpdateInner::new(index, Vector3::zero(), 0));
                                                    println!("Marked {:?} as NeighborChanged", index);
                                                },
                                                _ => {},
                                            }
                                            
                                        }
                                    }
                                }
                            }
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