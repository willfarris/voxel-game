use cgmath::{Vector3, Vector2};

use crate::world::block::{Block, MeshType, self};

use super::vertex::Vertex3D;

const CUBE_FACES: [[Vertex3D; 6]; 10] = [
    
    // Facing positive-X
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },  // Front-bottom-right
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
    
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },  // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 }, // Front-top-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
        
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-bottom-right
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
    
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
    ],

    // Diagonal (0, 0) -> (1, 1)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
    ],

    // Diagonal (1, 1) -> (0, 0)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
    ],

    // Diagonal (0, 1) -> (1, 0)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
    ],

    // Diagonal (1, 0) -> (0, 1)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0 },
    ],
];

pub(crate) const DEFAULT_CUBE: [Vertex3D; 36] = [
    // Facing positive-X
    Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },  // Front-bottom-right
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right

    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },  // Back-top-right

    // Facing negative-X
    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 }, // Front-top-left
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
    
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
    Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-left

    // Facing positive-Y
    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left

    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-top-left
    
    // Facing negative-Y
    Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-bottom-right
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

    // Facing positive-Z
    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left

    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Front-bottom-right

    // Facing negative-Z
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex3D { position: Vector3::new( 1.0, 1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right

];

pub(crate) fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex3D>, texmap_offset: &(f32, f32), vertex_type: i32) {

    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v];
        vertex.position.x += position[0];
        vertex.position.y += position[1];
        vertex.position.z += position[2];

        vertex.tex_coords.x = vertex.tex_coords.x * 0.0625 + 0.0625 * texmap_offset.0 as f32;
        vertex.tex_coords.y = vertex.tex_coords.y * 0.0625 + 0.0625 * texmap_offset.1 as f32;

        vertex.vtype = vertex_type as i32;

        vertices.push(vertex);
    }
}

pub(crate) fn block_drop_vertices(block: &Block) -> Vec<Vertex3D> {
    let mut vertices = Vec::new();
    let mesh_type = block.mesh_type;
    let vertex_type = block.block_type as i32;
    let texture_map = block.texture_map;
    let dummy_position = [0.0, 0.0, 0.0];
    let tex_coords:[(f32, f32);  6] = if let Some(texture_type) = &texture_map {
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
                coords[5] = (*x_front_inactive, *y_front_inactive);
            }
        }
        coords
    } else {
        [(0.0, 0.0); 6]
    };
    match mesh_type {
        MeshType::Block => {
            push_face(&dummy_position, 0, &mut vertices, &tex_coords[0], vertex_type);
            push_face(&dummy_position, 1, &mut vertices, &tex_coords[1], vertex_type);
            push_face(&dummy_position, 2, &mut vertices, &tex_coords[2], vertex_type);
            push_face(&dummy_position, 3, &mut vertices, &tex_coords[3], vertex_type);
            push_face(&dummy_position, 4, &mut vertices, &tex_coords[4], vertex_type);
            push_face(&dummy_position, 5, &mut vertices, &tex_coords[5], vertex_type);
        },
        MeshType::CrossedPlanes => {
            push_face(&dummy_position, 6, &mut vertices, &tex_coords[0], vertex_type);
            push_face(&dummy_position, 7, &mut vertices, &tex_coords[0], vertex_type);
            push_face(&dummy_position, 8, &mut vertices, &tex_coords[0], vertex_type);
            push_face(&dummy_position, 9, &mut vertices, &tex_coords[0], vertex_type);
        }
    }
    /*for vertex in &vertices {
        println!("{:?}", vertex);
    }
    println!("");*/
    
    vertices
}