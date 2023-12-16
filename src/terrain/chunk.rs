use cgmath::Vector3;
use json::{object, JsonValue};

use super::{block::BLOCKS, save::save_chunk_data_to_json, BlockIndex};

pub(crate) const CHUNK_WIDTH: usize = 16;
pub(crate) const CHUNK_HEIGHT: usize = 256;

pub(crate) type BlockDataArray<T> = [[[T; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

#[derive(Clone)]
pub struct Chunk {
    blocks: BlockDataArray<usize>,
    metadata: BlockDataArray<usize>,
    lighting: BlockDataArray<usize>,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            metadata: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            lighting: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }

    pub fn get_lighting(&self, block_index: &BlockIndex) -> usize {
        self.lighting[block_index.x][block_index.y][block_index.z]
    }

    pub fn get_block(&self, block_index: &BlockIndex) -> usize {
        self.blocks[block_index.x][block_index.y][block_index.z]
    }

    pub fn set_block(&mut self, block_index: &BlockIndex, block_id: usize) -> usize {
        let prev_block_id = self.blocks[block_index.x][block_index.y][block_index.z];
        self.blocks[block_index.x][block_index.y][block_index.z] = block_id;
        prev_block_id
    }

    pub fn get_metadata(&self, block_index: &BlockIndex) -> usize {
        self.metadata[block_index.x][block_index.y][block_index.z]
    }

    pub fn update(&mut self) {
        // Block physics
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    // Rules for realistic block behavior
                    let block_id = self.blocks[x][y][z];
                    match block_id {
                        // Remove things that grow on grass/dirt if they're not on grass/dirt
                        4 | 6 | 8 | 9 => {
                            let lower_block = if y > 0 { self.blocks[x][y - 1][z] } else { 0 };
                            match lower_block {
                                2 | 3 => {}
                                _ => {
                                    self.blocks[x][y][z] = 0;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        //self.update_lighting();
    }

    pub fn update_lighting(&mut self) {
        // Reset lighting
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    self.lighting[x][y][z] = 0;
                }
            }
        }

        // Update lighting from sky
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                'skylight: for i in 1..=CHUNK_HEIGHT {
                    let y = CHUNK_HEIGHT - i;
                    let block_id = self.blocks[x][y][z];
                    if BLOCKS[block_id].transparent {
                        self.lighting[x][y][z] = 0xF;
                    } else {
                        break 'skylight;
                    }
                }
            }
        }

        fn flood_fill(
            lighting_array: &mut BlockDataArray<usize>,
            visited: &mut BlockDataArray<bool>,
            block_array: &BlockDataArray<usize>,
            block_index: &BlockIndex,
        ) {
            let current_lighting = lighting_array[block_index.x][block_index.y][block_index.z];
            if current_lighting == 0 || visited[block_index.x][block_index.y][block_index.z] {
                return;
            }
            visited[block_index.x][block_index.y][block_index.z] = true;

            if block_index.y > 0 {
                let y_neg_index = block_index - Vector3::new(0, 1, 0);
                let light_can_spread =
                    BLOCKS[block_array[y_neg_index.x][y_neg_index.y][y_neg_index.z]].transparent;
                if light_can_spread
                    && lighting_array[y_neg_index.x][y_neg_index.y][y_neg_index.z]
                        < current_lighting
                {
                    lighting_array[y_neg_index.x][y_neg_index.y][y_neg_index.z] =
                        current_lighting - 1;
                    flood_fill(lighting_array, visited, block_array, &y_neg_index);
                } else {
                    visited[y_neg_index.x][y_neg_index.y][y_neg_index.z] = true;
                }
            }

            if block_index.x < CHUNK_WIDTH - 1 {
                let x_pos_index = block_index + Vector3::new(1, 0, 0);
                let light_can_spread =
                    BLOCKS[block_array[x_pos_index.x][x_pos_index.y][x_pos_index.z]].transparent;
                if light_can_spread
                    && lighting_array[x_pos_index.x][x_pos_index.y][x_pos_index.z]
                        < current_lighting
                {
                    lighting_array[x_pos_index.x][x_pos_index.y][x_pos_index.z] =
                        current_lighting - 1;
                    flood_fill(lighting_array, visited, block_array, &x_pos_index);
                } else {
                    visited[x_pos_index.x][x_pos_index.y][x_pos_index.z] = true;
                }
            }

            if block_index.x > 0 {
                let x_neg_index = block_index - Vector3::new(1, 0, 0);
                let light_can_spread =
                    BLOCKS[block_array[x_neg_index.x][x_neg_index.y][x_neg_index.z]].transparent;
                if light_can_spread
                    && lighting_array[x_neg_index.x][x_neg_index.y][x_neg_index.z]
                        < current_lighting
                {
                    lighting_array[x_neg_index.x][x_neg_index.y][x_neg_index.z] =
                        current_lighting - 1;
                    flood_fill(lighting_array, visited, block_array, &x_neg_index);
                } else {
                    visited[x_neg_index.x][x_neg_index.y][x_neg_index.z] = true;
                }
            }

            if block_index.y < CHUNK_HEIGHT - 1 {
                let y_pos_index = block_index + Vector3::new(0, 1, 0);
                let light_can_spread =
                    BLOCKS[block_array[y_pos_index.x][y_pos_index.y][y_pos_index.z]].transparent;
                if light_can_spread
                    && lighting_array[y_pos_index.x][y_pos_index.y][y_pos_index.z]
                        < current_lighting
                {
                    lighting_array[y_pos_index.x][y_pos_index.y][y_pos_index.z] =
                        current_lighting - 1;
                    flood_fill(lighting_array, visited, block_array, &y_pos_index);
                } else {
                    visited[y_pos_index.x][y_pos_index.y][y_pos_index.z] = true;
                }
            }

            if block_index.z < CHUNK_WIDTH - 1 {
                let z_pos_index = block_index + Vector3::new(0, 0, 1);
                let light_can_spread =
                    BLOCKS[block_array[z_pos_index.x][z_pos_index.y][z_pos_index.z]].transparent;
                if light_can_spread
                    && lighting_array[z_pos_index.x][z_pos_index.y][z_pos_index.z]
                        < current_lighting
                {
                    lighting_array[z_pos_index.x][z_pos_index.y][z_pos_index.z] =
                        current_lighting - 1;
                    flood_fill(lighting_array, visited, block_array, &z_pos_index);
                } else {
                    visited[z_pos_index.x][z_pos_index.y][z_pos_index.z] = true;
                }
            }

            if block_index.z > 0 {
                let z_neg_index = block_index - Vector3::new(0, 0, 1);
                let light_can_spread =
                    BLOCKS[block_array[z_neg_index.x][z_neg_index.y][z_neg_index.z]].transparent;
                if light_can_spread
                    && lighting_array[z_neg_index.x][z_neg_index.y][z_neg_index.z]
                        < current_lighting
                {
                    lighting_array[z_neg_index.x][z_neg_index.y][z_neg_index.z] =
                        current_lighting - 1;
                    flood_fill(lighting_array, visited, block_array, &z_neg_index);
                } else {
                    visited[z_neg_index.x][z_neg_index.y][z_neg_index.z] = true;
                }
            }
        }

        // recursively spread that light until the level reaches zero or the max light
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    let mut visited = [[[false; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];
                    flood_fill(
                        &mut self.lighting,
                        &mut visited,
                        &self.blocks,
                        &BlockIndex::new(x, y, z),
                    )
                }
            }
        }
    }

    pub fn from_json_array(chunk_json: &JsonValue) -> Box<Self> {
        let mut chunk = Box::new(Self::new());

        let chunk_blocks = &chunk_json["blocks"];
        for (x, row) in chunk_blocks.members().enumerate() {
            for (y, column) in row.members().enumerate() {
                for (z, block) in column.members().enumerate() {
                    let block_id = block.as_usize().unwrap();
                    chunk.blocks[x][y][z] = block_id;
                }
            }
        }

        let chunk_lighting = &chunk_json["lighting"];
        for (x, row) in chunk_lighting.members().enumerate() {
            for (y, column) in row.members().enumerate() {
                for (z, light_value) in column.members().enumerate() {
                    let light_value = light_value.as_usize().unwrap();
                    chunk.lighting[x][y][z] = light_value;
                }
            }
        }

        chunk
    }

    pub fn to_json_array(&self) -> JsonValue {
        object! {
            "blocks": save_chunk_data_to_json(&self.blocks),
            "lighting": save_chunk_data_to_json(&self.lighting)
        }
    }
}
