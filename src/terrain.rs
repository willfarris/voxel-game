use std::{collections::HashMap, sync::{Arc, RwLock}};

use cgmath::{Matrix4, Vector2, Vector3};
use image::ImageFormat;

use crate::graphics::{
    mesh::push_face,
    resources::{GLRenderable, GLResources},
    shader::Shader,
    source::{TERRAIN_BITMAP, TERRAIN_FRAG_SRC, TERRAIN_VERT_SRC},
    texture::Texture,
    uniform::Uniform,
    vertex::Vertex3D,
};

use self::{
    block::{MeshType, BLOCKS},
    chunk::{Chunk, CHUNK_HEIGHT, CHUNK_WIDTH}, generation::TerrainGenConfig,
};

pub(crate) mod block;
pub(crate) mod chunk;
pub(crate) mod generation;
mod save;

pub type BlockWorldPos = Vector3<isize>;
pub type ChunkIndex = Vector2<isize>;
pub type BlockIndex = Vector3<usize>;

pub enum TerrainEvent {
    LoadingZones(Vec<ChunkIndex>),
    ModifyBlock(BlockWorldPos, usize),
}

const NUM_CHUNK_LISTS: usize = 2;
type ChunkList = [HashMap<ChunkIndex, Arc<RwLock<Box<Chunk>>>>; NUM_CHUNK_LISTS];

pub struct Terrain {
    /* Multi-level queue for chunk data
     * 0: chunks in view to be actively updated/drawn each tick/frame
     * 1: chunks in RAM but inactive, out of render distance
     */
    chunks: ChunkList,

    block_placement_queue: HashMap<ChunkIndex, Vec<(BlockIndex, usize)>>,
    chunk_update_queue: Vec<ChunkIndex>,

    event_queue: Vec<TerrainEvent>,

    config: TerrainGenConfig,
}

trait ChunkListTrait {
    fn at_index(&self, index: &ChunkIndex) -> Option<&Arc<RwLock<Box<Chunk>>>>;
    fn at_index_mut(&mut self, index: &ChunkIndex) -> Option<&mut Arc<RwLock<Box<Chunk>>>>;
    fn insert(&mut self, index: &ChunkIndex, chunk: Arc<RwLock<Box<Chunk>>>);
}

impl ChunkListTrait for ChunkList {

    fn at_index(&self, index: &ChunkIndex) -> Option<&Arc<RwLock<Box<Chunk>>>> {
        let priority_levels = self.len();
        for p in 0..priority_levels {
            if let Some(chunk) = self[p].get(index) {
                return Some(chunk);
            }
        }
        None
    }

    fn at_index_mut(&mut self, index: &ChunkIndex) -> Option<&mut Arc<RwLock<Box<Chunk>>>> {
        let priority_levels = self.len();
        let mut i = priority_levels;
        for p in 0..priority_levels {
            if self[p].get_mut(index).is_some() {
                i = p;
            } else {
                break;
            }
        }
        if i == priority_levels {
            None
        } else {
            self[i].get_mut(index)
        }
    }

    fn insert(&mut self, index: &ChunkIndex, chunk: Arc<RwLock<Box<Chunk>>>) {
        self[1].insert(*index, chunk);
    }
}


impl Terrain {
    pub fn new(config: TerrainGenConfig) -> Self {
        Self {
            chunks: [HashMap::new(), HashMap::new()],

            block_placement_queue: HashMap::new(),
            chunk_update_queue: Vec::new(),
            event_queue: Vec::new(),

            config,
        }
    }

    pub fn event(&mut self, event: TerrainEvent) {
        self.event_queue.push(event);
    }

    pub fn tick(&mut self) {
        while let Some(event) = self.event_queue.pop() {
            match event {
                TerrainEvent::LoadingZones(active_chunks) => {
                    
                    let [ref mut cur_visible, ref mut backburner] = self.chunks;
                    backburner.extend(cur_visible.drain());

                    let radius = 3;
                    assert!(radius > 0);
                    for chunk_index in active_chunks {
                        for x in -radius..=radius {
                            for z in -radius..=radius {
                                let chunk_index = chunk_index + Vector2::new(x, z);
                                if let Some(chunk) = backburner.remove(&chunk_index) {
                                    cur_visible.insert(chunk_index, chunk);
                                } else {
                                    self.chunk_update_queue.push(chunk_index);
                                }
                            }
                        }
                    }

                },
                TerrainEvent::ModifyBlock(block_world_pos, new_value) => {
                    if let Some((chunk_index, block_index)) = Self::chunk_and_block_index(&block_world_pos) {
                        if let Some(chunk) = self.chunks.at_index_mut(&chunk_index) {
                            let mut chunk = chunk.write().unwrap();
                            chunk.set_block(&block_index, new_value);
                        }
                    }
                },
            }
        }
    }

    /// Fetch the ID of the block at the global position `world_pos`
    pub fn block_at_world_pos(&self, world_pos: &BlockWorldPos) -> usize {
        if let Some((chunk_index, block_index)) = Terrain::chunk_and_block_index(world_pos) {
            if let Some(chunk) = self.chunks.at_index(&chunk_index) {
                let chunk = chunk.read().unwrap();
                return chunk.get_block(&block_index)
            }
            0
        } else {
            0
        }
    }

    /// Convert from world coordinates to chunk indices, and the block index within the chunk
    pub fn chunk_and_block_index(world_pos: &BlockWorldPos) -> Option<(ChunkIndex, BlockIndex)> {
        if world_pos.y > (CHUNK_HEIGHT - 1) as isize {
            None
        } else {
            let chunk_index = ChunkIndex {
                x: (world_pos.x as f32 / CHUNK_WIDTH as f32).floor() as isize,
                y: (world_pos.z as f32 / CHUNK_WIDTH as f32).floor() as isize,
            };
            let block_index = Vector3 {
                x: (world_pos.x.rem_euclid(CHUNK_WIDTH as isize)) as usize,
                y: (world_pos.y.rem_euclid(CHUNK_HEIGHT as isize)) as usize,
                z: (world_pos.z.rem_euclid(CHUNK_WIDTH as isize)) as usize,
            };
            Some((chunk_index, block_index))
        }
    }

    /// Generate an array of vertices to pass along to a VBO for drawing
    pub(crate) fn generate_chunk_vertices(
        &self,
        chunk_index: &ChunkIndex,
    ) -> Option<Vec<Vertex3D>> {
        if let Some(chunk) = self.chunks.at_index(chunk_index) {
            let chunk = chunk.read().unwrap();
            let x_pos_chunk = self.chunks.at_index(&(chunk_index + ChunkIndex::new(1, 0)));
            let x_neg_chunk = self.chunks.at_index(&(chunk_index + ChunkIndex::new(-1, 0)));
            let z_pos_chunk = self.chunks.at_index(&(chunk_index + ChunkIndex::new(0, 1)));
            let z_neg_chunk = self.chunks.at_index(&(chunk_index + ChunkIndex::new(0, -1)));

            let mut vertices = Vec::new();
            for x in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    for z in 0..CHUNK_WIDTH {
                        let block_index = BlockIndex::new(x, y, z);
                        let i = chunk.get_block(&block_index);
                        if i == 0 {
                            continue;
                        }
                        let cur = &block::BLOCKS[i];
                        let tex_coords: [(f32, f32); 6] =
                            if let Some(texture_type) = &cur.texture_map {
                                let mut coords = [(0.0f32, 0.0f32); 6];
                                match texture_type {
                                    block::TextureType::Single(x, y) => {
                                        for item in &mut coords {
                                            *item = (*x, *y);
                                        }
                                    }
                                    block::TextureType::TopAndSide(
                                        (x_top, y_top),
                                        (x_side, y_side),
                                    ) => {
                                        coords[0] = (*x_side, *y_side);
                                        coords[1] = (*x_side, *y_side);
                                        coords[2] = (*x_top, *y_top);
                                        coords[3] = (*x_side, *y_side);
                                        coords[4] = (*x_side, *y_side);
                                        coords[5] = (*x_side, *y_side);
                                    }
                                    block::TextureType::TopSideBottom(
                                        (x_top, y_top),
                                        (x_side, y_side),
                                        (x_bottom, y_bottom),
                                    ) => {
                                        coords[0] = (*x_side, *y_side);
                                        coords[1] = (*x_side, *y_side);
                                        coords[2] = (*x_top, *y_top);
                                        coords[3] = (*x_bottom, *y_bottom);
                                        coords[4] = (*x_side, *y_side);
                                        coords[5] = (*x_side, *y_side);
                                    }
                                    block::TextureType::TopSideFrontActivatable(
                                        (x_front_inactive, y_front_inactive),
                                        (x_front_active, y_front_active),
                                        (x_side, y_side),
                                        (x_top, y_top),
                                    ) => {
                                        coords[0] = (*x_side, *y_side);
                                        coords[1] = (*x_side, *y_side);
                                        coords[2] = (*x_top, *y_top);
                                        coords[3] = (*x_top, *y_top);
                                        coords[4] = (*x_side, *y_side);
                                        //let active = chunk.metadata[x][y][z] == 1;
                                        let active = chunk.get_metadata(&block_index) == 1;
                                        coords[5] = if active {
                                            (*x_front_active, *y_front_active)
                                        } else {
                                            (*x_front_inactive, *y_front_inactive)
                                        };
                                    }
                                }
                                coords
                            } else {
                                [(0.0, 0.0); 6]
                            };

                        let position = [x as f32, y as f32, z as f32];
                        let vertex_type = cur.block_type as i32;
                        match cur.mesh_type {
                            MeshType::Block => {
                                let (x_pos_block, x_pos_lighting) = if x < CHUNK_WIDTH - 1 {
                                    let x_pos_index = BlockIndex::new(x + 1, y, z);
                                    (
                                        Some(BLOCKS[chunk.get_block(&x_pos_index)]),
                                        chunk.get_lighting(&x_pos_index),
                                    )
                                } else {
                                    let x_pos_index = BlockIndex::new(0, y, z);
                                    x_pos_chunk
                                        .map(|adjacent_chunk| {
                                            let adjacent_chunk = adjacent_chunk.read().unwrap();
                                            (
                                                Some(
                                                    BLOCKS[adjacent_chunk.get_block(&x_pos_index)],
                                                ),
                                                adjacent_chunk.get_lighting(&x_pos_index),
                                            )
                                        })
                                        .unwrap_or((None, 0))
                                };
                                if let Some(adjacent_block) = x_pos_block {
                                    if adjacent_block.transparent {
                                        push_face(
                                            &position,
                                            0,
                                            &mut vertices,
                                            &tex_coords[0],
                                            vertex_type,
                                            x_pos_lighting as f32,
                                        );
                                    }
                                }

                                let (x_neg_block, x_neg_lighting) = if x > 0 {
                                    let x_neg_index = Vector3::new(x - 1, y, z);
                                    (
                                        Some(BLOCKS[chunk.get_block(&x_neg_index)]),
                                        chunk.get_lighting(&x_neg_index),
                                    )
                                } else {
                                    let x_neg_index = Vector3::new(CHUNK_WIDTH - 1, y, z);
                                    x_neg_chunk
                                        .map(|adjacent_chunk| {
                                            let adjacent_chunk = adjacent_chunk.read().unwrap();
                                            (
                                                Some(
                                                    BLOCKS[adjacent_chunk.get_block(&x_neg_index)],
                                                ),
                                                adjacent_chunk.get_lighting(&x_neg_index),
                                            )
                                        })
                                        .unwrap_or((None, 0))
                                };
                                if let Some(adjacent_block) = x_neg_block {
                                    if adjacent_block.transparent {
                                        push_face(
                                            &position,
                                            1,
                                            &mut vertices,
                                            &tex_coords[1],
                                            vertex_type,
                                            x_neg_lighting as f32,
                                        );
                                    }
                                }

                                let (y_pos_block, y_pos_lighting) = if y < CHUNK_HEIGHT - 1 {
                                    let y_pos_index = Vector3::new(x, y + 1, z);
                                    (
                                        Some(BLOCKS[chunk.get_block(&y_pos_index)]),
                                        chunk.get_lighting(&y_pos_index),
                                    )
                                } else {
                                    (None, 0)
                                };
                                if let Some(adjacent_block) = y_pos_block {
                                    if adjacent_block.transparent {
                                        push_face(
                                            &position,
                                            2,
                                            &mut vertices,
                                            &tex_coords[2],
                                            vertex_type,
                                            y_pos_lighting as f32,
                                        );
                                    }
                                }

                                let (y_neg_block, y_neg_lighting) = if y > 0 {
                                    let y_neg_index = Vector3::new(x, y - 1, z);
                                    (
                                        Some(BLOCKS[chunk.get_block(&y_neg_index)]),
                                        chunk.get_lighting(&y_neg_index),
                                    )
                                } else {
                                    (None, 0)
                                };
                                if let Some(adjacent_block) = y_neg_block {
                                    if adjacent_block.transparent {
                                        push_face(
                                            &position,
                                            3,
                                            &mut vertices,
                                            &tex_coords[3],
                                            vertex_type,
                                            y_neg_lighting as f32,
                                        );
                                    }
                                }

                                let (z_pos_block, z_pos_lighting) = if z < CHUNK_WIDTH - 1 {
                                    let z_pos_index = Vector3::new(x, y, z + 1);
                                    (
                                        Some(BLOCKS[chunk.get_block(&z_pos_index)]),
                                        chunk.get_lighting(&z_pos_index),
                                    )
                                } else {
                                    let z_pos_index = Vector3::new(x, y, 0);
                                    z_pos_chunk
                                        .map(|adjacent_chunk| {
                                            let adjacent_chunk = adjacent_chunk.read().unwrap();
                                            (
                                                Some(
                                                    BLOCKS[adjacent_chunk.get_block(&z_pos_index)],
                                                ),
                                                adjacent_chunk.get_lighting(&z_pos_index),
                                            )
                                        })
                                        .unwrap_or((None, 16))
                                };
                                if let Some(adjacent_block) = z_pos_block {
                                    if adjacent_block.transparent {
                                        push_face(
                                            &position,
                                            4,
                                            &mut vertices,
                                            &tex_coords[4],
                                            vertex_type,
                                            z_pos_lighting as f32,
                                        );
                                    }
                                }

                                let (z_neg_index, z_neg_lighting) = if z > 0 {
                                    let z_neg_index = Vector3::new(x, y, z - 1);
                                    (
                                        Some(BLOCKS[chunk.get_block(&z_neg_index)]),
                                        chunk.get_lighting(&z_neg_index),
                                    )
                                } else {
                                    let z_neg_index = Vector3::new(x, y, CHUNK_WIDTH - 1);
                                    z_neg_chunk
                                        .map(|adjacent_chunk| {
                                            let adjacent_chunk = adjacent_chunk.read().unwrap();
                                            (
                                                Some(
                                                    BLOCKS[adjacent_chunk.get_block(&z_neg_index)],
                                                ),
                                                adjacent_chunk.get_lighting(&z_neg_index),
                                            )
                                        })
                                        .unwrap_or((None, 0))
                                };
                                if let Some(adjacent_block) = z_neg_index {
                                    if adjacent_block.transparent {
                                        push_face(
                                            &position,
                                            5,
                                            &mut vertices,
                                            &tex_coords[5],
                                            vertex_type,
                                            z_neg_lighting as f32,
                                        );
                                    }
                                }
                            }
                            MeshType::CrossedPlanes => {
                                let lighting = chunk.get_lighting(&block_index) as f32;
                                push_face(
                                    &position,
                                    6,
                                    &mut vertices,
                                    &tex_coords[0],
                                    vertex_type,
                                    lighting,
                                );
                                push_face(
                                    &position,
                                    7,
                                    &mut vertices,
                                    &tex_coords[0],
                                    vertex_type,
                                    lighting,
                                );
                                push_face(
                                    &position,
                                    8,
                                    &mut vertices,
                                    &tex_coords[0],
                                    vertex_type,
                                    lighting,
                                );
                                push_face(
                                    &position,
                                    9,
                                    &mut vertices,
                                    &tex_coords[0],
                                    vertex_type,
                                    lighting,
                                );
                            }
                        }
                    }
                }
            }

            if vertices.is_empty() {
                None
            } else {
                Some(vertices)
            }
        } else {
            None
        }
    }

    pub fn collision_at_world_pos(&self, world_pos: &BlockWorldPos) -> bool {
        if let Some((chunk_index, block_index)) = Terrain::chunk_and_block_index(world_pos) {
            if let Some(chunk) = self.chunks.at_index(&chunk_index) {
                let chunk = chunk.read().unwrap();
                chunk.get_block(&block_index) != 0
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn update_single_chunk_mesh(
        &mut self,
        chunk_index: &ChunkIndex,
        gl_resources: &mut GLResources,
    ) {
        if self.chunks.at_index(chunk_index).is_some() {
            if let Some(chunk_vertices) = self.generate_chunk_vertices(chunk_index) {
                let name = format!("chunk_{}_{}", chunk_index.x, chunk_index.y);
                let verts = Box::new(chunk_vertices);
                gl_resources.update_vao_buffer(name, verts);
                for p in 0..self.chunks.len() {
                    if let Some(chunk) = self.chunks[p].get_mut(chunk_index) {
                        let mut chunk = chunk.write().unwrap();
                        chunk.needs_mesh_rebuild = false;
                    }
                }
            }
        }
    }

    pub(crate) fn update_chunk_mesh(
        &mut self,
        chunk_index: &ChunkIndex,
        gl_resources: &mut GLResources,
    ) {
        let x_pos = chunk_index + ChunkIndex::new(1, 0);
        let x_neg = chunk_index + ChunkIndex::new(-1, 0);
        let z_pos = chunk_index + ChunkIndex::new(0, 1);
        let z_neg = chunk_index + ChunkIndex::new(0, -1);

        self.update_single_chunk_mesh(chunk_index, gl_resources);
        self.update_single_chunk_mesh(&x_pos, gl_resources);
        self.update_single_chunk_mesh(&x_neg, gl_resources);
        self.update_single_chunk_mesh(&z_pos, gl_resources);
        self.update_single_chunk_mesh(&z_neg, gl_resources);
    }


    pub fn insert_chunk(&mut self, chunk_index: ChunkIndex, chunk: Arc<RwLock<Box<Chunk>>>) {
        self.chunks.insert(&chunk_index, chunk);
    }

    pub fn solid_block_at_world_pos(&self, world_pos: &BlockWorldPos) -> bool {
        BLOCKS[self.block_at_world_pos(world_pos)].solid
    }

    pub fn update_meshes(&mut self, gl_resources: &mut GLResources) {
        let mut rebuild = Vec::new();
        let mut rebuild_count = 0;
        for (index, chunk) in self.chunks[0].iter_mut() {
            let chunk = chunk.read().unwrap();
            if chunk.needs_mesh_rebuild {
                rebuild.push(*index);
                rebuild_count += 1;
            }
            if rebuild_count > 5 {
                break;
            }
        }

        for index in rebuild {
            self.update_chunk_mesh(&index, gl_resources);
        }
    }

    pub fn needs_regen(&mut self) -> Vec<ChunkIndex> {
        let queue = self.chunk_update_queue.clone();
        self.chunk_update_queue.clear();
        queue
    }

    pub fn terrain_config(&self) -> &TerrainGenConfig {
        &self.config
    }
}

impl GLRenderable for Terrain {
    fn init_gl_resources(&self, gl_resources: &mut GLResources) {
        // Texture is also used by drops and may already exist
        if gl_resources.get_texture("terrain").is_none() {
            let terrain_texture =
                Texture::from_dynamic_image_bytes(TERRAIN_BITMAP, ImageFormat::Png);
            gl_resources.add_texture("terrain", terrain_texture);
        }

        let terrain_program = Shader::new(TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC).unwrap();
        gl_resources.add_shader("terrain", terrain_program);

        for chunk_index in self.chunks[0].keys() {
            if let Some(chunk_vertices) = self.generate_chunk_vertices(chunk_index) {
                let name = format!("chunk_{}_{}", chunk_index.x, chunk_index.y);
                let verts = Box::new(chunk_vertices);
                gl_resources.create_or_update_vao(name, verts);
            }
        }
    }

    fn draw(&self, gl_resources: &GLResources, uniforms: &[(&str, Box<dyn Uniform>)]) {
        let shader = gl_resources.get_shader("terrain").unwrap();
        let texture = gl_resources.get_texture("terrain").unwrap();

        texture.use_as_framebuffer_texture(0);

        shader.use_program();

        for (name, uniform) in uniforms {
            uniform.set_as_uniform(shader, name);
        }

        shader.set_texture(unsafe { c_str!("texture_map") }, 0);

        for chunk_index in self.chunks[0].keys() {
            let model_matrix = Matrix4::from_translation(Vector3::new(
                (chunk_index.x * CHUNK_WIDTH as isize) as f32,
                0f32,
                (chunk_index.y * CHUNK_WIDTH as isize) as f32,
            ));
            shader.set_mat4(unsafe { c_str!("model_matrix") }, &model_matrix);

            let name = format!("chunk_{}_{}", chunk_index.x, chunk_index.y);
            if let Some(vao) = gl_resources.get_vao(&name) {
                vao.draw();
            }
        }
    }
}
