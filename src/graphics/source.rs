pub const SCREENQUAD_VERT_SRC: &str = include_str!("../../shaders/screenquad.vert");

pub const LIGHTING_FRAG_SRC: &str = include_str!("../../shaders/lighting.frag");
pub const COMPOSITE_FRAG_SRC: &str = include_str!("../../shaders/composite.frag");
pub const POSTPROCESS_FRAG_SRC: &str = include_str!("../../shaders/postprocess.frag");

pub const TERRAIN_VERT_SRC: &str = include_str!("../../shaders/cube.vert");
pub const TERRAIN_FRAG_SRC: &str = include_str!("../../shaders/cube.frag");
pub const TERRAIN_BITMAP: &[u8] = include_bytes!("../../assets/terrain.png");

pub const SKYBOX_VERT_SRC: &str = include_str!("../../shaders/skybox.vert");
pub const SKYBOX_FRAG_SRC: &str = include_str!("../../shaders/skybox.frag");
pub const SKYBOX_BITMAP: &[u8] = include_bytes!("../../assets/sky.png");
