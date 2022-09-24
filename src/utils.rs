use bevy::prelude::*;

use crate::vectors::Vector2Int;

pub fn v2iv3(
    v: Vector2Int,
    z: f32,
    scale: f32
) -> Vec3 {
    Vec3::new(
        scale * v.x as f32,
        scale * v.y as f32,
        z
    )
}