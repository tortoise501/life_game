mod camera;
mod cell;
mod cell_spawner;
mod grid;
mod timer;
mod save_manager;

use bevy::prelude::*;
use camera::CameraPlugin;
use cell::CellPlugin;
use cell_spawner::CellSpawnerPlugin;
use grid::GridPlugin;
use save_manager::SavePlugin;

fn main() {
    let _app = App::new()
        .add_plugins((
            DefaultPlugins,
            CellPlugin,
            CameraPlugin,
            GridPlugin,
            CellSpawnerPlugin,
            SavePlugin
        ))
        .run();
}
