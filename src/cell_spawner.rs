use crate::cell::{CellBundle, CellState, CELL_WIDTH};
use bevy::{ecs::query, prelude::*, render::view::visibility};

pub struct CellSpawnerPlugin;

impl Plugin for CellSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorWorldPosition(Vec2{x:0.,y:0.})).
        add_systems(Update, (remove_on_cursor,spawn_on_cursor,update_cursor_world_position))   ;
    }
}

fn spawn_on_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cell_q: Query<(Entity, &Transform), With<crate::cell::CellState>>,
    cursor_pos: Res<CursorWorldPosition>
) {
    if buttons.just_pressed(MouseButton::Left) {
        
        let snapped_pos = Vec2 {
            x: (cursor_pos.0.x / CELL_WIDTH).round(),
            y: (cursor_pos.0.y / CELL_WIDTH).round(),
        };
        let bun = CellBundle::from_coords(snapped_pos, &asset_server);
        for (cell, cell_transform) in &mut cell_q {
            if cell_transform.translation == bun.sprite.transform.translation{
                commands.entity(cell).despawn();
            }
        }
        commands.spawn(bun);
        
    }
}


#[derive(Resource)]
struct CursorWorldPosition(Vec2);

fn update_cursor_world_position(
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    windows: Query<&Window>,
    mut pos: ResMut<CursorWorldPosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        pos.0 = world_position;   
    }
}


fn remove_on_cursor(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cell_q: Query<(Entity, &Transform), With<crate::cell::CellState>>,
    cursor_pos: Res<CursorWorldPosition>
) {

    if buttons.just_pressed(MouseButton::Right) {

        let snapped_pos = Vec2 {
            x: (cursor_pos.0.x / CELL_WIDTH).round() * CELL_WIDTH,
            y: (cursor_pos.0.y / CELL_WIDTH).round() * CELL_WIDTH,
        };
        for (entity, transform) in &mut cell_q {
            if transform.translation == (Vec3{x:snapped_pos.x,y:snapped_pos.y,z:transform.translation.z}){
                commands.entity(entity).despawn();
            }
        }
        
    }
}