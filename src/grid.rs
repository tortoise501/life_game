use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, grid_test_system)
            .add_systems(Update,gris_system);
    }
}

fn gris_system(
    mut grid_query: Query<(&mut Transform,&mut Sprite,&Grid), (With<Grid>,Without<Camera>)>,
    mut camera_query: Query<(&OrthographicProjection,&Transform),Without<Grid>>){


        for (projection, camera_transform) in &mut camera_query {
    
            for (mut grid,mut sprite, grid_id) in &mut grid_query {
                let screen_size = projection.area.max - projection.area.min + Vec2{x:200.0,y:200.0};
                // info!("screen_size {}",screen_size);
                sprite.custom_size = Some(screen_size/2.0);
                let snapped_camera_pos = Vec2{ x: quotient(camera_transform.translation.x , GRID_CELL_SIDE) * GRID_CELL_SIDE, y: quotient(camera_transform.translation.y , GRID_CELL_SIDE) * GRID_CELL_SIDE};
                // info!("snapped_camera_pos {}    camera_pos{}",snapped_camera_pos, camera_transform.translation);
                let move_vector = screen_size/4.0;
                // info!("move_vector {}",move_vector);
                let side_vector = Vec2{x: -1.0 + 2.0 * ((grid_id.0 >> 1) & 1) as f32 ,y:-1.0 + 2.0 * (grid_id.0 & 1) as f32};
                if ((grid_id.0 >> 1) + 1) & 1 == 1 {
                    sprite.flip_x = true;
                }
                if (grid_id.0 & 1) == 1 {
                    sprite.flip_y = true;
                }
                // info!("side_vector {}   id {}",side_vector,grid_id.0);
                let pos_vec = snapped_camera_pos + (move_vector * side_vector);
                // info!("pos_vec {}",pos_vec);
                grid.translation = Vec3{ x: pos_vec.x + 0.5 * GRID_CELL_SIDE, y: pos_vec.y + 0.5 * GRID_CELL_SIDE, z: -1.0 };
                // info!("-----------------------------------------------------------------------------------------------")
                // grid.translation = camera_transform.translation;
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

const GRID_CELL_SIDE: f32 = 100.0;
#[derive(Component)]
struct Grid(u8);



fn quotient(a:f32,b:f32) -> f32{
    let rem= a % b;
    let c = a - rem ;
    c / b
}