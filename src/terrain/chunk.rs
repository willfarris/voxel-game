use json::JsonValue;

use super::{BlockIndex, save::save_chunk_data_to_json, block::BLOCKS};

pub(crate) const CHUNK_WIDTH: usize = 16;
pub(crate) const CHUNK_HEIGHT: usize = 256;

pub(crate) type BlockDataArray = [[[usize; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];


pub struct Chunk {
    blocks: BlockDataArray,
    metadata: BlockDataArray,
    lighting: BlockDataArray,
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

    pub fn set_block(&mut self, block_index: &BlockIndex, block_id: usize) {
        self.blocks[block_index.x][block_index.y][block_index.z] = block_id;
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
                            let lower_block = if y > 0 {self.blocks[x][y-1][z]} else {0};
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

        // Reset lighting array
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    self.lighting[x][y][z] = 0;
                }
            }
        }

        // Update lighting from sky
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                'skylight: for i in 1..=CHUNK_HEIGHT {
                    let y = CHUNK_HEIGHT-i;
                    let block_id = self.blocks[x][y][z];
                    if BLOCKS[block_id].transparent {
                       self.lighting[x][y][z] = 16; 
                    } else {
                        break 'skylight;
                    }
                }
            }
        }

        // Update lighting from blocks
        for _x in 0..CHUNK_WIDTH {
            for _y in 0..CHUNK_HEIGHT {
                for _z in 0..CHUNK_WIDTH {
                    //TODO: set lighting for lit blocks here
                    
                }
            }
        }

        // TODO: Flood fill light
        //let mut light_map = [[[0usize; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

    }

    pub fn from_json_array(chunk_json: &JsonValue) -> Box<Self> {
        let mut chunk = Box::new(Self::new());

        for (x, row) in chunk_json.members().enumerate() {
            for (y, column) in row.members().enumerate() {
                for (z, block) in column.members().enumerate() {
                    let block_id = block.as_usize().unwrap();
                    chunk.blocks[x][y][z] = block_id;
                }
            }
        }

        chunk
    }

    pub fn to_json_array(&self) -> JsonValue {
        // TODO: Save block metadata
        save_chunk_data_to_json(&self.blocks)
    }
}
