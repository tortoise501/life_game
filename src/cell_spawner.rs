use crate::cell::{CellBundle, CELL_WIDTH};
use bevy::prelude::*;

pub struct CellSpawnerPlugin;

impl Plugin for CellSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_on_cursor);
    }
}

fn spawn_on_cursor(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let snapped_pos = Vec2 {
                x: (world_position.x / CELL_WIDTH).round(),
                y: (world_position.y / CELL_WIDTH).round(),
            };
            let bun = CellBundle::from_coords(snapped_pos, &asset_server);
            info!("{}", snapped_pos);
            commands.spawn(bun);
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        }
        info!("well");
    }
}
