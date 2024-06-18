use std::time::Duration;

use bevy::prelude::*;

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    sprite: SpriteBundle,
}

#[derive(Component, Debug)]
struct Cell {
    neighbors: u8,
}
impl Cell {
    fn new() -> Cell {
        Cell { neighbors: 0 }
    }
}

fn cell_test(query: Query<&Transform, With<Cell>>) {
    for transform in &query {
        info!("{:?}", transform);
    }
}

fn spawn_cell(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CellBundle {
        cell: Cell::new(),
        sprite: SpriteBundle {
            texture: asset_server.load("cell.png"),
            transform: Transform::from_scale(Vec3 { x: 100.0, y: 100.0, z: 100.0 }),
            ..default()
        },
    });
}

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cell)
            .add_systems(Update, cell_test);
    }
}
