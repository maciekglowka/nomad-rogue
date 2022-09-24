use bevy::prelude::*;

use crate::structures::{spawn_structure, Structure, StructureKind};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            PlayerAssets { }
        );
        app.add_startup_system(player_init);
    }
}

fn player_init(
    mut commands: Commands
) {
    for _ in 0..crate::globals::INIT_STRUCTURES {
        spawn_structure(&mut commands, StructureKind::Gatherer);
    }
}

pub struct PlayerAssets {
}
