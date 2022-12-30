use cgmath::{Vector3, Matrix4};

use crate::graphics::{texture::Texture, shader::Shader, vertex::Vertex3D, resources::GLRenderable, buffer::BufferObject};

pub(crate) const CHUNK_SIZE: usize = 16;

pub enum ChunkMeshState {
    Uninit,
    Valid,
}

pub struct Chunk {
    position: Vector3<f32>,
    pub blocks: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub metadata: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub mesh_state: ChunkMeshState,
}

impl Chunk {
    pub fn new(position: Vector3<f32>) -> Self {
        Self {
            position,
            blocks: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            metadata: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            mesh_state: ChunkMeshState::Uninit,
        }
    }

    pub fn block_at_chunk_pos(&self, position: &Vector3<usize>) -> usize {
        self.blocks[position.x][position.y][position.z] as usize
    }
}
