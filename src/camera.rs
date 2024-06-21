use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

// const CAMERA_DISTANCE: f32 = 24.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Zoom(1.0))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_zoom)
            .add_systems(Update, camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });
}

fn camera_movement(
    buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    zoom: Res<Zoom>,
) {
    for mut camera_transform in &mut query {
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

#[derive(Resource)]
struct Zoom(f32);

fn camera_zoom(
    mut evr_scroll: EventReader<MouseWheel>,
    mut zoom: ResMut<Zoom>,
    mut camera_query: Query<&mut OrthographicProjection>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                zoom.0 -= 0.1 * ev.y; //? add delta seconds?
            }
            MouseScrollUnit::Pixel => {
                zoom.0 -= 0.1 * ev.y; //? add delta seconds?
            }
        }
    }

    for mut projection in &mut camera_query {
        projection.scale = zoom.0; //? add delta seconds?
    }
}
