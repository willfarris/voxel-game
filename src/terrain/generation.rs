use std::collections::HashMap;

use cgmath::{Vector2, Vector3};
use noise::{NoiseFn, Perlin};
use splines::Spline;

use super::{
    chunk::{Chunk, CHUNK_WIDTH}, BlockWorldPos, ChunkIndex, ChunkListTrait, Terrain
};

pub struct TerrainGenConfig {
    perlin: Perlin,
    continentalness_scale: Vector2<f64>,
    continentalness_spline: Spline<f64, f64>,

    biome_table: [Biome; 3],
    cont_map_spline: Spline<f64, f64>,

    world_features: HashMap<String, Vec<Vec<Vec<usize>>>>,
}

impl Default for TerrainGenConfig {
    fn default() -> Self {
        let cont_keys = vec![
            splines::Key::new(f64::MIN, 0.0, splines::Interpolation::Linear),
            splines::Key::new(-1.0, 0.0, splines::Interpolation::Linear),
            splines::Key::new(-0.7, 0.1, splines::Interpolation::Linear),
            splines::Key::new(-0.2, 0.3, splines::Interpolation::Linear),
            splines::Key::new(-0.1, 0.6, splines::Interpolation::Linear),
            splines::Key::new(0.25, 0.7, splines::Interpolation::Linear),
            splines::Key::new(0.3, 0.8, splines::Interpolation::Linear),
            splines::Key::new(1.0, 1.0, splines::Interpolation::Linear),
            splines::Key::new(f64::MAX, 1.0, splines::Interpolation::Linear),
        ];

        let biome_cont_map = vec![
            splines::Key::new(f64::MIN, 0.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(0.4, 0.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(0.6, 1.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(0.7, 2.0, splines::Interpolation::Step(1.0)),
            splines::Key::new(f64::MAX, 2.0, splines::Interpolation::Linear),
        ];

        Self {
            perlin: Perlin::new(1),
            continentalness_scale: Vector2::new(0.002, 0.002),
            continentalness_spline: splines::Spline::from_vec(cont_keys),

            biome_table: [Biome::Forest, Biome::Plains, Biome::Hills],
            cont_map_spline: splines::Spline::from_vec(biome_cont_map),

            world_features: HashMap::new(),
        }
    }
}

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum Biome {
    Plains,
    Hills,
    Forest,
    Desert,
}

impl TerrainGenConfig {
    pub(crate) fn get_perlin(&self, offset: [f64; 2]) -> f64 {
        self.perlin.get(offset)
    }

    fn get_continentalness(&self, offset: [f64; 2]) -> f64 {
        let cont_sample = self.perlin.get([
            offset[0] * self.continentalness_scale.x,
            offset[1] * self.continentalness_scale.y,
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
        32.0 * surface_noise * continentalness + 16.0 + 16.0 * continentalness
    }

    pub(crate) fn load_features(&mut self, features_json: &'static str) {
        let features = json::parse(features_json).unwrap();
        for (feature_name, feature) in features.entries() {
            let dimensions = &feature["feature_dimensions"];
            let dimensions = (
                dimensions[0].as_usize().unwrap(),
                dimensions[1].as_usize().unwrap(),
                dimensions[2].as_usize().unwrap(),
            );
            let mut feature_vec: Vec<Vec<Vec<usize>>> = vec![];

            let feature_blocks = &feature["block_data"];
            for y in 0..dimensions.1 {
                let feature_slice_y = &feature_blocks[y];
                let mut z_vec = Vec::new();
                for z in 0..dimensions.2 {
                    let feature_slice_z = &feature_slice_y[z];
                    let mut x_vec = Vec::new();
                    for x in 0..dimensions.0 {
                        let feature_block = feature_slice_z[x].as_usize().unwrap();
                        x_vec.push(feature_block);
                    }
                    z_vec.push(x_vec);
                }
                feature_vec.push(z_vec);
            }
            self.world_features
                .insert(feature_name.to_string(), feature_vec);
        }
    }

    fn get_feature_blueprint(&self, feature_name: &str) -> Option<&Vec<Vec<Vec<usize>>>> {
        self.world_features.get(feature_name)
    }
}

pub(crate) mod terraingen {
    use super::{Biome, TerrainGenConfig};
    use crate::terrain::{
        block::block_index_by_name,
        chunk::{Chunk, CHUNK_WIDTH},
        BlockIndex, BlockWorldPos, ChunkIndex,
    };

    pub fn generate_surface(
        chunk_index: &ChunkIndex,
        chunk: &mut Chunk,
        noise_config: &TerrainGenConfig,
    ) -> Vec<(BlockWorldPos, usize)> {
        shape_terrain(chunk_index, chunk, noise_config);
        place_biomes(chunk_index, chunk, noise_config);

        enqueue_features(chunk_index, noise_config)
    }

    fn shape_terrain(chunk_index: &ChunkIndex, chunk: &mut Chunk, noise_config: &TerrainGenConfig) {
        for block_x in 0..CHUNK_WIDTH {
            for block_z in 0..CHUNK_WIDTH {
                let global_coords = BlockWorldPos::new(chunk_index.x, 0, chunk_index.y) * 16
                    + BlockWorldPos::new(block_x as isize, 0, block_z as isize);
                let surface =
                    noise_config.get_surface([global_coords.x as f64, global_coords.z as f64]);
                for block_y in 0..=surface.round() as usize {
                    let block_index = BlockIndex::new(block_x, block_y, block_z);
                    chunk.set_block(&block_index, block_index_by_name("Stone"));
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
                    Biome::Plains | Biome::Hills | Biome::Forest => {
                        for block_y in surface - 1..surface {
                            let block_index = BlockIndex::new(block_x, block_y, block_z);
                            chunk.set_block(&block_index, block_index_by_name("Dirt"));
                        }
                        let block_index = BlockIndex::new(block_x, surface, block_z);
                        chunk.set_block(&block_index, block_index_by_name("Grass"));
                    }
                    Biome::Desert => {
                        for block_y in surface - 1..=surface {
                            let block_index = BlockIndex::new(block_x, block_y, block_z);
                            chunk.set_block(&block_index, block_index_by_name("Sand"));
                        }
                    }
                }
            }
        }
    }

    fn enqueue_features(
        chunk_index: &ChunkIndex,
        terrain_config: &TerrainGenConfig,
    ) -> Vec<(BlockWorldPos, usize)> {
        let mut placement_queue = Vec::new();

        for block_x in 0..CHUNK_WIDTH {
            for block_z in 0..CHUNK_WIDTH {
                let global_coords = [
                    chunk_index.x as f64 * 16.0 + block_x as f64,
                    chunk_index.y as f64 * 16.0 + block_z as f64,
                ];
                let surface = terrain_config.get_surface(global_coords).round() as usize;
                let global_index = BlockWorldPos::new(
                    global_coords[0] as isize,
                    surface as isize,
                    global_coords[1] as isize,
                );

                let biome = terrain_config.get_biome(global_coords);
                match biome {
                    Biome::Forest => {
                        let has_grass: u8 = rand::random();
                        match has_grass {
                            0..=63 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "short_grass",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            64..=65 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "rose",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            66..=67 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "dandelion",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            68 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "oak_tree",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            _ => {}
                        }
                    }
                    Biome::Plains => {
                        let has_grass: u8 = rand::random();
                        match has_grass {
                            0..=63 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "short_grass",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            64..=65 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "rose",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            66..=75 => instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "dandelion",
                                terrain_config,
                                &mut placement_queue,
                            ),
                            _ => {}
                        }
                    }
                    Biome::Hills => {
                        let has_grass: u8 = rand::random();
                        if let 0..=16 = has_grass {
                            instantiate_feature(
                                &(global_index + BlockWorldPos::new(0, 1, 0)),
                                "short_grass",
                                terrain_config,
                                &mut placement_queue,
                            );
                        }
                    }
                    Biome::Desert => {}
                }
            }
        }

        placement_queue
    }

    fn instantiate_feature(
        world_position: &BlockWorldPos,
        feature_name: &str,
        terrain_config: &TerrainGenConfig,
        placement_queue: &mut Vec<(BlockWorldPos, usize)>,
    ) {
        if let Some(feature) = terrain_config.get_feature_blueprint(feature_name) {
            let y_len = feature.len();
            for (y, slice) in feature.iter().enumerate().take(y_len) {
                let z_len = slice.len();
                for (z, row) in slice.iter().enumerate().take(z_len) {
                    let x_len = row.len();
                    for (x, block_id) in row.iter().enumerate().take(x_len) {
                        if *block_id != 0 {
                            let block_world_pos = world_position
                                + BlockWorldPos::new(x as isize, y as isize, z as isize);
                            placement_queue.push((block_world_pos, *block_id));
                        }
                    }
                }
            }
        }
    }
}

impl Terrain {
    pub(crate) fn init_worldgen(
        &mut self,
        start_position: &Vector3<f32>,
        chunk_radius: isize,
        noise_config: &TerrainGenConfig,
    ) {
        let start_chunk_index = ChunkIndex::new(
            start_position.x.floor() as isize / CHUNK_WIDTH as isize,
            start_position.z.floor() as isize / CHUNK_WIDTH as isize,
        );

        for chunk_x in -chunk_radius..chunk_radius {
            for chunk_z in -chunk_radius..chunk_radius {
                let chunk_index = start_chunk_index + ChunkIndex::new(chunk_x, chunk_z);
                if self.chunks.at_index(&chunk_index).is_none() {
                    let mut cur_chunk = Box::new(Chunk::new());
                    let placement_queue =
                        terraingen::generate_surface(&chunk_index, &mut cur_chunk, noise_config);
                    self.chunks.insert(&chunk_index, cur_chunk);
                    self.place_features(placement_queue);
                }
            }
        }
    }

    pub(crate) fn place_features(&mut self, feature_blocks: Vec<(BlockWorldPos, usize)>) {
        // Update the placement queue with blocks that are part of the new feature
        for (world_pos, block_id) in feature_blocks {
            if let Some((chunk_index, block_index)) = Terrain::chunk_and_block_index(&world_pos) {
                if let Some(chunk) = self.chunks.at_index_mut(&chunk_index) {
                    // Place the block in the chunk if it exists
                    chunk.set_block(&block_index, block_id);
                } else {
                    // If the chunk does not yet exist, place the block into the placement queue
                    if let Some(block_vec) = self.block_placement_queue.get_mut(&chunk_index) {
                        block_vec.push((block_index, block_id));
                    } else {
                        self.block_placement_queue
                            .insert(chunk_index, vec![(block_index, block_id)]);
                    }
                }
            }
        }

        // Place all blocks in the placement queue which have a corresponding chunk
        self.block_placement_queue.retain(|key, blocks_queue| {
            if let Some(chunk) = self.chunks.at_index_mut(key) {
                for (block_index, block_id) in blocks_queue {
                    chunk.set_block(block_index, *block_id);
                }
                false
            } else {
                true
            }
        });
    }
}
