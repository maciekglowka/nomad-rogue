use bevy::prelude::*;

use crate::globals::{TILE_SIZE, STRUCTURE_Z};
use crate::vectors::Vector2Int;

use super::StructureKind;

pub type BUNDLE_TYPE = SpriteSheetBundle;

pub fn get_structure_color(kind: StructureKind) -> Color {
    match kind {
        StructureKind::Gatherer => {
            Color::PURPLE
        }
    }
}

pub fn get_structure_renderer(
    v: Vector2Int,
    assets: &super::StructureAssets,
    kind: StructureKind
) -> BUNDLE_TYPE {
    let mut sprite = TextureAtlasSprite::new(254);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = get_structure_color(kind);

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: assets.texture.clone(),
        transform: Transform::from_translation(
            Vec3::new(v.x as f32 * TILE_SIZE, v.y as f32 * TILE_SIZE, STRUCTURE_Z)
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

    commands.insert_resource(super::StructureAssets { 
        texture: atlas_handle
    });
}