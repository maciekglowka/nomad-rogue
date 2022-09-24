use bevy::prelude::*;

use crate::globals::{CURSOR_Z, TILE_SIZE};
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct UIPlugin;

mod board_setup;
mod cursor;

pub struct ReloadUIEvent;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            InputAssets { current_input: InputType::None }
        );
        app.add_event::<ReloadUIEvent>();
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            cursor::load_assets
        );
        app.add_startup_system(cursor::spawn_cursor);
        app.add_event::<cursor::UpdateCursorEvent>();
        app.add_system(cursor::update_cursor);

        app.add_system_set(
            SystemSet::on_enter(GameState::BoardSetup)
                .with_system(board_setup::init)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::BoardSetup)
                .with_system(board_setup::keys)
                .with_system(board_setup::mouse)
                .with_system(board_setup::reload)
        );

    }
}


fn mouse_to_world(
    windows: &Res<Windows>,
    camera_query: &Query<(&Camera, &GlobalTransform)>
) -> Option<Vec2> {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(c) => c,
        Err(_) => return None
    };
    let window = windows.get_primary()?;

    let screen_pos = window.cursor_position()?;
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (screen_pos / window_size) * 2. - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    Some(ndc_to_world.project_point3(ndc.extend(-1.)).truncate())
}

fn world_to_tile_position(v: Vec2) -> Vector2Int {
    let x = (v.x / TILE_SIZE - 0.5).ceil();
    let y = (v.y / TILE_SIZE - 0.5).ceil();
    Vector2Int::new(x as i32, y as i32)
}

pub struct InputAssets {
    pub current_input: InputType
}

#[derive(Clone, Copy)]
pub enum InputType {
    None,
    PlaceStructure(Entity)
}