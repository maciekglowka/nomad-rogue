use bevy::prelude::*;

use crate::globals::{CURSOR_Z, TILE_SIZE};

pub struct UpdateCursorEvent;

pub fn spawn_cursor(
    mut commands: Commands,
    assets: Res<CursorAssets>
) {
    let mut sprite = TextureAtlasSprite::new(4);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: assets.texture.clone(),
        transform: Transform::from_translation(
            Vec3::new(0., 0., CURSOR_Z)
        ),
        ..Default::default()
    }).insert(Cursor);
}

pub fn update_cursor(
    mut ev: EventReader<UpdateCursorEvent>
) {
    for _ in ev.iter() {
        
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

    commands.insert_resource(CursorAssets { 
        texture: atlas_handle,
    });
}

pub struct CursorAssets {
    texture: Handle<TextureAtlas>
}

#[derive(Component)]
pub struct Cursor;