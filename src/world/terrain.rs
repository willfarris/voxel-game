use std::collections::LinkedList;

use cgmath::Vector3;
use noise::{NoiseFn, Seedable};

use super::{World, chunk::{CHUNK_SIZE, Chunk}, ChunkIndex, BlockIndex, block::block_index_by_name, BlockWorldPos};

impl World {
    pub fn gen_terrain(&mut self, chunk_radius: isize, seed: usize) {
        self.perlin.set_seed(seed.try_into().unwrap());
        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_y in 0..chunk_radius {
                for chunk_z in -chunk_radius..chunk_radius {
                    let chunk_index = ChunkIndex::new(chunk_x, chunk_y, chunk_z);
                    let position = Vector3::new(
                        (chunk_index.x * CHUNK_SIZE as isize) as f32,
                        (chunk_index.y * CHUNK_SIZE as isize) as f32,
                        (chunk_index.z * CHUNK_SIZE as isize) as f32,
                    );
                    let mut cur_chunk = Chunk::new(position);
                    
                    self.gen_chunk(&chunk_index, &mut cur_chunk);
                    self.gen_caves(&chunk_index, &mut cur_chunk);
                    self.gen_foliage(&chunk_index, &mut cur_chunk);
                    self.chunks.insert(chunk_index, cur_chunk);
                }
            }
        }

        self.place_enqueued();

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

    fn surface_noise(&self, global_x: f64, global_z: f64) -> f64 {
        5.0 * self.perlin.get([self.noise_scale * global_x + self.noise_offset.x, self.noise_scale * global_z + self.noise_offset.y])
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
    }

    fn gen_chunk(&mut self, chunk_index: &ChunkIndex, chunk: &mut Chunk) {
        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = block_x as isize + (chunk_index.x * CHUNK_SIZE as isize);
                    let global_y = block_y as isize + (chunk_index.y * CHUNK_SIZE as isize);
                    let global_z = block_z as isize + (chunk_index.z * CHUNK_SIZE as isize);
                    let surface_y = self.surface_noise(global_x as f64, global_z as f64);
                    if (global_y as f64) < surface_y {
                        if global_y == surface_y.floor() as isize {
                            chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Grass");
                        } else if (global_y as f64) < (7.0 * surface_y/8.0).floor() {
                            match rand::random::<usize>()%100 {
                                0 => chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Iron Ore"),
                                1..=3 => chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Coal"),
                                _ => chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Stone"),
                            }                            
                        } else {
                            chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Dirt");
                        }
                    }
                }
            }
        }
    }

    fn gen_caves(&mut self, chunk_index: &ChunkIndex, chunk: &mut Chunk) {
        let noise_scale = 0.1;
        let cutoff = 0.6;
        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = (block_x as isize + (chunk_index.x * CHUNK_SIZE as isize)) as f64;
                    let global_y = (block_y as isize + (chunk_index.y * CHUNK_SIZE as isize)) as f64;
                    let global_z = (block_z as isize + (chunk_index.z * CHUNK_SIZE as isize)) as f64;
                    let noise = self.perlin.get([noise_scale * global_x, noise_scale * global_y, noise_scale * global_z]);
                    if noise > cutoff {
                        chunk.blocks[block_x][block_y][block_z] = 0;
                    }
                }
            }
        }
    }

    fn gen_foliage(&mut self, chunk_index: &ChunkIndex, chunk: &mut Chunk) {
        for block_x in 0..CHUNK_SIZE {
            for block_y in 0..CHUNK_SIZE {
                for block_z in 0..CHUNK_SIZE {
                    let global_x = block_x as isize + (chunk_index.x * CHUNK_SIZE as isize);
                    let global_y = block_y as isize + (chunk_index.y * CHUNK_SIZE as isize);
                    let global_z = block_z as isize + (chunk_index.z * CHUNK_SIZE as isize);
                    let surface_y = self.surface_noise(global_x as f64, global_z as f64);
                    if (global_y as f64) < surface_y {
                        if global_y == surface_y.floor() as isize {
                            let (_position, current_block_index) = World::chunk_and_block_index(&Vector3::new(global_x, global_y, global_z));
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
                    
                                    let (position, foliage_block_index) = World::chunk_and_block_index(&Vector3::new(global_x, global_y+1, global_z));
                                    if let Some(chunk) = self.chunks.get_mut(&position) {
                                        chunk.blocks[foliage_block_index.x][foliage_block_index.y][foliage_block_index.z] = block_id;
                                    } else {
                                        self.append_queued_block(block_id, &position, &foliage_block_index);
                                    }
                                }
                                40 => {
                                    self.place_tree(Vector3::new(global_x, global_y+1, global_z))
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

    fn place_tree(&mut self, world_pos: BlockWorldPos) {
        for y in 0..5 {
            let (chunk_index, block_index) = World::chunk_and_block_index(&(world_pos + Vector3::new(0, y, 0)));
            if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
                chunk.blocks[block_index.x][block_index.y][block_index.z] = block_index_by_name("Oak Log");
            } else {
                self.append_queued_block(block_index_by_name("Oak Log"), &chunk_index, &block_index);
            }
        }

        for x in -1..=1 {
            for z in -1..=1 {
                for y in 3..=5 {
                    if (x == -1 && z == -1 && y == 5) || (x == 1 && z == 1 && y == 5) || (x == -1 && z == 1 && y == 5) || (x == 1 && z == -1 && y == 5) || (x == 0 && z == 0 && y == 3) {
                        continue;
                    }
                    let (chunk_index, block_index) = World::chunk_and_block_index(&(world_pos + Vector3::new(x, y, z)));
                    if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
                        chunk.blocks[block_index.x][block_index.y][block_index.z] = block_index_by_name("Oak Leaves");
                    } else {
                        self.append_queued_block(block_index_by_name("Oak Leaves"), &chunk_index, &block_index);
                    }
                }
            }
        }
    }
}