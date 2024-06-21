use bevy::{
    ecs::query::{self, QuerySingleError}, input::mouse::{MouseMotion, MouseWheel}, prelude::*
};

const CAMERA_DISTANCE: f32 = 24.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Zoom(1.0))
        
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_zoom)
            .add_systems(Update, camera_movement)
            .add_systems(Startup, grid_test_system)
            .add_systems(Update,gris_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });
}

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


fn gris_system(
    mut grid_query: Query<(&mut Transform,&mut Sprite,&Grid), (With<Grid>,Without<Camera>)>,
    mut camera_query: Query<(&OrthographicProjection,&Transform),(Without<Grid>)>){


        for (projection, camera_transform) in &mut camera_query {
    
            for (mut grid,mut sprite, grid_id) in &mut grid_query {
                let screen_size = (projection.area.max - projection.area.min + Vec2{x:200.0,y:200.0});
                info!("screen_size {}",screen_size);
                sprite.custom_size = Some(screen_size/2.0);
                let snapped_camera_pos = Vec2{ x: quotient(camera_transform.translation.x , GRID_CELL_SIDE) * GRID_CELL_SIDE, y: quotient(camera_transform.translation.y , GRID_CELL_SIDE) * GRID_CELL_SIDE};
                info!("snapped_camera_pos {}    camera_pos{}",snapped_camera_pos, camera_transform.translation);
                let move_vector = screen_size/4.0;
                info!("move_vector {}",move_vector);
                let side_vector = Vec2{x: -1.0 + 2.0 * ((grid_id.0 >> 1) & 1) as f32 ,y:-1.0 + 2.0 * (grid_id.0 & 1) as f32};
                if (((grid_id.0 >> 1) + 1) & 1 == 1){
                    sprite.flip_x = true;
                }
                if ((grid_id.0 & 1) == 1){
                    sprite.flip_y = true;
                }
                info!("side_vector {}   id {}",side_vector,grid_id.0);
                let pos_vec = snapped_camera_pos + (move_vector * side_vector);
                info!("pos_vec {}",pos_vec);
                grid.translation = Vec3{ x: pos_vec.x, y: pos_vec.y, z: -1.0 };
                info!("-----------------------------------------------------------------------------------------------")
                // grid.translation = camera_transform.translation;
            }
        }
}


fn quotient(a:f32,b:f32) -> f32{
    let rem= a % b;
    let c = a - rem ;
    c / b
}

fn camera_movement(
    buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut camera_transform in &mut query {
        if buttons.pressed(MouseButton::Middle) {
            for ev in evr_motion.read() {
                camera_transform.translation += Vec3 {
                    x: ev.delta.x * -1.0,
                    y: ev.delta.y,
                    z: 0.0,
                }
            }
        }
    }
}





fn grid_test_system(mut commands: Commands, asset_server: Res<AssetServer>){
    for i in 0..=4 {
        let b = (SpriteBundle {
            texture: asset_server.load("grid.png"),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0, // The image will tile every 128px
        },
        Grid(i)
        );
        commands.spawn(b);
    }
}

#[derive(Resource)]
struct Zoom (f32);


const GRID_CELL_SIDE: f32 = 100.0;
#[derive(Component)]
struct Grid(u8);



// fn animate(mut sprites: Query<&mut Sprite,With<Grid>>, mut state: ResMut<AnimationState>, time: Res<Time>) {
//     if state.current >= state.max || state.current <= state.min {
//         state.speed = -state.speed;
//     };
//     state.current += state.speed * time.delta_seconds();
//     for mut sprite in &mut sprites {
//         sprite.custom_size = Some(Vec2::splat(state.current));
//     }
// }
