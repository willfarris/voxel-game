

use crate::{graphics::resources::GLRenderable, physics::{physics_update::PhysicsUpdate, collision::Collider}};

pub trait EntityTrait: GLRenderable + PhysicsUpdate + Collider + Sync + Send {}