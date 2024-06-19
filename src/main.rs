mod cell;
mod camera;
mod timer;

use bevy::prelude::*;
use cell::CellPlugin;
use camera::CameraPlugin;
use timer::{AllowNextFrame, TimerPlugin};

fn main() {
    let app = App::new()
        .add_plugins((CameraPlugin,DefaultPlugins, CellPlugin))
        .run();
}