use std::time::{Duration, SystemTime};

use bevy::prelude::*;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(Speed(1))
            .insert_resource(Timer {
                last_frame: SystemTime::now(),
            })
            .init_state::<GamePause>()
            .add_systems(Update, keyboard_input)
            .add_systems(Update, check_timer.run_if(in_state(GamePause::Running)));
    }
}

/// Processes keyboard input: changes speed on arrow presses, pauses/unpauses the game on space press
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GamePause>>,
    mut next_state: ResMut<NextState<GamePause>>,
    mut speed: ResMut<Speed>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            GamePause::Paused => next_state.set(GamePause::Running),
            GamePause::Running => next_state.set(GamePause::Paused),
        }
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        speed.0 = num::clamp(speed.0 - 1, 1, 60);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        speed.0 = num::clamp(speed.0 + 1, 1, 60);
    }
}

/// Time speed
#[derive(Resource)]
struct Speed(u32);

/// Pause state
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GamePause {
    #[default]
    Paused,
    Running,
}

/// Allows or disallows new frames from being played  
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllowNextFrame {
    #[default]
    No,
    Yes,
}

/// Timer used to control game speed
#[derive(Resource)]
struct Timer {
    /// Last time frame was generated
    last_frame: SystemTime,
}

/// Updates timers and allows frame if time passed
fn check_timer(
    mut timer: ResMut<Timer>,
    mut allow_frame: ResMut<NextState<AllowNextFrame>>,
    speed: Res<Speed>,
) {
    let passed_time = SystemTime::now()
        .duration_since(timer.last_frame)
        .unwrap_or(Duration::from_secs(0));
    if speed.0 == 0 {
        return; //prevent division by 0 
    }
    if passed_time > Duration::from_secs_f32(1.0 / speed.0 as f32) {
        allow_frame.set(AllowNextFrame::Yes);
        timer.last_frame = SystemTime::now();
    }
}
