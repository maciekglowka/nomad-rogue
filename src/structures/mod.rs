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

// fn place_structure(
//     mut commands: Commands,
//     mut ev_place: EventReader<StructureEvent>,
//     mut struct_query: Query<&Structure>,
//     element_query: Query<&TileElement>,
//     assets: Res<StructureAssets>
// ) {
//     for ev in ev_place.iter() {
//         let (entity, v) = match ev.0 {
//             StructureEventType::Place(e, v) => (e, v),
//             _ => continue
//         };

//         // TODO change to use tile assets map ?
//         if element_query.iter()
//             .find(|e| e.v == v)
//             .is_some() {
//                 continue;
//             }

//         if let Ok(structure) = struct_query.get(entity) {
//             // let element = commands.spawn_bundle(
//             //     renderer::get_structure_renderer(v, &assets, structure.kind)
//             // )
//             // .insert(TileElement {v})
//             // .id();
//             // commands.entity(entity).push_children(&[element]);
//             commands.entity(entity)
//                 .insert_bundle(
//                     renderer::get_structure_renderer(v, &assets, structure.kind)
//                 )
//                 .insert(TileElement {v});
//         }
//     }
// }

// fn unplace_structure(
//     mut commands: Commands,
//     mut ev_unplace: EventReader<UnPlaceStructureEvent>,
//     // element_query: Query<Entity, With<TileElement>>,
//     // struct_query: Query<&Children, With<Structure>>,
// ) {
//     for ev in ev_unplace.iter() {
//         // let children = match struct_query.get(ev.0) {
//         //     Ok(c) => c,
//         //     Err(_) => continue
//         // };

//         // for &child in children.iter() {
//         //     if let Ok(element) = element_query.get(child) {
//         //         commands.entity(element)
//         //             .despawn_recursive();
//         //     }
//         // }
//         commands.entity(ev.0)
//             .remove_bundle::<renderer::BUNDLE_TYPE>()
//             .remove::<TileElement>();
//     }
// }

pub fn spawn_structure(
    mut commands: &mut Commands,
    kind: StructureKind
) {
    commands.spawn()
    .insert(Structure {
        kind
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
   pub kind: StructureKind
}