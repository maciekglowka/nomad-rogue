use bevy::prelude::*;

use crate::manager::{CommandEvent, CommandType};
use crate::states::GameState;
use crate::structures;
use crate::tiles;

use super::{
    cursor::CursorAssets,
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
    mut keys: ResMut<Input<KeyCode>>,
    mut ev_command: EventWriter<CommandEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        ev_command.send(CommandEvent(CommandType::SetInputMode(InputType::None)))
    }
    if keys.just_pressed(KeyCode::Return) {
        game_state.set(crate::states::GameState::TurnPlanning);
        keys.reset_all();
    }
}

pub fn mouse(
    buttons: Res<Input<MouseButton>>,
    mut assets: ResMut<InputAssets>,
    interactions: Query<(&Interaction, &StructureButton)>,
    cursor: Res<CursorAssets>,
    mut ev_command: EventWriter<CommandEvent>
) {
    let mut menu_interaction = false;
    for (interaction, button) in interactions.iter() {
        match interaction {
            Interaction::Clicked => {
                ev_command.send(CommandEvent(
                    CommandType::SetInputMode(InputType::PlaceStructure(button.entity))
                ));
                menu_interaction = true;       
            },
            _ => ()
        }
    }
    if menu_interaction { return; }

    if buttons.just_pressed(MouseButton::Left) {
            if let Some(v) = cursor.v {
            match assets.current_input {
                InputType::PlaceStructure(e) => ev_command.send(
                    CommandEvent(CommandType::PlaceStructure(e, v))
                ),
                InputType::None => ev_command.send(
                    CommandEvent(CommandType::UnplaceStructure(v))
                ),
                _ => ()
            }
        }
    }
}

pub fn init(
    mut commands: Commands,
    struct_query: Query<(Entity, &structures::Structure), Without<tiles::TileElement>>,
    assets: Res<InputAssets>
) {
    draw_structures_menu(&mut commands, &struct_query, &assets);
}

pub fn clear(
    mut commands: Commands,
    menu_query: Query<Entity, With<StructuresMenu>>
) {
    clear_menus(&mut commands, &menu_query);
}

pub fn reload(
    mut commands: Commands,
    mut ev: EventReader<super::ReloadUIEvent>,
    menu_query: Query<Entity, With<StructuresMenu>>,
    struct_query: Query<(Entity, &structures::Structure), Without<tiles::TileElement>>,
    assets: Res<InputAssets>
) {
    for _ in ev.iter() {
        clear_menus(&mut commands, &menu_query);
        draw_structures_menu(&mut commands, &struct_query, &assets);
    }
}

fn clear_menus(
    commands: &mut Commands,
    query: &Query<Entity, With<StructuresMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive()
    }
}

fn draw_structures_menu(
    commands: &mut Commands,
    struct_query: &Query<(Entity, &structures::Structure), Without<tiles::TileElement>>,
    assets: &InputAssets
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
                let selected = match assets.current_input {
                    InputType::PlaceStructure(e) => e == entity,
                    _ => false
                };
                parent.spawn_bundle(
                    get_structure_button_bundle(color, selected)
                )
                .insert(StructureButton {entity });
            }
        });
}

fn get_structure_button_bundle(mut color: Color, selected: bool) -> ButtonBundle {
    if selected {
        color.set_a(0.5);
    }
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

