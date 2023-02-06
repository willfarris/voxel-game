pub const BLOCKS: [Block; 14] = [
    Block {
        id: 0,
        name: "Air",
        solid: false,
        transparent: true,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: None,
    },
    Block {
        id: 1,
        name: "Stone",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(1.0, 15.0)),
    },
    Block {
        id: 2,
        name: "Grass",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::TopSideBottom(
            (0.0, 15.0),
            (3.0, 15.0),
            (2.0, 15.0),
        )),
    },
    Block {
        id: 3,
        name: "Dirt",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(2.0, 15.0)),
    },
    Block {
        id: 4,
        name: "Rose",
        solid: false,
        transparent: true,
        block_type: BlockType::Grass,
        mesh_type: MeshType::CrossedPlanes,
        texture_map: Some(TextureType::Single(12.0, 15.0)),
    },
    Block {
        id: 5,
        name: "Oak Log",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::TopSideBottom(
            (5.0, 14.0),
            (4.0, 14.0),
            (5.0, 14.0),
        )),
    },
    Block {
        id: 6,
        name: "Dandelion",
        solid: false,
        transparent: true,
        block_type: BlockType::Grass,
        mesh_type: MeshType::CrossedPlanes,
        texture_map: Some(TextureType::Single(13.0, 15.0)),
    },
    Block {
        id: 7,
        name: "Oak Leaves",
        solid: true,
        transparent: true,
        block_type: BlockType::Leaves,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(4.0, 12.0)),
    },
    Block {
        id: 8,
        name: "Short Grass",
        solid: false,
        transparent: true,
        block_type: BlockType::Grass,
        mesh_type: MeshType::CrossedPlanes,
        texture_map: Some(TextureType::Single(7.0, 13.0)),
    },
    Block {
        id: 9,
        name: "Fern",
        solid: false,
        transparent: true,
        block_type: BlockType::Grass,
        mesh_type: MeshType::CrossedPlanes,
        texture_map: Some(TextureType::Single(8.0, 12.0)),
    },
    Block {
        id: 10,
        name: "Iron Ore",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(1.0, 13.0)),
    },
    Block {
        id: 11,
        name: "Coal",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(2.0, 13.0)),
    },
    Block {
        id: 12,
        name: "Glass",
        solid: true,
        transparent: true,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(1.0, 12.0)),
    },
    Block {
        id: 13,
        name: "Sand",
        solid: true,
        transparent: false,
        block_type: BlockType::Block,
        mesh_type: MeshType::Block,
        texture_map: Some(TextureType::Single(2.0, 14.0)),
    },
    //Block {id: 14, name: "Diamond Ore", solid: true, transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(2.0, 12.0))},
    //Block {id: 4, name: "Cobblestone", solid: true, transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(0.0, 14.0)) },
    //Block {id: 5, name: "Oak Plank", solid: true, transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::Single(4.0, 15.0)) },
    //Block {id: 17, name: "Furnace", solid: true, transparent: false, block_type: BlockType::Block, mesh_type: MeshType::Block, texture_map: Some(TextureType::TopSideFrontActivatable((12.0, 13.0),(13.0, 12.0), (13.0, 13.0),(14.0, 12.0)))},
];

pub fn block_index_by_name(name: &str) -> usize {
    for i in 0..BLOCKS.len() {
        if BLOCKS[i].name == name {
            return i;
        }
    }
    0
}

#[derive(Clone, Copy)]
pub enum BlockType {
    Block,
    Grass,
    Leaves,
}

#[derive(Clone, Copy)]
pub enum MeshType {
    Block,
    CrossedPlanes,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum TextureType {
    Single(f32, f32),
    TopAndSide((f32, f32), (f32, f32)),
    TopSideBottom((f32, f32), (f32, f32), (f32, f32)),
    TopSideFrontActivatable((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub struct Block {
    pub id: usize,
    pub name: &'static str,
    pub transparent: bool,
    pub solid: bool,
    pub block_type: BlockType,
    pub mesh_type: MeshType,
    pub texture_map: Option<TextureType>,
}

impl Block {
    pub fn _new(
        id: usize,
        name: &'static str,
        solid: bool,
        transparent: bool,
        block_type: BlockType,
        mesh_type: MeshType,
        texture_map: Option<TextureType>,
    ) -> Self {
        Self {
            id,
            name,
            solid,
            transparent,
            block_type,
            mesh_type,
            texture_map,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Air",
            solid: false,
            transparent: true,
            block_type: BlockType::Block,
            mesh_type: MeshType::Block,
            texture_map: None,
        }
    }
}
