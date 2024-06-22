use crate::cell::{CellBundle, CELL_WIDTH};
use bevy::prelude::*;

pub struct CellSpawnerPlugin;

impl Plugin for CellSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorWorldPosition(Vec2{x:0.,y:0.})).
        add_systems(Update, (remove_cell_on_cursor,spawn_cell_on_cursor,update_cursor_world_position))   ;
    }
}

/// Position of cursor on scene 
#[derive(Resource)]
struct CursorWorldPosition(Vec2);

/// Update cursor world position resource
fn update_cursor_world_position(
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    windows: Query<&Window>,
    mut cursor_world_position: ResMut<CursorWorldPosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_world_position.0 = world_position;   
    }
}

/// Spawn cell in cursor position
fn spawn_cell_on_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPosition>,
    cell_q: Query<(Entity, &crate::cell::CellCoordinates), With<crate::cell::CellState>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        
        //cursor "position" snapped to "grid" 
        //position is cell "id"
        let snapped_pos = Vec2 {
            x: (cursor_pos.0.x / CELL_WIDTH).round(),
            y: (cursor_pos.0.y / CELL_WIDTH).round(),
        };
        let spawned_cell = CellBundle::from_coords(snapped_pos, &asset_server);

        remove_cell_at_cellpos(cell_q, snapped_pos.as_ivec2(), &mut commands);

        commands.spawn(spawned_cell);
    }
}

/// Removes cell in cursor position
fn remove_cell_on_cursor(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    cell_q: Query<(Entity, &crate::cell::CellCoordinates), With<crate::cell::CellState>>,
    cursor_pos: Res<CursorWorldPosition>
) {
    if buttons.just_pressed(MouseButton::Right) {
        //cursor "position" snapped to "grid" 
        //position is cell "id"
        let snapped_pos = IVec2 {
            x: (cursor_pos.0.x / CELL_WIDTH).round() as i32,
            y: (cursor_pos.0.y / CELL_WIDTH).round() as i32,
        };
        remove_cell_at_cellpos(cell_q, snapped_pos, &mut commands);
    }
}

/// Remove cell at given "cell position"
/// Only works correctly with CellCoordinates
fn remove_cell_at_cellpos(
    mut cell_q: Query<(Entity, &crate::cell::CellCoordinates), With<crate::cell::CellState>>,
    pos: IVec2,
    commands: &mut Commands
) {
    for (entity, cell_transform) in &mut cell_q {
        //remove 
        if cell_transform.0 == pos{
            commands.entity(entity).despawn();
        }
    }
}