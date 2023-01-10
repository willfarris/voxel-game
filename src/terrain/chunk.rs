use cgmath::Vector3;

use super::BlockIndex;

pub(crate) const CHUNK_SIZE: usize = 16;

pub(crate) type BlockDataArray = [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

#[derive(Clone)]
pub struct Chunk {
    pub blocks: BlockDataArray,
    pub metadata: BlockDataArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            metadata: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn block_in_chunk(&self, position: &BlockIndex) -> usize {
        self.blocks[position.x][position.y][position.z] as usize
    }
}
