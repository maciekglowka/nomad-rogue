use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::{TILE_SIZE, TILE_Z};
use crate::vectors::Vector2Int;

pub fn get_tile_renderer(
    v: Vector2Int,
    assets: &ResMut<super::TileAssets>
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(176);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: assets.texture.clone(),
        transform: Transform::from_translation(
            Vec3::new(v.x as f32 * TILE_SIZE, v.y as f32 * TILE_SIZE, TILE_Z)
        ),
        ..Default::default()
    }
}

pub fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let image = assets.load("ascii.png");
    asset_list.0.push(image.clone_untyped());
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0),
        Vec2::ZERO
    );

    let atlas_handle = texture_atlasses.add(atlas);

    commands.insert_resource(super::TileAssets { 
        texture: atlas_handle,
        tile_map: HashMap::new()
    });
}