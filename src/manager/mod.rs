use bevy::prelude::*;

use crate::states::GameState;
use crate::ui::{InputType, InputAssets, ReloadUIEvent};
use crate::vectors::Vector2Int;

mod board_setup;
mod turns;

pub struct CommandEvent(pub CommandType);

pub enum CommandType {
    SetInputMode(InputType),
    PlaceStructure(Entity, Vector2Int),
    UnplaceStructure(Vector2Int),
    SelectStructure(Vector2Int),
    AssignStructure(Entity, Vector2Int),
}

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>();
        app.add_system_set(
            SystemSet::on_update(GameState::BoardSetup)
                .after("ui")
                .with_system(set_input_type)
                .with_system(board_setup::place_structure)
                .with_system(board_setup::unplace_structure)
        );

        app.add_system_set(
            SystemSet::on_update(GameState::TurnPlanning)
                .after("ui")
                .with_system(set_input_type)
                .with_system(turns::select_structure)
                .with_system(turns::assign_structure)
        );
    }
}

fn set_input_type(
    mut ev_command: EventReader<CommandEvent>,
    mut input_assets: ResMut<InputAssets>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::SetInputMode(mode) = ev.0 {
            input_assets.current_input = mode;
            ev_ui.send(ReloadUIEvent);
        }
    }
}

