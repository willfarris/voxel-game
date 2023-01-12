

use cgmath::{Vector3, Vector2};
use noise::{NoiseFn, Perlin};
use splines::Spline;


use crate::graphics::resources::GLResources;

use super::{Terrain, chunk::{Chunk, CHUNK_WIDTH, CHUNK_HEIGHT}, ChunkIndex, block::{block_index_by_name}, BlockWorldPos};

pub struct NoiseConfig {
    perlin: Perlin,
    continentalness_scale: Vector2<f64>,
    continentalness_spline: Spline<f64, f64>,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        let cont_keys = vec![
            splines::Key::new(-1.0, 0.0, splines::Interpolation::Linear),
            splines::Key::new(0.0, 3.0, splines::Interpolation::Linear),
            splines::Key::new(0.1, 18.0, splines::Interpolation::Linear),
            splines::Key::new(1.0, 20.0, splines::Interpolation::Linear),
        ];
        let continentalness_spline = splines::Spline::from_vec(cont_keys);

        Self {
            perlin: Perlin::new(1),
            continentalness_scale: Vector2::new(0.01, 0.01),
            continentalness_spline,
        }
    }
}

impl NoiseConfig {
    pub(crate) fn get_continentalness(&self, x: isize, z: isize) -> f64 {
        let noise = self.perlin.get([x as f64 * self.continentalness_scale.x, z as f64 * self.continentalness_scale.y]);
        noise
    }

    pub(crate) fn get_perlin(&self, offset: [f64; 2]) -> f64 {
        self.perlin.get(offset)
    }

    pub(crate) fn get_surface(&self, offset: [f64; 2]) -> f64 {
        let mut sample = 0.0;
        
        for i in 1..=4 {
            sample += (5 - i) as f64 * self.perlin.get([offset[0] * self.continentalness_scale.x * i as f64, offset[1] * self.continentalness_scale.y * i as f64]);
        }
        
        
        let splined = self.continentalness_spline.sample(sample).unwrap_or(0.0);

        //println!("{} -> {}", sample, splined);

        splined
    }
}

pub(crate) mod generation {
    use crate::terrain::{ChunkIndex, chunk::{Chunk, CHUNK_WIDTH, CHUNK_HEIGHT}, BlockWorldPos, block::block_index_by_name};
    use super::NoiseConfig;

    pub fn generate_surface(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &NoiseConfig) {
        shape_terrain(chunk_index, chunk, noise_config);

    }

    fn shape_terrain(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &NoiseConfig) {
        for block_x in 0..CHUNK_WIDTH {
            for block_y in 0..CHUNK_HEIGHT {
                for block_z in 0..CHUNK_WIDTH {
                    let global_coords = BlockWorldPos::new(chunk_index.x, 0, chunk_index.y) * 16 + BlockWorldPos::new(block_x as isize, block_y as isize, block_z as isize);

                    let surface = noise_config.get_surface([global_coords.x as f64, global_coords.z as f64]);
                    
                    if global_coords.y <= surface.round() as isize {
                        chunk.blocks[block_x][block_y][block_z] =  block_index_by_name("Stone");
                    }/* else if global_coords.y < 10 {
                        chunk.blocks[block_x][block_y][block_z] =  block_index_by_name("Dirt");
                    }*/
                }
            }
        }
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
                generation::generate_surface(&chunk_index, &mut cur_chunk, noise_config);
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

}