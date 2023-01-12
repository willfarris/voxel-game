

use super::{BlockIndex};

pub(crate) const CHUNK_WIDTH: usize = 16;
pub(crate) const CHUNK_HEIGHT: usize = 256;

pub(crate) type BlockDataArray = [[[usize; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

#[derive(Clone)]
pub struct Chunk {
    pub blocks: BlockDataArray,
    pub metadata: BlockDataArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            metadata: [[[0; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }

    pub fn block_in_chunk(&self, position: &BlockIndex) -> usize {
        self.blocks[position.x][position.y][position.z] as usize
    }
}
