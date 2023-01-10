use std::collections::LinkedList;

use cgmath::{Vector3, Vector2};
use noise::{NoiseFn, Seedable, Perlin};


use crate::graphics::resources::GLResources;

use super::{Terrain, chunk::{CHUNK_SIZE, Chunk}, ChunkIndex, BlockIndex, block::{block_index_by_name, Block}, BlockWorldPos};

pub struct NoiseConfig {
    pub noise_offset: Vector2<f64>,
    pub noise_scale: f64,
    pub perlin: Perlin,
}

impl NoiseConfig {
    fn get_perlin(&self, offset: [f64; 2]) -> f64 {
        self.perlin.get(offset)
    }
}

impl Terrain {
    pub(crate) fn init_worldgen(&mut self, start_position: &Vector3<f32>, chunk_radius: isize, gl_resources: &mut GLResources, noise_config: &NoiseConfig) {
        let world_pos = Vector3::new(
            start_position.x.floor() as isize,
            start_position.y.floor() as isize,
            start_position.z.floor() as isize,
        );
        let (start_chunk_index, _block_index) = Terrain::chunk_and_block_index(&world_pos);

        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_y in 0..chunk_radius {
                for chunk_z in -chunk_radius..chunk_radius {
                    let chunk_index = start_chunk_index + ChunkIndex::new(chunk_x, chunk_y, chunk_z);
                    let mut cur_chunk = Chunk::new();
                    Self::gen_surface_terrain(&chunk_index, &mut cur_chunk, &noise_config);
                    self.chunks.insert(chunk_index, cur_chunk);
                }
            }
        }
        self.place_enqueued();

        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_y in 0..chunk_radius {
                for chunk_z in -chunk_radius..chunk_radius {
                    let chunk_index = start_chunk_index + ChunkIndex::new(chunk_x, chunk_y, chunk_z);
                    self.update_chunk_mesh(&chunk_index, gl_resources);
                }
            }
        }
        
    }

    pub(crate) fn gen_surface_terrain(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &NoiseConfig) {
        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_coords = chunk_index * 16 + Vector3::new(block_x as isize, block_y as isize, block_z as isize);
                    let surface = Self::surface_noise(global_coords.x as f64, global_coords.z as f64, noise_config).round() as isize;
                    if global_coords.y == surface + 1 {
                        match rand::random::<usize>()%100 {
                            85..=99 => {
                                let rand_val = rand::random::<usize>()%10;
                                let block_id = match rand_val {
                                    0..=6 => block_index_by_name("Short Grass"),
                                    7 => block_index_by_name("Fern"),
                                    8 => block_index_by_name("Rose"),
                                    _ => block_index_by_name("Dandelion"),
                                };
                                chunk.blocks[block_x][block_y][block_z] = block_id;
                            }
                            40 => {
                                //self.place_tree(Vector3::new(global_x, global_y+1, global_z))
                            }
                            _ => {
                
                            }
                        }
                    } else if global_coords.y == surface {
                        chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Grass");
                    } else if global_coords.y < surface - 2 {
                        chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Stone");
                    } else if global_coords.y < surface {
                        chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Dirt");
                    }
                }
            }
        }

        //Self::gen_foliage(chunk_index, chunk, noise_config);
    }

    fn gen_foliage(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &NoiseConfig) {
        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = block_x as isize + (chunk_index.x * CHUNK_SIZE as isize);
                    let global_y = block_y as isize + (chunk_index.y * CHUNK_SIZE as isize);
                    let global_z = block_z as isize + (chunk_index.z * CHUNK_SIZE as isize);
                    let surface_y = Self::surface_noise(global_x as f64, global_z as f64, noise_config);
                    if (global_y as f64) < surface_y {
                        if global_y == surface_y.floor() as isize {
                            let (_position, current_block_index) = Terrain::chunk_and_block_index(&Vector3::new(global_x, global_y, global_z));
                            if chunk.blocks[current_block_index.x][current_block_index.y][current_block_index.z] != 2 && chunk.blocks[current_block_index.x][current_block_index.y][current_block_index.z] != 3 {
                                continue;
                            }
                            match rand::random::<usize>()%100 {
                                50..=99 => {
                                    let rand_val = rand::random::<usize>()%10;
                                    let block_id = match rand_val {
                                        0..=6 => block_index_by_name("Short Grass"),
                                        7 => block_index_by_name("Fern"),
                                        8 => block_index_by_name("Rose"),
                                        _ => block_index_by_name("Dandelion"),
                                    };
                    
                                    let (position, foliage_block_index) = Terrain::chunk_and_block_index(&Vector3::new(global_x, global_y+1, global_z));
                                    chunk.blocks[foliage_block_index.x][foliage_block_index.y][foliage_block_index.z] = block_id;
                                }
                                40 => {
                                    //self.place_tree(Vector3::new(global_x, global_y+1, global_z))
                                }
                                _ => {
                    
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn surface_noise(global_x: f64, global_z: f64, noise_config: &NoiseConfig) -> f64 {
        5.0 * noise_config.get_perlin([noise_config.noise_scale * global_x + noise_config.noise_offset.x, noise_config.noise_scale * global_z + noise_config.noise_offset.y])
                            //+ (50.0 * self.perlin.get([0.1 * noise_scale * self.noise_offset.x - 100.0, self.noise_offset.y - 44310.0]) + 3.0)
                            + 10.1
    }

    fn append_queued_block(&mut self, block_id: usize, chunk_index: &ChunkIndex, block_index: &BlockIndex) {
        if let Some(list) = self.generation_queue.get_mut(chunk_index) {
            list.push_back((*block_index, block_id));
        } else {
            self.generation_queue.insert(*chunk_index, LinkedList::new());
            if let Some(list) = self.generation_queue.get_mut(chunk_index) {
                list.push_back((*block_index, block_id));
            }
        }
    }

    fn place_enqueued(&mut self) {
        for (position, chunk) in &mut self.chunks {
            if let Some(queue) = self.generation_queue.get(position) {
                for (block_pos, block_id) in queue {
                    chunk.blocks[block_pos.x][block_pos.y][block_pos.z] = *block_id;
                }
            }
        }

        let chunks = &mut self.chunks;
        self.generation_queue.retain( |key, blocks_queue| {
            if let Some(chunk) = chunks.get_mut(key) {
                for (index, block_id) in blocks_queue {
                    chunk.blocks[index.x][index.y][index.z] = *block_id;
                }
                return false;
            }
            true
        });
    }


}