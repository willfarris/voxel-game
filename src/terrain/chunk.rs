use cgmath::Vector3;
use json::JsonValue;

use super::{BlockIndex, save::save_chunk_data_to_json, block::BLOCKS};

pub(crate) const CHUNK_WIDTH: usize = 16;
pub(crate) const CHUNK_HEIGHT: usize = 256;

pub(crate) type BlockDataArray<T> = [[[T; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];


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
                       self.lighting[x][y][z] = 0xF; 
                    } else {
                        break 'skylight;
                    }
                }
            }
        }

        // TODO: Update lighting from blocks
        /*for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    //TODO: set lighting for lit blocks here
                    let block_id = self.blocks[x][y][z];
                    if BLOCKS[block_id].emissive {
                        self.lighting[x][y][z] = 0xF;
                    }
                }
            }
        }*/

        // Flood fill lighting
        //  fn flood_fill(lighting_array, block_index)
        //      if lighting_array[block_index] > 0
        //          light_level = max(lighting_array[block_index] - 1, 0)
        //          for each adjacent
        //              add index to visited
        //              if light_level > adjacent && adjacent is not visited
        //                  lighting_array[adjacent] = light_level
        //                  flood_fill(lighting_array, adjecent)
        //

        fn flood_fill(lighting_array: &mut BlockDataArray<usize>, visited: &mut BlockDataArray<bool>, block_index: &BlockIndex) {
            let light_level = lighting_array[block_index.x][block_index.y][block_index.z];
            if light_level > 0 && !visited[block_index.x][block_index.y][block_index.z] {
                visited[block_index.x][block_index.y][block_index.z] = true;
                let light_level = (light_level - 1).max(0);
                if block_index.x > 0 {

                } else if block_index.x >= CHUNK_WIDTH {

                }

                if block_index.y > 0 {

                } else if block_index.y >= CHUNK_HEIGHT {

                }

                if block_index.z > 0 {

                } else if block_index.z >= CHUNK_WIDTH {

                }
            }
        }


        // recursively spread that light until the level reaches zero or the max light
        let mut visited = [[[false; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    if self.lighting[x][y][z] > 0 && self.lighting[x][y][z] < 0x10 {
                        let adjacent = vec![
                            Vector3::new(x+1, y, z),
                            Vector3::new(x-1, y, z),
                            Vector3::new(x, y+1, z),
                            Vector3::new(x, y-1, z),
                            Vector3::new(x, y, z+1),
                            Vector3::new(x, y, z-1)
                        ];

                        if x > 0 {

                        }
                    }
                }
            }
        }

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
