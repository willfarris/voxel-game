use super::{BlockIndex, ChunkIndex};

pub(crate) const CHUNK_WIDTH: usize = 16;
pub(crate) const CHUNK_HEIGHT: usize = 256;

pub(crate) type BlockDataArray<T> = [[[T; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];


#[derive(Copy, Clone, Debug)]
pub(crate) struct ChunkUpdateInner {
    updated_chunk: ChunkIndex,
    updated_block: BlockIndex,
    new_block_id: usize,
}

impl ChunkUpdateInner {
    pub fn new(updated_chunk: ChunkIndex, updated_block: BlockIndex, new_block_id: usize) -> Self {
        Self {
            updated_chunk,
            updated_block,
            new_block_id,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ChunkUpdate {
    BlockUpdate(ChunkUpdateInner),
    NeighborChanged(ChunkUpdateInner),
    Generated,
    NoUpdate,
}

#[derive(Clone)]
pub struct Chunk {
    blocks: BlockDataArray<usize>,
    metadata: BlockDataArray<usize>,
    lighting: BlockDataArray<usize>,

    pub next_update: ChunkUpdate,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            metadata: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            lighting: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            next_update: ChunkUpdate::NoUpdate,
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

    /*pub fn update_lighting(&mut self, pending_lights: Vec<(BlockIndex, usize)>) -> HashMap<ChunkIndex, Vec<(BlockIndex, usize)>>{
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

        // Add light spilling over from adjacent chunks
        for (index, light_val) in pending_lights {
            if BLOCKS[self.blocks[index.x][index.y][index.z]].transparent {
                self.lighting[index.x][index.y][index.z] = light_val;
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

        HashMap::new()
    }*/

    /* pub fn from_json_array(chunk_json: &JsonValue) -> Box<Self> {
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
    } */
}
