pub const TERRAIN_VERT_SRC: &str = include_str!("../../shaders/cube.vert");
pub const TERRAIN_FRAG_SRC: &str = include_str!("../../shaders/cube.frag");

pub const SCREENQUAD_VERT_SRC: &str = include_str!("../../shaders/screenquad.vert");

pub const SSAO_FRAG_SRC: &str = include_str!("../../shaders/ssao.frag");
pub const POSTPROCESS_FRAG_SRC: &str = include_str!("../../shaders/postprocess.frag");

pub const TERRAIN_BITMAP: &[u8] = include_bytes!("../../assets/terrain.png");
