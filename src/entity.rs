use crate::{
    graphics::resources::GLRenderable,
    physics::{collision::Collider, physics_update::PhysicsUpdate},
};

pub trait EntityTrait: GLRenderable + PhysicsUpdate + Collider + Sync + Send {}
