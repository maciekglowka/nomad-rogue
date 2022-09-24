use bevy::prelude::*;

use crate::globals::{CURSOR_Z, TILE_SIZE};
use crate::manager::{CommandEvent, CommandType};
use crate::structures;
use crate::tiles;

use super::{
    cursor::Cursor,
    InputAssets, 
    InputType
};

const BUTTON_WIDTH: Val = Val::Px(32.);
const BUTTON_HEIGHT: Val = Val::Px(32.);
const BUTTON_MARGIN: Val = Val::Px(16.);
const BUTTON_COLOR: Color = Color::WHITE;
const SIDEBAR_WIDTH: Val = Val::Px(64.);
const ZERO: Val = Val::Px(0.);

#[derive(Component)]
pub struct StructuresMenu;

pub fn keys(
    keys: Res<Input<KeyCode>>,
    mut ev_command: EventWriter<CommandEvent>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        ev_command.send(CommandEvent(CommandType::SetInputMode(InputType::None)))
    }
}

pub fn mouse(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut assets: ResMut<InputAssets>,
    interactions: Query<(&Interaction, &StructureButton)>,
    mut cursor_query: Query<&mut Transform, With<Cursor>>,
    mut ev_command: EventWriter<CommandEvent>
) {
    let mut menu_interaction = false;
    for (interaction, button) in interactions.iter() {
        match interaction {
            Interaction::Clicked => {
                assets.current_input = InputType::PlaceStructure(button.entity);
                menu_interaction = true;       
            },
            _ => ()
        }
    }
    if menu_interaction { return; }

    if let Some(world_v) = super::mouse_to_world(&windows, &camera_query) {
        let v = super::world_to_tile_position(world_v);
        if let Ok(mut cursor_transform) = cursor_query.get_single_mut() {
            cursor_transform.translation = Vec3::new(
                v.x as f32 * TILE_SIZE,
                v.y as f32 * TILE_SIZE,
                CURSOR_Z
            );
        }

        if buttons.just_pressed(MouseButton::Left) {
            match assets.current_input {
                InputType::PlaceStructure(e) => ev_command.send(
                    CommandEvent(CommandType::PlaceStructure(e, v))
                ),
                _ => ()
            }
        }
    }
}

pub fn init(
    mut commands: Commands,
    struct_query: Query<(Entity, &structures::Structure), Without<tiles::TileElement>>
) {
    draw_structures_menu(&mut commands, &struct_query);
}

pub fn reload(
    mut commands: Commands,
    mut ev: EventReader<super::ReloadUIEvent>,
    menu_query: Query<Entity, With<StructuresMenu>>,
    struct_query: Query<(Entity, &structures::Structure), Without<tiles::TileElement>>
) {
    for _ in ev.iter() {
        clear_menus(&mut commands, &menu_query);
        draw_structures_menu(&mut commands, &struct_query);
    }
}

fn clear_menus(
    mut commands: &mut Commands,
    query: &Query<Entity, With<StructuresMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive()
    }
}

fn draw_structures_menu(
    mut commands: &mut Commands,
    struct_query: &Query<(Entity, &structures::Structure), Without<tiles::TileElement>>
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(SIDEBAR_WIDTH, Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect::new(Val::Auto, ZERO, ZERO, ZERO),
                ..Default::default()
            },
            color: Color::GRAY.into(),
            ..Default::default()
        })
        .insert(StructuresMenu)
        .with_children(|parent| {
            for (entity, structure) in struct_query.iter() {
                let color = structures::get_structure_color(structure.kind);
                parent.spawn_bundle(
                    get_structure_button_bundle(color)
                )
                .insert(StructureButton {entity });
            }
            // for (idx, card) in assets.card_queue.iter().enumerate() {
            //     let color = match card.action {
            //         CardAction::ChangeBuilding(b) => {
            //             crate::buildings::get_building_color(b.building_type, 1)
            //         }
            //     };
            //     parent
            //         .spawn_bundle(get_card_bundle(color))
            //         .insert(CardButton { card_idx: idx });
            // }
        });
}

fn get_structure_button_bundle(color: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(BUTTON_WIDTH, BUTTON_HEIGHT),
            margin: UiRect::new(ZERO, ZERO, BUTTON_MARGIN, ZERO),
            ..Default::default()
        },
        color: color.into(),
        ..Default::default()
    }
}

#[derive(Component, Debug)]
pub struct StructureButton {
    pub entity: Entity
}
