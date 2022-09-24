use bevy::prelude::*;
use bevy::asset::LoadState;

#[derive(Default)]
pub struct AssetList(pub Vec<HandleUntyped>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut game_state: ResMut<State<crate::states::GameState>>
) {
    match asset_server.get_group_load_state(
        asset_list.0.iter().map(|a| a.id)
    ) {
        LoadState::Loading => {info!("Waiting for assets.")},
        LoadState::Loaded => {
            game_state.set(crate::states::GameState::BoardSetup);
        },
        LoadState::Failed => {
            error!("asset loading error");
        },
        _ => {}
    };
}