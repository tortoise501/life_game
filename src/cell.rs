use bevy::{prelude::*, utils::hashbrown::HashMap};

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::timer::TimerPlugin)
            .init_state::<crate::timer::AllowNextFrame>()
            .add_systems(
                Update,
                (
                    update_neighbors,
                    update_marks,
                    update_texture,
                    clear_dead_cells,
                    finish_frame
                ).chain().run_if(in_state(crate::timer::AllowNextFrame::Yes)),
            );
    }
}


pub const CELL_WIDTH: f32 = 100.0;
#[derive(Bundle)]
pub struct CellBundle {
    pub sprite: SpriteBundle,
    state: CellState,
    neighbors: CellLivingNeighborsCount,
    coords: CellCoordinates,
}
impl CellBundle {
    pub fn from_coords(coords: Vec2, asset_server: &AssetServer) -> CellBundle {
        CellBundle {
            sprite: SpriteBundle {
                transform: Transform::from_xyz(coords.x * CELL_WIDTH, coords.y * CELL_WIDTH, 0.0),
                texture: asset_server.load("cell.png"),
                ..default()
            },
            state: CellState::Alive,
            neighbors: CellLivingNeighborsCount(0),
            coords: CellCoordinates(IVec2 {
                x: coords.x as i32,
                y: coords.y as i32,
            }),
        }
    }
}



/// Represents cell state 
#[derive(Component, Debug, Clone)]
pub enum CellState {
    Alive,
    Dead,
    Unsettled,
}

/// Amount of living neighbors
#[derive(Component, Debug)]
struct CellLivingNeighborsCount(u32);

/// Cell position relative to other cells
/// 
/// Used only as "ID" to identify certain cells 
#[derive(Component, Debug)]
pub struct CellCoordinates(pub IVec2);

/// Updates neighbor count for every cell
fn update_neighbors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&CellCoordinates, &CellState, &mut CellLivingNeighborsCount)>,
) {
    // Hashmap where key is "cell position" and value is amount of neighbors
    let mut new_cells: HashMap<IVec2, u32> = HashMap::new();
    // loop to generate neighbors hashmap
    for (coords, state, mut neighbors) in &mut query {
        neighbors.0 = 0;
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
                    let a = new_cells.entry(current_coord);
                    *a.or_insert(0) += 1;
                }
            }
        }
    }

    // loop to apply neighbors
    for (coords, _state, mut neighbors) in &mut query {
        match new_cells.get(&IVec2 {
            x: coords.0.x,
            y: coords.0.y,
        }) {
            Some(new_neighbors) => neighbors.0 = *new_neighbors,
            None => (), //?IDK <= impossible 
        }
        // remove cell from hashmap as it is completed and will not be used 
        new_cells.remove(&IVec2 {
            x: coords.0.x,
            y: coords.0.y,
        });
    }

    // loop to spawn new unsettled cells from hashmap leftovers
    // leftovers are unsettled cells which got their first living neighbor this frame  
    for (coords, neighbors) in new_cells {
        let mut cell = CellBundle::from_coords(
            Vec2 {
                x: coords.x as f32,
                y: coords.y as f32,
            },
            &asset_server,
        );
        cell.neighbors.0 = neighbors;
        cell.state = CellState::Unsettled;
        commands.spawn(cell);
    }
}

/// Uses neighbor count of each cell to update cell state
/// 
/// +-----------+-----------------+
/// 
/// | neighbors | change to state |
/// 
/// +-----------+-----------------+
/// 
/// |     0     |      Dead       |
/// 
/// |  1 or >3  |    Unsettled    |
/// 
/// |     2     |  Doest change   |
/// 
/// |     3     |      Alive      |
/// 
/// +-----------+-----------------+

fn update_marks(
    mut query: Query<(&mut CellState, &CellLivingNeighborsCount)>,
) {
    for (mut state, neighbors) in &mut query {
        let curr_state = state.clone();
        *state = match neighbors.0 {
            0 => CellState::Dead,
            2 => curr_state,
            3 => CellState::Alive,
            _ => CellState::Unsettled,
        }
    }
}


/// Deletes dead cell entities
fn clear_dead_cells(mut commands: Commands, query: Query<(Entity, &CellState)>) {
    for (cell, state) in &query {
        match state {
            CellState::Dead => commands.entity(cell).despawn(),
            _ => (),
        }
    }
}

/// Updates cell textures and hides dead or unsettled cells
fn update_texture(
    mut query: Query<(
        &mut Handle<Image>,
        &mut Visibility,
        &CellState,
        &CellLivingNeighborsCount,
    )>,
    asset_server: Res<AssetServer>,
) {
    //neighbor is used to check neighbors for debug
    for (mut sprite, mut visibility, state, _neighbors) in &mut query {
        match state {
            CellState::Alive => {
                *sprite = asset_server.load(format!("cell.png"));
                *visibility = Visibility::Visible;
            }
            CellState::Dead => {
            //    *sprite = asset_server.load(format!("n{}.png", neighbors.0));
                *visibility = Visibility::Hidden;
            }
            CellState::Unsettled => {
            //    *sprite = asset_server.load(format!("n{}.png", neighbors.0));
                *visibility = Visibility::Hidden;
            }
        }
    }
}

/// Stop frames
fn finish_frame(
    mut next_state: ResMut<NextState<crate::timer::AllowNextFrame>>,){
    next_state.set(crate::timer::AllowNextFrame::No);
}