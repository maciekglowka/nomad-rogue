use bevy::prelude::*;

use crate::states::GameState;
use crate::structures;
use crate::tiles;
use crate::ui::{InputType, InputAssets, ReloadUIEvent};
use crate::vectors::Vector2Int;

pub struct CommandEvent(pub CommandType);

pub enum CommandType {
    SetInputMode(InputType),
    PlaceStructure(Entity, Vector2Int)
}

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::BoardSetup)
                .with_system(set_input_type)
                .with_system(place_structure)
        );
    }
}

fn set_input_type(
    mut ev_command: EventReader<CommandEvent>,
    mut input_assets: ResMut<InputAssets>,
) {
    for ev in ev_command.iter() {
        if let CommandType::SetInputMode(mode) = ev.0 {
            input_assets.current_input = mode;
        }
    }
}

fn place_structure(
    mut commands: Commands,
    mut ev_command: EventReader<CommandEvent>,
    mut input_assets: ResMut<InputAssets>,
    struct_query: Query<&structures::Structure>,
    element_query: Query<&tiles::TileElement>,
    struct_assets: Res<structures::StructureAssets>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::PlaceStructure(entity, v) = ev.0 {
            if structures::place_structure(
                &mut commands,
                entity,
                v,
                &struct_query,
                &element_query,
                struct_assets.as_ref()
            ) {
                ev_ui.send(ReloadUIEvent);
                input_assets.current_input = InputType::None;
            }
        }
    }
}
