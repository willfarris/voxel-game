use crate::terrain::{Terrain, BlockWorldPos};
use cgmath::{Vector3, Quaternion, Rotation, InnerSpace};

pub const X_VECTOR: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
pub const Y_VECTOR: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
pub const Z_VECTOR: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

#[derive(PartialEq, Eq)]
pub enum Vec3Direction {
    X,
    Y,
    Z
}

pub fn quaternion_rotate(vec: Vector3<f32>, angle: f32, axis: Vector3<f32>) -> Vector3<f32> {
    Quaternion::from_sv(-angle, axis).rotate_vector(vec)
}

// Fast inverse square root <3
// https://en.wikipedia.org/wiki/Fast_inverse_square_root
pub fn q_rsqrt(number: f32)  -> f32 {
    let x2 = number * 0.5f32;
    let threehalfs = 1.5f32;
    let mut i: u32 = number.to_bits();
    i = 0x5f375a86 - (i >> 1);
    let mut y: f32 = f32::from_bits(i);
    y = y * ( threehalfs - (x2 * y * y ) );
    y
}

pub fn dda(world: &Terrain, start: &Vector3<f32>, dir: &Vector3<f32>, max_dist: f32) -> Option<(Vector3<f32>, BlockWorldPos)> {
    let ray_dir = dir.normalize();

    let mut ray_unit_step_size = Vector3 {
        x: (1.0 + (ray_dir.y/ray_dir.x)*(ray_dir.y/ray_dir.x) + (ray_dir.z/ray_dir.x)*(ray_dir.z/ray_dir.x)).sqrt(),
        y: ((ray_dir.x/ray_dir.y)*(ray_dir.x/ray_dir.y) + 1.0 + (ray_dir.z/ray_dir.y)*(ray_dir.z/ray_dir.y)).sqrt(),
        z: ((ray_dir.x/ray_dir.z)*(ray_dir.x/ray_dir.z) + (ray_dir.y/ray_dir.z)*(ray_dir.y/ray_dir.z) + 1.0).sqrt(),
    };

    if ray_unit_step_size.x.is_nan() {
        ray_unit_step_size.x = 1.0;
    }
    if ray_unit_step_size.y.is_nan() {
        ray_unit_step_size.y = 1.0;
    }
    if ray_unit_step_size.z.is_nan() {
        ray_unit_step_size.z = 1.0;
    }

    let mut map_check = BlockWorldPos {
        x: start.x.floor() as isize,
        y: start.y.floor() as isize,
        z: start.z.floor() as isize,
    };
    let mut ray_length_1d = Vector3 {x: 0.0, y: 0.0, z: 0.0 };
    let mut step = Vector3 {x: 0, y: 0, z: 0};

    if ray_dir.x < 0.0 {
        step.x = -1;
        ray_length_1d.x = (start.x - map_check.x as f32) * ray_unit_step_size.x;
    } else {
        step.x = 1;
        ray_length_1d.x = ((map_check.x as f32 + 1.0) - start.x) * ray_unit_step_size.x;
    }

    if ray_dir.y < 0.0 {
        step.y = -1;
        ray_length_1d.y = (start.y - map_check.y as f32) * ray_unit_step_size.y;
    } else {
        step.y = 1;
        ray_length_1d.y = ((map_check.y as f32 + 1.0) - start.y) * ray_unit_step_size.y;
    }

    if ray_dir.z < 0.0 {
        step.z = -1;
        ray_length_1d.z = (start.z - map_check.z as f32) * ray_unit_step_size.z;
    } else {
        step.z = 1;
        ray_length_1d.z = ((map_check.z as f32 + 1.0) - start.z) * ray_unit_step_size.z;
    }

    let mut dist = 0.0;
    while dist < max_dist {

        let mut min_dist = ray_length_1d.x;
        let mut min_dir = Vec3Direction::X;
        if ray_length_1d.y < min_dist { min_dist = ray_length_1d.y; min_dir = Vec3Direction::Y }
        #[allow(unused_assignments)]
        if ray_length_1d.z < min_dist { min_dist = ray_length_1d.z; min_dir = Vec3Direction::Z }

        if min_dir == Vec3Direction::X {
            map_check.x += step.x;
            dist = ray_length_1d.x;
            ray_length_1d.x += ray_unit_step_size.x;
        } else if min_dir == Vec3Direction::Y {
            map_check.y += step.y;
            dist = ray_length_1d.y;
            ray_length_1d.y += ray_unit_step_size.y;
        } else {
            map_check.z += step.z;
            dist = ray_length_1d.z;
            ray_length_1d.z += ray_unit_step_size.z;
        }
        if world.collision_at_world_pos(&map_check) {
            return Some(
                (start + ray_dir * dist, Vector3 { x: map_check.x, y: map_check.y, z: map_check.z})
            );
        }
    }
    None
}