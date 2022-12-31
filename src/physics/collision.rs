use cgmath::Vector3;

use crate::{world::{World, block::BLOCKS, BlockWorldPos}, entity::EntityTrait};

use super::vectormath::Vec3Direction;

pub trait Collider {
    fn bounding_box(&self) -> Rect3;
    fn movement_delta(&self) -> Vector3<f32>;
    fn correct_position_axis(&mut self, axis: Vec3Direction, overlap: f32);
    fn has_collider(&self) -> bool;
}

// Returns the overlap of `entity` with `world` along the specified axis
pub fn check_world_collision_axis(axis: Vec3Direction, bounding_box: Rect3, world: &World) -> f32 {
    for block_x in (bounding_box.pos.x.floor() as isize - 1) ..= ((bounding_box.pos.x + bounding_box.size.x).floor() as isize + 1) {
        for block_y in (bounding_box.pos.y.floor() as isize - 1) ..= ((bounding_box.pos.y + bounding_box.size.y).floor() as isize + 2) {
            for block_z in (bounding_box.pos.z.floor() as isize - 1) ..= ((bounding_box.pos.z + bounding_box.size.z).floor() as isize + 1) {
                if !BLOCKS[world.block_at_world_pos(&BlockWorldPos::new(block_x, block_y, block_z))].solid {
                    continue;
                }
                let block_bounding_box = Rect3 {
                    pos: Vector3::new(block_x as f32, block_y as f32, block_z as f32),
                    size: Vector3::new(1.0, 1.0, 1.0)
                };
                if rect_vs_rect(&bounding_box, &block_bounding_box) {
                    match axis {
                        Vec3Direction::X => {
                            let x_overlap = if bounding_box.pos.x > block_bounding_box.pos.x {
                                (block_bounding_box.pos.x + 1.0) - bounding_box.pos.x 
                            } else {
                                -1.0 * (bounding_box.pos.x + bounding_box.size.x - block_bounding_box.pos.x)
                            };
                            return x_overlap;
                        },
                        Vec3Direction::Y => {
                            let y_overlap = if bounding_box.pos.y > block_bounding_box.pos.y {
                                (block_bounding_box.pos.y + 1.0) - bounding_box.pos.y
                            } else {
                                -1.0 * (bounding_box.pos.y + bounding_box.size.y - block_bounding_box.pos.y)
                            };
                            return y_overlap;
                        },
                        Vec3Direction::Z => {
                            let z_overlap = if bounding_box.pos.z > block_bounding_box.pos.z {
                                (block_bounding_box.pos.z + 1.0) - bounding_box.pos.z
                            } else {
                                -1.0 * (bounding_box.pos.z + bounding_box.size.z - block_bounding_box.pos.z)
                            };
                            return z_overlap;
                        },
                    }
                }
            }
        }
    }
    0f32
}

#[allow(unused)]
pub fn check_collision_axis(axis: Vec3Direction, bounding_box1: Rect3, bounding_box2: Rect3) -> f32 {
    if rect_vs_rect(&bounding_box1, &bounding_box2) {
        match axis {
            Vec3Direction::X => {
                let x_overlap = if bounding_box1.pos.x > bounding_box2.pos.x {
                    (bounding_box2.pos.x + bounding_box2.size.x) - bounding_box1.pos.x 
                } else {
                    -1.0 * (bounding_box1.pos.x + bounding_box1.size.x - bounding_box2.pos.x)
                };
                return x_overlap;
            },
            Vec3Direction::Y => {
                let y_overlap = if bounding_box1.pos.y > bounding_box2.pos.y {
                    (bounding_box2.pos.y + bounding_box2.size.y) - bounding_box1.pos.y
                } else {
                    -1.0 * (bounding_box1.pos.y + bounding_box1.size.y - bounding_box2.pos.y)
                };
                return y_overlap;
            },
            Vec3Direction::Z => {
                let z_overlap = if bounding_box1.pos.z > bounding_box2.pos.z {
                    (bounding_box2.pos.z + bounding_box2.size.z) - bounding_box1.pos.z
                } else {
                    -1.0 * (bounding_box1.pos.z + bounding_box1.size.z - bounding_box2.pos.z)
                };
                return z_overlap;
            },
        }
    }
    0f32
}

#[derive(Clone)]
pub struct Rect3 {
    pub pos: Vector3<f32>,
    pub size: Vector3<f32>,
}

impl Rect3 {
    pub fn new(pos: Vector3<f32>, size: Vector3<f32>) -> Rect3 {
        Rect3 {
            pos,
            size,
        }
    }
}

/*pub fn point_vs_rect(p: &Vector3<f32>, r: &Rect3) -> bool { 
    p.x >= r.pos.x &&
    p.y >= r.pos.y &&
    p.z >= r.pos.z &&
    
    p.x <= (r.pos.x + r.size.x) &&
    p.y <= (r.pos.y + r.size.y) &&
    p.z <= (r.pos.z + r.size.z)
}*/

pub fn rect_vs_rect(r1: &Rect3, r2: &Rect3) -> bool {
    r1.pos.x < (r2.pos.x + r2.size.x) && (r1.pos.x + r1.size.x) > r2.pos.x &&
    r1.pos.y < (r2.pos.y + r2.size.y) && (r1.pos.y + r1.size.y) > r2.pos.y &&
    r1.pos.z < (r2.pos.z + r2.size.z) && (r1.pos.z + r1.size.z) > r2.pos.z
}

/*fn ray_vs_rect(
    ray_origin: &Vector3<f32>,
    ray_dir: &Vector3<f32>,
    target: &Rect3,
    contact_point: &mut Vector3<f32>,
    contact_normal: &mut Vector3<f32>,
    t_hit_near: &mut f32) -> bool {

        *contact_normal = Vector3::new(0.0, 0.0, 0.0);
        *contact_point = Vector3::new(0.0, 0.0, 0.0);

        let invdir = 1.0 / ray_dir;

        let mut t_near = target.pos - ray_origin;
        t_near.x *= invdir.x;
        t_near.y *= invdir.y;
        t_near.z *= invdir.z;
        let mut t_far = target.pos + target.size - ray_origin;
        t_far.x *= invdir.x;
        t_far.y *= invdir.y;
        t_far.z *= invdir.z;

        if t_far.x.is_nan() || t_far.y.is_nan() || t_far.z.is_nan() {
            return false;
        }
        if t_near.x.is_nan() || t_near.y.is_nan() || t_near.z.is_nan() {
            return false;
        }

        if t_near.x > t_far.x { swap(&mut t_near.x, &mut t_far.x)}
        if t_near.y > t_far.y { swap(&mut t_near.y, &mut t_far.y)}
        if t_near.z > t_far.z { swap(&mut t_near.z, &mut t_far.z)}

        if (t_near.x > t_far.y && t_near.x > t_far.z) ||
           (t_near.y > t_far.x && t_near.y > t_far.z) ||
           (t_near.z > t_far.x && t_near.z > t_far.y) {return false; }

        *t_hit_near = t_near.x.max(t_near.y.max(t_near.z));

        let t_hit_far = t_far.x.min(t_far.y.min(t_far.z));

        if t_hit_far < 0.0 {
            return false;
        }

        *contact_point = ray_origin + *t_hit_near * ray_dir;
        
        if t_near.x > t_near.y && t_near.x > t_near.z {
            if invdir.x < 0.0 {
                *contact_normal = Vector3::new(1.0, 0.0, 0.0);
            } else {
                *contact_normal = Vector3::new(-1.0, 0.0, 0.0);
            }
        } else if t_near.y > t_near.x && t_near.y > t_near.z {
            if invdir.y < 0.0 {
                *contact_normal = Vector3::new(0.0, 1.0, 0.0);
            } else {
                *contact_normal = Vector3::new(0.0, -1.0, 0.0);
            }
        } else if t_near.z > t_near.x && t_near.z > t_near.y {
            if invdir.z < 0.0 {
                *contact_normal = Vector3::new(0.0, 0.0, 1.0);
            } else {
                *contact_normal = Vector3::new(0.0, 0.0, -1.0);
            }
        } 

        true
}*/
