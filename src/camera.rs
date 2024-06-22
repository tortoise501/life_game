use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use num::traits::clamp_min;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Zoom(1.0))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_zoom)
            .add_systems(Update, camera_movement_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });
}

/// Camera movement on wheel hold
fn camera_movement_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut camera_q: Query<&mut Transform, With<Camera>>,
    zoom: Res<Zoom>,
) {
    for mut camera_transform in &mut camera_q {
        if buttons.pressed(MouseButton::Middle) {
            for ev in evr_motion.read() {
                camera_transform.translation += Vec3 {
                    x: ev.delta.x * -1.0 * zoom.0,
                    y: ev.delta.y * zoom.0,
                    z: 0.0,
                }
            }
        }
    }
}
/// Zoom ratio
#[derive(Resource)]
struct Zoom(f32);


/// Camera zoom on scroll
fn camera_zoom(
    mut evr_scroll: EventReader<MouseWheel>,
    mut zoom: ResMut<Zoom>,
    mut camera_query: Query<&mut OrthographicProjection>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line | MouseScrollUnit::Pixel => {
                zoom.0 = clamp_min(zoom.0 - 0.1 * ev.y,0.5);
            }
        }
    }

    for mut projection in &mut camera_query {
        projection.scale = zoom.0; //? add delta seconds?
    }
}
