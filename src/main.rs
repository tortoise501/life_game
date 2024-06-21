mod cell;
mod camera;
mod timer;
mod grid;

use bevy::prelude::*;
use cell::CellPlugin;
use camera::CameraPlugin;

fn main() {
    let _app = App::new()
        .add_plugins((DefaultPlugins, CellPlugin,CameraPlugin))
        .run();
}