use bevy::prelude::*;

use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct TileElement {
    pub v: Vector2Int
}