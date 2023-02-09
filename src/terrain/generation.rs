use cgmath::{Vector2, Vector3};
use noise::{NoiseFn, Perlin};
use splines::Spline;

use crate::graphics::resources::GLResources;

use super::{
    chunk::{Chunk, CHUNK_WIDTH},
    ChunkIndex, Terrain, BlockWorldPos,
};

pub struct TerrainGenConfig {
    perlin: Perlin,
    continentalness_scale: Vector2<f64>,
    continentalness_spline: Spline<f64, f64>,

    biome_table: [Biome; 2],
    cont_map_spline: Spline<f64, f64>,
}

impl Default for TerrainGenConfig {
    fn default() -> Self {
        let cont_keys = vec![
            splines::Key::new(f64::MIN, 0.0, splines::Interpolation::Linear),
            splines::Key::new(-1.0, 0.0, splines::Interpolation::Linear),
            splines::Key::new(-0.48, 0.3, splines::Interpolation::Linear),
            splines::Key::new(0.2, 0.4, splines::Interpolation::Linear),
            splines::Key::new(0.3, 0.9, splines::Interpolation::Linear),
            splines::Key::new(1.0, 1.0, splines::Interpolation::Linear),
            splines::Key::new(f64::MAX, 1.0, splines::Interpolation::Linear),
        ];

        let biome_cont_map = vec![
            splines::Key::new(f64::MIN, 0.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(0.0, 0.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(0.5, 1.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(0.75, 1.0, splines::Interpolation::Step(1.0)),
        ];

        Self {
            perlin: Perlin::new(1),
            continentalness_scale: Vector2::new(0.002, 0.002),
            continentalness_spline: splines::Spline::from_vec(cont_keys),

            biome_table: [Biome::Plains, Biome::Hills],
            cont_map_spline: splines::Spline::from_vec(biome_cont_map),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Biome {
    Plains,
    Hills,
    Desert,
}

impl TerrainGenConfig {
    pub(crate) fn get_perlin(&self, offset: [f64; 2]) -> f64 {
        self.perlin.get(offset)
    }

    fn get_continentalness(&self, offset: [f64; 2]) -> f64 {
        let cont_sample = self.perlin.get([
            offset[0] * self.continentalness_scale.x as f64,
            offset[1] * self.continentalness_scale.y as f64,
        ]);
        self.continentalness_spline
            .sample(cont_sample)
            .unwrap_or(0.0)
    }

    fn get_biome(&self, offset: [f64; 2]) -> Biome {
        let cont_index = self
            .cont_map_spline
            .sample(self.get_continentalness(offset))
            .unwrap_or(0.0)
            .round() as usize;
        self.biome_table[cont_index]
    }

    pub(crate) fn get_surface(&self, offset: [f64; 2]) -> f64 {
        let continentalness = self.get_continentalness(offset);
        let surface_noise = 0.5 * self.get_perlin([offset[0] * 0.02, offset[1] * 0.02]) + 0.5;
        32.0 * surface_noise * continentalness + 32.0
    }
}

pub(crate) mod terraingen {
    use std::collections::LinkedList;

    use super::{Biome, TerrainGenConfig};
    use crate::terrain::{
        block::block_index_by_name,
        chunk::{Chunk, CHUNK_WIDTH},
        BlockWorldPos, ChunkIndex, BlockIndex,
    };

    pub fn generate_surface(
        chunk_index: &ChunkIndex,
        chunk: &mut Chunk,
        noise_config: &TerrainGenConfig,
    ) -> Vec<(BlockWorldPos, usize)> {
        shape_terrain(chunk_index, chunk, noise_config);
        place_biomes(chunk_index, chunk, noise_config);
        let queued_features = place_features(chunk_index, chunk, noise_config);

        queued_features
    }

    fn shape_terrain(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &TerrainGenConfig) {
        for block_x in 0..CHUNK_WIDTH {
            for block_z in 0..CHUNK_WIDTH {
                let global_coords = BlockWorldPos::new(chunk_index.x, 0, chunk_index.y) * 16
                    + BlockWorldPos::new(block_x as isize, 0, block_z as isize);
                let surface =
                    noise_config.get_surface([global_coords.x as f64, global_coords.z as f64]);
                for block_y in 0..=surface.round() as usize {
                    chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Stone");
                }
            }
        }
    }

    fn place_biomes(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &TerrainGenConfig) {
        for block_x in 0..CHUNK_WIDTH {
            for block_z in 0..CHUNK_WIDTH {
                let global_coords = [
                    chunk_index.x as f64 * 16.0 + block_x as f64,
                    chunk_index.y as f64 * 16.0 + block_z as f64,
                ];
                let surface = noise_config.get_surface(global_coords).round() as usize;
                let biome = noise_config.get_biome(global_coords);
                match biome {
                    Biome::Plains | Biome::Hills => {
                        for block_y in surface - 1..surface {
                            chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Dirt");
                        }
                        chunk.blocks[block_x][surface][block_z] = block_index_by_name("Grass");
                    }
                    Biome::Desert => {
                        for block_y in surface - 1..=surface {
                            chunk.blocks[block_x][block_y][block_z] = block_index_by_name("Sand");
                        }
                    }
                }
            }
        }
    }

    fn place_features(
        chunk_index: &ChunkIndex,
        chunk: &mut Chunk,
        noise_config: &TerrainGenConfig,
    ) -> Vec<(BlockWorldPos, usize)> {
        let mut placement_queue = Vec::new();

        for block_x in 0..CHUNK_WIDTH {
            for block_z in 0..CHUNK_WIDTH {
                let global_coords = [
                    chunk_index.x as f64 * 16.0 + block_x as f64,
                    chunk_index.y as f64 * 16.0 + block_z as f64,
                ];
                let surface = noise_config.get_surface(global_coords).round() as usize;
                let biome = noise_config.get_biome(global_coords);
                match biome {
                    Biome::Plains => {
                        let has_grass: u8 = rand::random();
                        if let 0..=64 = has_grass {
                            /*chunk.blocks[block_x][surface + 1][block_z] =
                                block_index_by_name("Short Grass");*/
                            placement_queue.push((BlockWorldPos::new(global_coords[0] as isize, surface as isize + 1, global_coords[1] as isize), block_index_by_name("Short Grass")));

                        } else if let 65 = has_grass {
                            //place_tree(chunk_index, chunk, BlockWorldPos::new(global_coords[0] as isize, surface as isize, global_coords[1] as isize));
                        }
                    }
                    Biome::Hills => {}
                    Biome::Desert => {}
                }
            }
        }

        placement_queue
    }
}

impl Terrain {
    pub(crate) fn init_worldgen(
        &mut self,
        start_position: &Vector3<f32>,
        chunk_radius: isize,
        gl_resources: &mut GLResources,
        noise_config: &TerrainGenConfig,
    ) {
        let start_chunk_index = ChunkIndex::new(
            start_position.x.floor() as isize / CHUNK_WIDTH as isize,
            start_position.z.floor() as isize / CHUNK_WIDTH as isize,
        );

        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_z in -chunk_radius..chunk_radius {
                let chunk_index = start_chunk_index + ChunkIndex::new(chunk_x, chunk_z);
                let mut cur_chunk = Box::new(Chunk::new());
                terraingen::generate_surface(&chunk_index, &mut cur_chunk, noise_config);
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

    pub(crate) fn place_features(&mut self, feature_blocks: Vec<(BlockWorldPos, usize)>) {
        //todo!("place the blocks in feature_blocks in the world and place the rest in the placement queue");

        for (world_pos, block_id) in feature_blocks {
            if let Some((chunk_index, block_index)) = Terrain::chunk_and_block_index(&world_pos) {
                if let Some(chunk) = self.chunks.get(&chunk_index) {
                    todo!("Place the block in the chunk if it exists")
                } else {
                    todo!("If the chunk does not yet exist, place the block into the placement queue")
                    //self.placement_queue.insert(chunk_index, (block_index, block_id));
                }
            }
        }
    }
}
