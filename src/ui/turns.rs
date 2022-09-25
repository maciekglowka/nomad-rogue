use bevy::{
    prelude::*,
    render,
    sprite
};

use crate::globals::{BOARD_UI_Z, TILE_SIZE};
use crate::manager::{CommandEvent, CommandType};
use crate::states::GameState;
use crate::structures;
use crate::tiles;
use crate::vectors::Vector2Int;
use crate::utils::v2iv3;

use super::{
    cursor::CursorAssets,
    InputAssets, 
    InputType
};

const PATH_HALF_WIDTH: f32 = 1.;
const PATH_COLOR: Color = Color::WHITE;

pub fn keys(
    mut keys: ResMut<Input<KeyCode>>,
    mut ev_command: EventWriter<CommandEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        ev_command.send(CommandEvent(CommandType::SetInputMode(InputType::None)))
    }
    if keys.just_pressed(KeyCode::Return) {
        game_state.set(crate::states::GameState::TurnRun);
        keys.reset_all();
    }
}

pub fn mouse(
    buttons: Res<Input<MouseButton>>,
    cursor: Res<CursorAssets>,
    mut assets: ResMut<InputAssets>,
    // mut cursor_query: Query<&mut Transform, With<Cursor>>,
    mut ev_command: EventWriter<CommandEvent>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(v) = cursor.v {
            match assets.current_input {
                InputType::None => ev_command.send(
                    CommandEvent(CommandType::SelectStructure(v))
                ),
                InputType::AssignStructure(entity) => ev_command.send(
                    CommandEvent(CommandType::AssignStructure(entity, v))
                ),
                _ => ()
            }
        }
    }
}

pub fn clear(
    mut commands: Commands,
    path_query: Query<Entity, With<StructurePath>>
) {
    remove_elements(&mut commands, &path_query);
}

fn remove_elements(
    commands: &mut Commands,
    query: &Query<Entity, With<StructurePath>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive()
    }
}

pub fn reload(
    mut commands: Commands,
    mut ev: EventReader<super::ReloadUIEvent>,
    struct_query: Query<(&structures::Structure, &tiles::TileElement)>,
    path_query: Query<Entity, With<StructurePath>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<TurnUIAssets>
    // assets: Res<InputAssets>
) {
    for _ in ev.iter() {
        remove_elements(&mut commands, &path_query);

        for (structure, element) in struct_query.iter() {
            if let Some(target) = structure.target {
                let positions = get_structure_path(element.v, target);
                draw_path(&mut commands, positions, &mut meshes, &assets);
            }
        }
    }
}

fn get_structure_path(origin: Vector2Int, target: Vector2Int) -> Vec<Vector2Int> {
    let mut v = vec![origin];

    if origin.x != target.x || origin.y != target.y {
        v.push(Vector2Int::new(origin.x, target.y));
    }

    v.push(target);
    v
}

#[derive(Component)]
pub struct StructurePath;

pub fn draw_path(
    mut commands: &mut Commands,
    positions: Vec<Vector2Int>,
    mut meshes: &mut Assets<Mesh>,
    assets: &TurnUIAssets
) {
    let mesh = create_path_mesh(positions);

    commands.spawn_bundle(sprite::MaterialMesh2dBundle {
        mesh: sprite::Mesh2dHandle(meshes.add(mesh.unwrap())),
        material: assets.material.clone(),
        transform: Transform::default()
            .with_translation(Vec3::new(0., 0., BOARD_UI_Z)),
        ..Default::default()
    })
    .insert(StructurePath);
}

fn create_path_mesh(positions: Vec<Vector2Int>) -> Option<Mesh> {
    if positions.len() < 2 {
        return None
    }

    let mut lines = Vec::new();
    for idx in 0..positions.len() - 1 {
        lines.push((
            v2iv3(positions[idx], 0., TILE_SIZE),
            v2iv3(positions[idx+1], 0., TILE_SIZE)
        ))
    }

    let mut verts = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut tris = Vec::new();

    let mut idx = 0;

    for line in lines {
        let dir = line.1 - line.0;
        let n = PATH_HALF_WIDTH * dir.cross(Vec3::Z).normalize();
        
        verts.push((line.0 - n).to_array());
        verts.push((line.1 - n).to_array());
        verts.push((line.1 + n).to_array());
        verts.push((line.0 + n).to_array());

        for _ in 0..4 {
            normals.push([0.0, 1.0, 0.0]);
        }

        uvs.extend(vec!(
            [0.0,0.0], [0.0,1.0], [1.0,1.0], [1.0,0.0]
        ));
        tris.extend([idx, idx + 2, idx + 1, idx, idx + 3, idx +2]);
        idx += 4;
    }

    let mut mesh = Mesh::new(render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(render::mesh::Indices::U32(tris)));
    Some(mesh)
}

pub struct TurnUIAssets {
    material: Handle<ColorMaterial>
}

pub fn load_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let material_handle = materials.add(
        ColorMaterial{ color: PATH_COLOR, texture: None}
    );
    commands.insert_resource(
        TurnUIAssets {
            material: material_handle
        }
    )
}