use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid)
            .add_systems(Update, update_grid);
    }
}

fn update_grid(
    mut grid_query: Query<(&mut Transform, &mut Sprite, &Grid), (With<Grid>, Without<Camera>)>,
    camera_query: Query<(&OrthographicProjection, &Transform), Without<Grid>>,
) {
    let (projection, camera_transform) = camera_query.single();

    for (mut grid, mut sprite, grid_id) in &mut grid_query {
        //screen size in window pixels with 200 additional pixels to work with camera snapping
        let screen_size = projection.area.max - projection.area.min + Vec2 { x: 200.0, y: 200.0 };
        //change sprite of each grid sprite to be a quarter of a screen
        sprite.custom_size = Some(screen_size / 2.0);
        //snap camera position depending on cell size
        let snapped_camera_pos = Vec2 {
            x: quotient(camera_transform.translation.x, GRID_CELL_SIDE) * GRID_CELL_SIDE,
            y: quotient(camera_transform.translation.y, GRID_CELL_SIDE) * GRID_CELL_SIDE,
        };
        //vector to move grid block from camera center to screen quarter
        let move_vector = screen_size / 4.0;
        //vector used to redirect move_vector to other quarters depending on ID
        let side_vector = Vec2 {
            x: -1.0 + 2.0 * ((grid_id.0 >> 1) & 1) as f32,
            y: -1.0 + 2.0 * (grid_id.0 & 1) as f32,
        };
        //flips grid blocks
        //needed because texture is tiled from top left corner
        if ((grid_id.0 >> 1) + 1) & 1 == 1 {
            sprite.flip_x = true;
        }
        if (grid_id.0 & 1) == 1 {
            sprite.flip_y = true;
        }
        //calculate position for grid block
        let pos_vec = snapped_camera_pos + (move_vector * side_vector);
        grid.translation = Vec3 {
            x: pos_vec.x + 0.5 * GRID_CELL_SIDE,
            y: pos_vec.y + 0.5 * GRID_CELL_SIDE,
            z: -1.0,
        };
    }
}

/// Spawns four grid entities
fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..=4 {
        let b = (
            SpriteBundle {
                texture: asset_server.load("grid.png"),
                ..default()
            },
            //tiled to make it scale with zoom more efficiently
            ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            Grid(i),
        );
        commands.spawn(b);
    }
}
/// Width of grid sprite in pixels
const GRID_CELL_SIDE: f32 = 100.0;
/// Grid marker with id
#[derive(Component)]
struct Grid(u8);

// like % but returns quotient
use std::ops::{Rem,Div,Sub};
pub fn quotient<T:Rem<Output = T> + Div<Output = T> + Sub<Output = T> + Copy>(a: T, b: T) -> T {
    let rem = a % b;
    let c = a - rem;
    c / b
}
