use bevy::{
    prelude::*,
    render,
    sprite
};

use crate::globals::{BOARD_UI_Z, TILE_SIZE};
use crate::vectors::Vector2Int;
use crate::utils::v2iv3;

const PATH_HALF_WIDTH: f32 = 1.;
const PATH_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct UIPath;

pub fn draw_path(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<TurnUIAssets>
) {
    let positions = vec![
        Vector2Int::new(0, 0),
        Vector2Int::new(0, 3),
        Vector2Int::new(2, 3),
    ];
    let mesh = create_path_mesh(positions);

    commands.spawn_bundle(sprite::MaterialMesh2dBundle {
        mesh: sprite::Mesh2dHandle(meshes.add(mesh.unwrap())),
        material: assets.material.clone(),
        transform: Transform::default()
            .with_translation(Vec3::new(0., 0., BOARD_UI_Z)),
        ..Default::default()
    })
    .insert(UIPath);
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