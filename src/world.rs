use std::collections::{HashMap, LinkedList};

use cgmath::{Vector3, Matrix4, Vector2};
use noise::Perlin;

use crate::{graphics::{shader::Shader, texture::Texture, resources::{GLRenderable, GLResources}, vertex::Vertex3D, mesh::{push_face, block_drop_vertices}, source::{TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC, TERRAIN_BITMAP}}, item::drop::ItemDrop};

use self::{chunk::{Chunk, CHUNK_SIZE}, block::{BLOCKS, MeshType}};

mod chunk;
pub(crate) mod block;
mod terrain;

pub type BlockWorldPos = Vector3<isize>;
pub type ChunkIndex = Vector3<isize>;
pub type BlockIndex = Vector3<usize>; 

pub struct World {
    chunks: HashMap<Vector3<isize>, Chunk>,
    generation_queue: HashMap<Vector3<isize>, LinkedList<(Vector3<usize>, usize)>>,
    noise_offset: Vector2<f64>,
    noise_scale: f64,
    perlin: Perlin,
}

impl World {
    pub fn new() -> Self {
        let noise_scale = 0.02;
        let noise_offset = Vector2::new(
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
            1_000_000.0 * rand::random::<f64>() + 3_141_592.0,
        );
        let perlin = Perlin::new();
        Self {
            chunks: HashMap::new(),

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
        let (chunk_index, block_idx) = World::chunk_and_block_index(&world_pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
            chunk.blocks[block_idx.x][block_idx.y][block_idx.z] = block_id;
        } else {
            let position = Vector3::new(
                (chunk_index.x * CHUNK_SIZE as isize) as f32,
                (chunk_index.y * CHUNK_SIZE as isize) as f32,
                (chunk_index.z * CHUNK_SIZE as isize) as f32,
            );
            let mut new_chunk = Chunk::new(position);
            new_chunk.blocks[block_idx.x][block_idx.y][block_idx.z] = block_id;
            self.chunks.insert(chunk_index, new_chunk);
        }
    }

    pub fn build_all_chunk_vertices(&self, render_distance: isize, player_position: Vector3<f32>) -> Vec<(ChunkIndex, Vec<Vertex3D>)> {
        let player_world_pos = Vector3::new(
            player_position.x as isize,
            player_position.y as isize,
            player_position.z as isize,
        );
        let (chunk_idx, _block_idx) = World::chunk_and_block_index(&player_world_pos);
        let mut vertex_list = Vec::new();

        for x in chunk_idx.x-render_distance ..= chunk_idx.x+render_distance {
            for y in chunk_idx.y-render_distance ..= chunk_idx.y+render_distance {
                for z in chunk_idx.z-render_distance ..= chunk_idx.z+render_distance {
                    let chunk_index = Vector3::new(x, y, z);
                    if let Some(verts) = self.generate_chunk_mesh(&chunk_index) {
                        vertex_list.push((chunk_index, verts))
                    }
                }
            }
        }
        vertex_list
    }

    fn generate_chunk_mesh(&self, chunk_index: &ChunkIndex) -> Option<Vec<Vertex3D>>{
        if let Some(chunk) = self.chunks.get(&chunk_index) {
            let x_pos = self.chunks.get(&(chunk_index + Vector3::new(1, 0, 0)));
            let x_neg = self.chunks.get(&(chunk_index + Vector3::new(-1, 0, 0)));
            let y_pos = self.chunks.get(&(chunk_index + Vector3::new(0, 1, 0)));
            let y_neg = self.chunks.get(&(chunk_index + Vector3::new(0, -1, 0)));
            let z_pos = self.chunks.get(&(chunk_index + Vector3::new(0, 0, 1)));
            let z_neg = self.chunks.get(&(chunk_index + Vector3::new(0, 0, -1)));
            World::generate_chunk_verts(
                chunk,
                x_pos,
                x_neg,
                y_pos,
                y_neg,
                z_pos,
                z_neg,
            )
        } else {
            None
        }
    }

    fn generate_chunk_verts(
        chunk: &Chunk,
        x_pos: Option<&Chunk>,
        x_neg: Option<&Chunk>,
        y_pos: Option<&Chunk>,
        y_neg: Option<&Chunk>,
        z_pos: Option<&Chunk>, 
        z_neg: Option<&Chunk>,
    ) -> Option<Vec<Vertex3D>> {
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
            
        if vertices.is_empty() {
            None
        } else {
            Some(vertices)
        }
    }

    pub fn collision_at_world_pos(&self, world_pos: &BlockWorldPos) -> bool {
        let (chunk_index, block_index) = World::chunk_and_block_index(world_pos);
        if let Some(chunk) = self.chunks.get(&chunk_index) {
            if chunk.block_at_chunk_pos(&block_index) != 0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /*pub fn rebuild_queue(&mut self) {
        for i in 0..self.should_rebuild.len() {
            if i == self.should_rebuild.len() {
                break;
            }
            let (chunk_index, block_index) = self.should_rebuild.swap_remove(i);
            self.generate_chunk_mesh(&chunk_index);

            if block_index.x == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.generate_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.x == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.generate_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.y == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.generate_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.y == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.generate_chunk_mesh(&adjacent_chunk_index);
                }
            }

            if block_index.z == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.generate_chunk_mesh(&adjacent_chunk_index);
                }
            } else if block_index.z == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    self.generate_chunk_mesh(&adjacent_chunk_index);
                }
            }
        }
        
    }
    */
    
    pub fn destroy_at_global_pos(&mut self, world_pos: &BlockWorldPos, gl_resources: &mut GLResources) -> Option<ItemDrop> {
        let (chunk_index, block_index) = World::chunk_and_block_index(world_pos);

        if let Some(chunk) = self.chunks.get_mut(&chunk_index) {
            
            // Delete block in the world
            let block_id = chunk.blocks[block_index.x][block_index.y][block_index.z];
            chunk.blocks[block_index.x][block_index.y][block_index.z] = 0;
            
            if let Some(chunk_vertices) = self.generate_chunk_mesh(&chunk_index) {
                let name = format!("chunk_{}_{}_{}", chunk_index.x, chunk_index.y, chunk_index.z);
                gl_resources.update_buffer(name, chunk_vertices);
            }

            if block_index.x == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    if let Some(adjacent_chunk_vertices) = self.generate_chunk_mesh(&adjacent_chunk_index) {
                        let name = format!("chunk_{}_{}_{}", adjacent_chunk_index.x, adjacent_chunk_index.y, adjacent_chunk_index.z);
                        gl_resources.update_buffer(name, adjacent_chunk_vertices);
                    }
                }
            } else if block_index.x == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(1, 0, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    if let Some(adjacent_chunk_vertices) = self.generate_chunk_mesh(&adjacent_chunk_index) {
                        let name = format!("chunk_{}_{}_{}", adjacent_chunk_index.x, adjacent_chunk_index.y, adjacent_chunk_index.z);
                        gl_resources.update_buffer(name, adjacent_chunk_vertices);
                    }
                }
            }

            if block_index.y == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    if let Some(adjacent_chunk_vertices) = self.generate_chunk_mesh(&adjacent_chunk_index) {
                        let name = format!("chunk_{}_{}_{}", adjacent_chunk_index.x, adjacent_chunk_index.y, adjacent_chunk_index.z);
                        gl_resources.update_buffer(name, adjacent_chunk_vertices);
                    }
                }
            } else if block_index.y == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 1, 0);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    if let Some(adjacent_chunk_vertices) = self.generate_chunk_mesh(&adjacent_chunk_index) {
                        let name = format!("chunk_{}_{}_{}", adjacent_chunk_index.x, adjacent_chunk_index.y, adjacent_chunk_index.z);
                        gl_resources.update_buffer(name, adjacent_chunk_vertices);
                    }
                }
            }

            if block_index.z == 0 {
                let adjacent_chunk_index = chunk_index - Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    if let Some(adjacent_chunk_vertices) = self.generate_chunk_mesh(&adjacent_chunk_index) {
                        let name = format!("chunk_{}_{}_{}", adjacent_chunk_index.x, adjacent_chunk_index.y, adjacent_chunk_index.z);
                        gl_resources.update_buffer(name, adjacent_chunk_vertices);
                    }
                }
            } else if block_index.z == CHUNK_SIZE-1 {
                let adjacent_chunk_index = chunk_index + Vector3::new(0, 0, 1);
                if let Some(_) = self.chunks.get(&adjacent_chunk_index) {
                    if let Some(adjacent_chunk_vertices) = self.generate_chunk_mesh(&adjacent_chunk_index) {
                        let name = format!("chunk_{}_{}_{}", adjacent_chunk_index.x, adjacent_chunk_index.y, adjacent_chunk_index.z);
                        gl_resources.update_buffer(name, adjacent_chunk_vertices);
                    }
                }
            }

            // Create a drop and return it
            let drop_world_pos = Vector3::new(
                world_pos.x as f32 + 0.5,
                world_pos.y as f32 + 0.5,
                world_pos.z as f32 + 0.5,
            );
            let block_drop = ItemDrop::new(
                block_id,
                drop_world_pos,
            );
            return Some(block_drop);
        }
        None

    }
}

impl GLRenderable for World {
    fn init_gl_resources(&self, gl_resources: &mut GLResources) {
        if gl_resources.get_shader("terrain").is_none() {
            gl_resources.create_shader("terrain", TERRAIN_VERT_SRC, TERRAIN_FRAG_SRC);
        }
        if gl_resources.get_texture("terrain").is_none() {
            gl_resources.create_texture("terrain", TERRAIN_BITMAP);
        }

        let verts = self.build_all_chunk_vertices(5, Vector3::new(0.0, 0.0, 0.0));
        for (chunk_index, chunk_verts) in verts {
            let name = format!("chunk_{}_{}_{}", chunk_index.x, chunk_index.y, chunk_index.z);
            gl_resources.create_buffer_from_verts(name, chunk_verts);
        }
    }

    fn draw(&self, gl_resources: &mut GLResources, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, elapsed_time: f32) {
        
        let shader = gl_resources.get_shader("terrain").unwrap();
        let texture = gl_resources.get_texture("terrain").unwrap();

        texture.bind();

        shader.use_program();
        shader.set_mat4(unsafe {c_str!("perspective_matrix")}, &perspective_matrix);
        shader.set_mat4(unsafe {c_str!("view_matrix")}, &view_matrix);
        shader.set_float(unsafe {c_str!("time")}, elapsed_time);
        shader.set_texture(unsafe {c_str!("texture_map")}, 0);
        
        for (chunk_index, _chunk) in &self.chunks {
            let model_matrix = Matrix4::from_translation(Vector3::new(
                (chunk_index.x * CHUNK_SIZE as isize) as f32,
                (chunk_index.y * CHUNK_SIZE as isize) as f32,
                (chunk_index.z * CHUNK_SIZE as isize) as f32,
            ));
            shader.set_mat4(unsafe {c_str!("model_matrix")}, &model_matrix);

            let name = format!("chunk_{}_{}_{}", chunk_index.x, chunk_index.y, chunk_index.z);
            if let Some(vbo) = gl_resources.get_buffer(name) {
                vbo.draw_vertex_buffer();
            }
        }
    }
}