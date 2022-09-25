use bevy::prelude::*;

use crate::structures;
use crate::tiles;
use crate::ui::{InputType, InputAssets, ReloadUIEvent};

use super::{CommandEvent, CommandType};

pub fn place_structure(
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

pub fn unplace_structure(
    mut commands: Commands,
    mut ev_command: EventReader<CommandEvent>,
    struct_query: Query<(Entity, &tiles::TileElement), With<structures::Structure>>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::UnplaceStructure(v) = ev.0 {
            if structures::unplace_structure(
                &mut commands,
                v,
                &struct_query,
            ) {
                ev_ui.send(ReloadUIEvent);
            }
        }
    }
}
