use std::{collections::{HashMap, LinkedList}, hash::Hash};

use cgmath::{Vector3, Matrix4, Vector2};
use noise::{Perlin, Seedable};

use crate::graphics::{shader::Shader, texture::Texture, resources::GLRenderable, vertex::Vertex3D};

use self::{chunk::{Chunk, CHUNK_SIZE, push_face}, block::{BLOCKS, MeshType, Block}};

mod chunk;
pub(crate) mod block;
mod terrain;

pub type BlockWorldPos = Vector3<isize>;
pub type ChunkIndex = Vector3<isize>;
pub type BlockIndex = Vector3<usize>; 

pub struct World {
    chunks: HashMap<Vector3<isize>, Chunk>,
    
    // OpenGL Resources
    shader_src: (&'static str, &'static str),
    texture_bitmap: &'static [u8],
    shader: Option<Shader>,
    texture: Option<Texture>,

    // World Generation Resources
    generation_queue: HashMap<Vector3<isize>, LinkedList<(Vector3<usize>, usize)>>,
    noise_offset: Vector2<f64>,
    noise_scale: f64,
    perlin: Perlin,
}

impl World {
    pub fn new(vertex_shader_src: &'static str, fragment_shader_src: &'static str, texture_bitmap: &'static [u8]) -> Self {
        let noise_scale = 0.02;
        let noise_offset = Vector2::new(
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
        );
        let perlin = Perlin::new();
        Self {
            chunks: HashMap::new(),

            shader_src: (vertex_shader_src, fragment_shader_src),
            texture_bitmap,
            shader: None,
            texture: None,

            generation_queue: HashMap::new(),
            noise_scale,
            noise_offset,
            perlin,
        }
    }

    pub fn block_at_world_pos(&self, world_pos: &BlockWorldPos) -> usize {
        let (chunk_index, block_index) = World::chunk_and_block_index(world_pos);
        if let Some(chunk) = self.chunks.get(&chunk_index) {
            chunk.block_at_chunk_pos(&block_index)
        } else {
            0
        }
    }

    fn chunk_and_block_index(world_pos: &BlockWorldPos) -> (ChunkIndex, BlockIndex) {
        let chunk_index = Vector3 {
            x: (world_pos.x as f32 / CHUNK_SIZE as f32).floor() as isize,
            y: (world_pos.y as f32 / CHUNK_SIZE as f32).floor() as isize,
            z: (world_pos.z as f32 / CHUNK_SIZE as f32).floor() as isize,
        };
        let block_index = Vector3 {
            x: (world_pos.x.rem_euclid(CHUNK_SIZE as isize)) as usize,
            y: (world_pos.y.rem_euclid(CHUNK_SIZE as isize)) as usize,
            z: (world_pos.z.rem_euclid(CHUNK_SIZE as isize)) as usize,
        };
        (chunk_index, block_index)
    }

    pub fn place_block(&mut self, block_id: usize, world_pos: &BlockWorldPos) {
        let (chunk_id, block_idx) = World::chunk_and_block_index(&world_pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
            chunk.blocks[block_idx.x][block_idx.y][block_idx.z] = block_id;
        } else {
            let mut new_chunk = Chunk::new();
            new_chunk.blocks[block_idx.x][block_idx.y][block_idx.z] = block_id;
            self.chunks.insert(chunk_id, new_chunk);
        }
    }

    pub fn build_all_chunk_mesh(&mut self, render_distance: isize, player_position: Vector3<f32>) {
        self.init_gl_resources();

        let player_world_pos = Vector3::new(
            player_position.x as isize,
            player_position.y as isize,
            player_position.z as isize,
        );
        let (chunk_idx, _block_idx) = World::chunk_and_block_index(&player_world_pos);

        for x in chunk_idx.x-render_distance ..= chunk_idx.x+render_distance {
            for y in chunk_idx.y-render_distance ..= chunk_idx.y+render_distance {
                for z in chunk_idx.z-render_distance ..= chunk_idx.z+render_distance {
                    let idx = Vector3::new(x, y, z);

                    let vertices = if let Some(chunk) = self.chunks.get(&idx) {
                        let x_pos = self.chunks.get(&(idx + Vector3::new(1, 0, 0)));
                        let x_neg = self.chunks.get(&(idx + Vector3::new(-1, 0, 0)));
                        let y_pos = self.chunks.get(&(idx + Vector3::new(0, 1, 0)));
                        let y_neg = self.chunks.get(&(idx + Vector3::new(0, -1, 0)));
                        let z_pos = self.chunks.get(&(idx + Vector3::new(0, 0, 1)));
                        let z_neg = self.chunks.get(&(idx + Vector3::new(0, 0, -1)));
                        World::gen_chunk_verts(
                            chunk,
                            x_pos,
                            x_neg,
                            y_pos,
                            y_neg,
                            z_pos,
                            z_neg,
                        )
                    } else {
                        continue;
                    };
                    let chunk = self.chunks.get_mut(&idx).unwrap();
                    chunk.rebuild_mesh(vertices, self.shader.clone(), self.texture.clone())
                    
                }
            }
        }
    }

    fn gen_chunk_verts(
        chunk: &Chunk,
        x_pos: Option<&Chunk>,
        x_neg: Option<&Chunk>,
        y_pos: Option<&Chunk>,
        y_neg: Option<&Chunk>,
        z_pos: Option<&Chunk>, 
        z_neg: Option<&Chunk>,
    ) -> Vec<Vertex3D> {
        let mut vertices = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let i = chunk.blocks[x][y][z] as usize;
                    if i == 0 {
                        continue;
                    }
                    let cur = &block::BLOCKS[i];
                    let tex_coords:[(f32, f32);  6] = if let Some(texture_type) = &cur.texture_map {
                        let mut coords = [(0.0f32, 0.0f32); 6];
                        match texture_type {
                            block::TextureType::Single(x, y) => {
                                for i in 0..6 {
                                    coords[i] = (*x, *y)
                                }
                            },
                            block::TextureType::TopAndSide((x_top, y_top), (x_side, y_side)) => {
                                coords[0] = (*x_side, *y_side);
                                coords[1] = (*x_side, *y_side);
                                coords[2] = (*x_top, *y_top);
                                coords[3] = (*x_side, *y_side);
                                coords[4] = (*x_side, *y_side);
                                coords[5] = (*x_side, *y_side);
                            },
                            block::TextureType::TopSideBottom((x_top, y_top), (x_side, y_side), (x_bottom, y_bottom)) => {
                                coords[0] = (*x_side, *y_side);
                                coords[1] = (*x_side, *y_side);
                                coords[2] = (*x_top, *y_top);
                                coords[3] = (*x_bottom, *y_bottom);
                                coords[4] = (*x_side, *y_side);
                                coords[5] = (*x_side, *y_side);
                            },
                            block::TextureType::TopSideFrontActivatable(
                                (x_front_inactive, y_front_inactive),
                                (x_front_active, y_front_active),
                                (x_side, y_side),
                                (x_top, y_top)
                            ) => {
                                coords[0] = (*x_side, *y_side);
                                coords[1] = (*x_side, *y_side);
                                coords[2] = (*x_top, *y_top);
                                coords[3] = (*x_top, *y_top);
                                coords[4] = (*x_side, *y_side);
                                let active = chunk.metadata[x][y][z] == 1;
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
                            let x_right_adjacent = if x < CHUNK_SIZE-1 {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x+1, y, z))])
                            } else if let Some(chunk) = x_pos {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(0, y, z))])
                            } else {
                                None
                            };
                            if let Some(adjacent_block) = x_right_adjacent {
                                if adjacent_block.transparent {
                                    push_face(&position, 0, &mut vertices, &tex_coords[0], vertex_type);
                                }
                            }

                            let x_left_adjacent = if x > 0 {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x-1, y, z))])
                            } else if let Some(chunk) = x_neg {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(CHUNK_SIZE-1, y, z))])
                            } else {
                                None
                            };
                            if let Some(adjacent_block) = x_left_adjacent {
                                if adjacent_block.transparent {
                                    push_face(&position, 1, &mut vertices, &tex_coords[1], vertex_type);
                                }
                            }

    
                            let y_top_adjacent = if y < CHUNK_SIZE-1 {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y+1, z))])
                            } else if let Some(chunk) = y_pos {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x,0, z))])
                            } else {
                                None
                            };
                            if let Some(adjacent_block) = y_top_adjacent {
                                if adjacent_block.transparent {
                                    push_face(&position, 2, &mut vertices, &tex_coords[2], vertex_type);
                                }
                            }
    
                            let y_bottom_adjacent = if y > 0 {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y-1, z))])
                            } else if let Some(chunk) = y_neg {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x,CHUNK_SIZE-1, z))])
                            } else {
                                None
                            };
                            if let Some(adjacent_block) = y_bottom_adjacent {
                                if adjacent_block.transparent {
                                    push_face(&position, 3, &mut vertices, &tex_coords[3], vertex_type);
                                }
                            }

                            let z_back_adjacent = if z < CHUNK_SIZE-1 {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y, z+1))])
                            } else if let Some(chunk) = z_pos {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y, 0))])
                            } else {
                                None
                            };
                            if let Some(adjacent_block) = z_back_adjacent {
                                if adjacent_block.transparent {
                                    push_face(&position, 4, &mut vertices, &tex_coords[4], vertex_type);
                                }
                            }


                            let z_front_adjacent = if z > 0 {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y, z-1))])
                            } else if let Some(chunk) = z_neg {
                                Some(BLOCKS[chunk.block_at_chunk_pos(&Vector3::new(x, y, CHUNK_SIZE-1))])
                            } else {
                                None
                            };
                            if let Some(adjacent_block) = z_front_adjacent {
                                if adjacent_block.transparent {
                                    push_face(&position, 5, &mut vertices, &tex_coords[5], vertex_type);
                                }
                            }
                        }
                        MeshType::CrossedPlanes => {
                            push_face(&position, 6, &mut vertices, &tex_coords[0], vertex_type);
                            push_face(&position, 7, &mut vertices, &tex_coords[0], vertex_type);
                            push_face(&position, 8, &mut vertices, &tex_coords[0], vertex_type);
                            push_face(&position, 9, &mut vertices, &tex_coords[0], vertex_type);
                        }
                    }
                    
                }
            }
        }
            
        vertices
    }

    pub fn collision_at_world_pos(&self, world_pos: &BlockWorldPos) -> bool {
        true
    }
}

impl GLRenderable for World {
    fn init_gl_resources(&mut self) {
        self.shader = Some(match Shader::new(self.shader_src.0, self.shader_src.1) {
            Ok(s) => s,
            Err(_) => todo!(),
        });
        self.texture = Some(Texture::from_dynamic_image_bytes(&self.texture_bitmap, image::ImageFormat::Png));
    }

    fn draw(&self, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, elapsed_time: f32) {
        for (idx, chunk) in &self.chunks {
            let position = Vector3::new(
                (idx.x * CHUNK_SIZE as isize) as f32,
                (idx.y * CHUNK_SIZE as isize) as f32,
                (idx.z * CHUNK_SIZE as isize) as f32,
            );
            chunk.draw(position, perspective_matrix, view_matrix, elapsed_time);
        }
    }
}
