use super::BlockIndex;

pub(crate) const CHUNK_WIDTH: usize = 16;
pub(crate) const CHUNK_HEIGHT: usize = 256;

pub(crate) type BlockDataArray = [[[usize; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];


pub struct Chunk {
    blocks: BlockDataArray,
    metadata: BlockDataArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            metadata: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
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
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
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
    }
}
