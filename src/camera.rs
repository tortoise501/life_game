use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 24.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 30.0, 0.0),
        ..default()
    });
}
