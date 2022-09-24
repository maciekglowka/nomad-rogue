use bevy::{
    prelude::*,
    render::texture::ImageSettings
};

mod assets;
mod globals;
mod manager;
mod player;
mod states;
mod structures;
mod tiles;
mod ui;
mod vectors;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(
            WindowDescriptor {
                height: 600.,
                width: 600.,
                ..Default::default()
            }
        )
        .insert_resource(ImageSettings::default_nearest())
        .init_resource::<assets::AssetList>()
        .add_state(states::GameState::LoadAssets)
        .add_plugins(DefaultPlugins)
        .add_plugin(manager::ManagerPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(structures::StructurePlugin)
        .add_plugin(tiles::TilePlugin)
        .add_plugin(ui::UIPlugin)
        .add_system_set(
            SystemSet::on_update(states::GameState::LoadAssets)
                .with_system(assets::check_asset_loading)
        )
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(states::GameState::BoardSetup)
                .with_system(set_camera)
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn set_camera(
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mid = 0.5 * globals::MAP_SIZE as f32 * globals::TILE_SIZE;
        transform.translation = Vec3::new(mid, mid, transform.translation.z);
    };
}