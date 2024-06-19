use bevy::prelude::*;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<ButtonInput<KeyCode>>()
        //     // .insert_resource(ButtonInput::)
        //     .init_state::<MyPausedState>()
        //     .add_systems(Startup, spawn_cells)

        app.init_resource::<ButtonInput<KeyCode>>()
        .init_state::<GamePause>();
    }
}

// fn spawn_cells(mut commands: Commands, asset_server: Res<AssetServer>) {
//     for x in -1..2 {
//         let mut cell = CellBundle::from_coords(
//             Vec2 {
//                 x: x as f32,
//                 y: 0.0,
//             },
//             &asset_server,
//         );
//         cell.state = CellState::Alive;
//         commands.spawn(cell);
//     }
//     for x in 0..3 {
//         let mut cell = CellBundle::from_coords(
//             Vec2 {
//                 x: x as f32,
//                 y: -1.0,
//             },
//             &asset_server,
//         );
//         cell.state = CellState::Alive;
//         commands.spawn(cell);
//     }
// }

// fn check_cells(query: Query<(&CellCoordinates, &CellState, &CellLivingNeighborsCount)>) {
//     info!("checking");
//     for (coords, state, neighbors) in &query {
//         info!(
//             "coords = {:?}    state = {:?}    neighbors = {:?}",
//             coords, state, neighbors
//         );
//     }
// }

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GamePause>>,
    mut next_state: ResMut<NextState<GamePause>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            GamePause::Paused => next_state.set(GamePause::Running),
            GamePause::Running => next_state.set(GamePause::Paused),
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GamePause {
    #[default]
    Paused,
    Running,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AllowNextFrame {
    #[default]
    No,
    Yes,
}
