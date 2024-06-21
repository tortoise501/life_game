mod camera;
mod cell;
mod cell_spawner;
mod grid;
mod timer;

use bevy::prelude::*;
use camera::CameraPlugin;
use cell::CellPlugin;
use cell_spawner::CellSpawnerPlugin;
use grid::GridPlugin;

fn main() {
    let _app = App::new()
        .add_plugins((
            DefaultPlugins,
            CellPlugin,
            CameraPlugin,
            GridPlugin,
            CellSpawnerPlugin,
        ))
        .run();
}
