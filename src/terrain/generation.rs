use std::collections::LinkedList;

use cgmath::{Vector3, Vector2};
use noise::{NoiseFn, Seedable, Perlin};


use crate::graphics::resources::GLResources;

use super::{Terrain, chunk::{Chunk, CHUNK_WIDTH, CHUNK_HEIGHT}, ChunkIndex, BlockIndex, block::{block_index_by_name, Block}, BlockWorldPos};

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
        let start_chunk_index = ChunkIndex::new(
            start_position.x.floor() as isize / CHUNK_WIDTH as isize,
            start_position.z.floor() as isize / CHUNK_WIDTH as isize,
        );

        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_z in -chunk_radius..chunk_radius {
                let chunk_index = start_chunk_index + ChunkIndex::new(chunk_x, chunk_z);
                let mut cur_chunk = Box::new(Chunk::new());
                Self::gen_surface_terrain(&chunk_index, &mut cur_chunk, &noise_config);
                self.chunks.insert(chunk_index, cur_chunk);
            }
        }

        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_z in -chunk_radius..chunk_radius {
                let chunk_index = start_chunk_index + ChunkIndex::new(chunk_x, chunk_z);
                self.update_chunk_mesh(&chunk_index, gl_resources);
            }
        }
        
    }

    pub(crate) fn gen_surface_terrain(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &NoiseConfig) {
        for block_x in 0..CHUNK_WIDTH {
            for block_y in 0..CHUNK_HEIGHT {
                for block_z in 0..CHUNK_WIDTH {
                    let global_coords = BlockWorldPos::new(chunk_index.x, 0, chunk_index.y) * 16 + BlockWorldPos::new(block_x as isize, block_y as isize, block_z as isize);
                    let surface = Self::surface_noise(global_coords.x as f64, global_coords.z as f64, noise_config).round() as isize;
                    if global_coords.y < surface {
                        chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Stone");
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

}