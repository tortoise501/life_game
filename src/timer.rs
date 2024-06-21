use std::time::{Duration, SystemTime};

use bevy::prelude::*;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<ButtonInput<KeyCode>>()
        //     // .insert_resource(ButtonInput::)
        //     .init_state::<MyPausedState>()
        //     .add_systems(Startup, spawn_cells)

        app.init_resource::<ButtonInput<KeyCode>>()
        .insert_resource(Speed(1))
        .insert_resource(Timer{ last_frame: SystemTime::now() })
        .init_state::<GamePause>()
        .add_systems(Update, keyboard_input)
        .add_systems(Update,check_timer.run_if(in_state(GamePause::Running)));
    }
}



fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GamePause>>,
    mut next_state: ResMut<NextState<GamePause>>,
    mut speed: ResMut<Speed>
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            GamePause::Paused => next_state.set(GamePause::Running),
            GamePause::Running => next_state.set(GamePause::Paused),
        }
    }
    if keys.just_pressed(KeyCode::ArrowLeft){
        // info!("left     {:?}",speed.0);
        speed.0 = num::clamp(speed.0 - 1,1,60);
    }
    if keys.just_pressed(KeyCode::ArrowRight){
        // info!("right     {:?}",speed.0);
        speed.0 = num::clamp(speed.0 + 1,1,60);
    }
}

#[derive(Resource)]
struct Speed(u32);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GamePause {
    #[default]
    Paused,
    Running,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllowNextFrame {
    #[default]
    No,
    Yes,
}

#[derive(Resource)]
struct Timer {
    last_frame: SystemTime,
}

fn check_timer(mut timer: ResMut<Timer>,mut allow_frame: ResMut<NextState<AllowNextFrame>>, speed:Res<Speed>){
    let passed_time = SystemTime::now().duration_since(timer.last_frame).unwrap_or(Duration::from_secs(0));
    if speed.0 == 0{
        return;//?Do Something
    }
    if passed_time > Duration::from_secs_f32(1.0 / speed.0 as f32){
        allow_frame.set(AllowNextFrame::Yes);
        timer.last_frame = SystemTime::now();
    }
}