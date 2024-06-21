mod cell;
mod camera;
mod timer;
mod grid;
mod cell_spawner;

use bevy::prelude::*;
use cell::CellPlugin;
use camera::CameraPlugin;
use grid::GridPlugin;
use cell_spawner::CellSpawnerPlugin;

fn main() {
    let _app = App::new()
        .add_plugins((DefaultPlugins, CellPlugin,CameraPlugin,GridPlugin,CellSpawnerPlugin))
        .run();
}