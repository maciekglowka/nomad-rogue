use bevy::prelude::*;

use crate::structures;
use crate::tiles;
use crate::ui::{InputType, InputAssets, ReloadUIEvent};

use super::{CommandEvent, CommandType};

pub fn select_structure(
    mut ev_command: EventReader<CommandEvent>,
    mut input_assets: ResMut<InputAssets>,
    struct_query: Query<(Entity, &tiles::TileElement), With<structures::Structure>>,
    // mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::SelectStructure(v) = ev.0 {
            let structure = struct_query.iter()
                .find(|(_, e)| e.v == v);

            if let Some((entity, _)) = structure {
                input_assets.current_input = InputType::AssignStructure(entity);
            }            
        }
    }
}

pub fn assign_structure(
    mut ev_command: EventReader<CommandEvent>,
    mut struct_query: Query<(Entity, &mut structures::Structure), With<tiles::TileElement>>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::AssignStructure(entity, v) = ev.0 {
            if let Ok((_, mut structure)) = struct_query.get_mut(entity) {
                structure.target = Some(v);
                ev_ui.send(ReloadUIEvent);
            }       
        }
    }
}
