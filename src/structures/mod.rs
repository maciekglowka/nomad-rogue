use bevy::prelude::*;

use crate::states::GameState;
use crate::tiles::TileElement;
use crate::vectors::Vector2Int;

mod renderer;

pub use renderer::get_structure_color;

// pub struct StructureEvent(pub StructureEventType);

// pub enum StructureEventType {
//     Place(Entity, Vector2Int),
//     Unplace(Entity)
// }

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            renderer::load_assets
        );
        // app.add_event::<StructureEvent>();
        // app.add_system_set(
        //     SystemSet::on_update(GameState::BoardSetup)
        //         .with_system(place_structure)
        // );
    }
}

pub fn place_structure(
    mut commands: &mut Commands,
    entity: Entity,
    v: Vector2Int,
    struct_query: &Query<&Structure>,
    element_query: &Query<&TileElement>,
    assets: &StructureAssets
) -> bool {
    // TODO change to use tile assets map ?
    if element_query.iter()
        .find(|e| e.v == v)
        .is_some() {
            return false;
        }

    if let Ok(structure) = struct_query.get(entity) {;
        commands.entity(entity)
            .insert_bundle(
                renderer::get_structure_renderer(v, assets, structure.kind)
            )
            .insert(TileElement {v});
            return true
    }
    false
}

pub fn unplace_structure(
    mut commands: &mut Commands,
    v: Vector2Int,
    struct_query: &Query<(Entity, &TileElement), With<Structure>>,
) -> bool {
    if let Some((entity, _element)) = struct_query.iter()
        .find(|(_, t)| t.v == v) {
            commands.entity(entity)
                .remove_bundle::<renderer::BUNDLE_TYPE>()
                .remove::<TileElement>();
            return true;
        }
    false
}

pub fn spawn_structure(
    mut commands: &mut Commands,
    kind: StructureKind
) {
    commands.spawn()
    .insert(Structure {
        kind,
        target: None
    });
} 

pub struct StructureAssets {
    texture: Handle<TextureAtlas>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StructureKind {
    Gatherer
}

#[derive(Component)]
pub struct Structure {
   pub kind: StructureKind,
   pub target: Option<Vector2Int>
}