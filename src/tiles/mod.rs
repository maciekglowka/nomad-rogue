use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::MAP_SIZE;
use crate::vectors::Vector2Int;

mod element;
mod renderer;

pub use element::TileElement;
pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            renderer::load_assets
        );
        app.add_startup_system(spawn_board);
    }
}

pub fn spawn_tile(
    v: Vector2Int,
    commands: &mut Commands,
    assets: &mut ResMut<TileAssets>,
) -> bool {
    if assets.tile_map.contains_key(&v) { return false; }

    // tiles need to have at least one neighbour; except for the very first
    if assets.tile_map.len() > 0 && get_neighbours(v, assets).len() == 0 {
        return false;
    }    

    let tile = commands.spawn_bundle(
        renderer::get_tile_renderer(v, &assets)
    ).insert(Tile { 
        v: v,
        element: None
     })
    .id();

    assets.tile_map.insert(v, tile);
    true
}

pub fn spawn_board(
    mut commands: Commands,
    mut assets: ResMut<TileAssets>,
) {
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            spawn_tile(v, &mut commands, &mut assets);
        }
    }
}

pub struct TileAssets {
    texture: Handle<TextureAtlas>,
    pub tile_map: HashMap<Vector2Int, Entity>
}

#[derive(Component)]
pub struct Tile {
    pub element: Option<Entity>,
    pub v: Vector2Int
}

fn get_neighbour_vecs(v: Vector2Int) -> [Vector2Int; 4] {
    [
        Vector2Int::new(v.x, v.y - 1), Vector2Int::new(v.x, v.y + 1),
        Vector2Int::new(v.x - 1, v.y), Vector2Int::new(v.x + 1, v.y)
    ]
}

fn get_neighbours(v: Vector2Int, assets: &TileAssets) -> Vec<Entity> {
    get_neighbour_vecs(v)
        .iter()
        .filter(|a| assets.tile_map.contains_key(a))
        .map(|a| assets.tile_map[a])
        .collect()
}