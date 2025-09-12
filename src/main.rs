// On Windows, detach console when compiling in release mode.
// This attribute is ignored on non-Windows targets.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{log::LogPlugin, prelude::*};
use crates_plugins::CratesPlugins;
use game_state::{conditions::in_setup_state, GameState};

mod crates_plugins;

pub const TIC_RATE: f64 = 35.0;

fn main() -> AppExit {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(TIC_RATE))
        .add_plugins(CratesPlugins.set(LogPlugin {
            #[cfg(debug_assertions)]
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_systems(Update, init_game.run_if(in_setup_state()))
        .run()
}

fn init_game(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
