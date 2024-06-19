use std::{thread::sleep, time::Duration};

use bevy::{
    prelude::*,
    utils::{hashbrown::HashMap, info, petgraph::adj::Neighbors},
};

const CELL_WIDTH: f32 = 100.0;
#[derive(Bundle)]
struct CellBundle {
    sprite: SpriteBundle,
    state: CellState,
    neighbors: CellLivingNeighborsCount,
    coords: CellCoordinates,
}
impl CellBundle {
    fn from_coords(coords: Vec2, asset_server: &AssetServer) -> CellBundle {
        CellBundle {
            sprite: SpriteBundle {
                transform: Transform::from_xyz(coords.x * CELL_WIDTH, coords.y * CELL_WIDTH, 0.0),
                texture: asset_server.load("cell.png"),
                ..default()
            },
            state: CellState::Unsettled,
            neighbors: CellLivingNeighborsCount(0),
            coords: CellCoordinates(IVec2 {
                x: coords.x as i32,
                y: coords.y as i32,
            }),
        }
    }
}

#[derive(Component, Debug, Clone)]
enum CellState {
    Alive,
    Dead,
    Unsettled,
}

#[derive(Component, Debug)]
struct CellLivingNeighborsCount(u32);

#[derive(Component, Debug)]
struct CellCoordinates(IVec2);

fn update_marks_system(mut query: Query<(&mut CellState, &CellLivingNeighborsCount)>,mut next_state: ResMut<NextState<crate::timer::AllowNextFrame>>) {
    //info!("starting marks");
    for (mut state, neighbors) in &mut query {
        let curr_state = state.clone();
        *state = match neighbors.0 {
            0 => CellState::Dead,
            2 => curr_state,
            3 => CellState::Alive,
            _ => CellState::Unsettled,
        }
    }
    next_state.set(crate::timer::AllowNextFrame::No);
}
fn update_neighbors_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&CellCoordinates, &CellState, &mut CellLivingNeighborsCount)>,
) {
    //info!("starting neighbors");
    let mut new_cells: HashMap<IVec2, u32> = HashMap::new();
    for (coords, state, neighbors) in &mut query {
        if let CellState::Alive = state {
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let current_coord = IVec2 {
                        x: x + coords.0.x,
                        y: y + coords.0.y,
                    };
                    //info!("{:?}", coords.0);
                    let a = new_cells.entry(current_coord);
                    *a.or_insert(0) += 1;
                    //info!("{:?}", new_cells.get_key_value(&current_coord));
                }
            }
        }
    }
    //info!("{:?}", new_cells.capacity());
    for (coords, state, mut neighbors) in &mut query {
        match new_cells.get(&IVec2 {
            x: coords.0.x,
            y: coords.0.y,
        }) {
            Some(new_neighbors) => neighbors.0 = *new_neighbors,
            None => (), //?IDK
        }
        new_cells.remove(&IVec2 {
            x: coords.0.x,
            y: coords.0.y,
        });
    }

    for (coords, neighbors) in new_cells {
        let mut cell = CellBundle::from_coords(
            Vec2 {
                x: coords.x as f32,
                y: coords.y as f32,
            },
            &asset_server,
        );
        cell.neighbors.0 = neighbors;
        commands.spawn(cell);
    }
}

fn clear_dead_cells_system(mut commands: Commands, query: Query<(Entity, &CellState)>) {
    // //info!("starting cleaning");
    for (cell, state) in &query {
        match state {
            CellState::Dead => commands.entity(cell).despawn(),
            _ => (),
        }
    }
}
fn update_cells_visuals(mut query: Query<(&mut Visibility, &CellState)>) {
    // //info!("starting visuals");
    for (mut visibility, state) in &mut query {
        match state {
            CellState::Alive => *visibility = Visibility::Visible,
            CellState::Dead => *visibility = Visibility::Hidden,
            CellState::Unsettled => *visibility = Visibility::Hidden,
        }
    }
}

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(crate::timer::TimerPlugin)
            .init_state::<crate::timer::AllowNextFrame>()
            .add_systems(Startup, spawn_cells)
            .add_systems(
                Update,
                (
                    update_neighbors_system,
                    update_marks_system,
                    clear_dead_cells_system,
                    update_cells_visuals,
                )
                    .chain()
                    .run_if(in_state(crate::timer::AllowNextFrame::Yes)),
            );
    }
}

fn spawn_cells(mut commands: Commands, asset_server: Res<AssetServer>) {
    for y in 1..=3 {
        let mut cell = CellBundle::from_coords(
            Vec2 {
                x: 0.0,
                y: y as f32,
            },
            &asset_server,
        );
        cell.state = CellState::Alive;
        commands.spawn(cell);
    }


    let mut cell = CellBundle::from_coords(
        Vec2 {
            x: -1.0,
            y: 1.0,
        },
        &asset_server,
    );
    cell.state = CellState::Alive;
    commands.spawn(cell);


    let mut cell = CellBundle::from_coords(
        Vec2 {
            x: -2.0,
            y: 2.0,
        },
        &asset_server,
    );
    cell.state = CellState::Alive;
    commands.spawn(cell);
}

// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum WaitForFrame {
//     #[default]
//     Waiting,
//     Blocking,
// }
