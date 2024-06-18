mod cell;
mod camera;

use bevy::prelude::*;
use cell::CellPlugin;
use camera::CameraPlugin;

fn main() {
    let app = App::new()
        .add_plugins((CameraPlugin,DefaultPlugins, cell::CellPlugin))
        .run();
}
