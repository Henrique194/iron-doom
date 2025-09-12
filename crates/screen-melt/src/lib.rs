use bevy::prelude::*;
use events::InitMeltScreen;
use game_state::{conditions::in_screen_melt_state, PlayingState};
use schedules::{InitEndScreen, InitStartScreen};

mod components;
mod events;
mod resources;
mod schedules;
mod systems;

/// Plugin responsible for the screen melt effect seen when Doom changes scene,
/// for example, when  starting or exiting a level. The screen appears to "melt"
/// away to the new screen. In order to work properly, it is necessary that all
/// screen buffer updates take place in [`PostUpdate`] schedule.
#[derive(Default)]
pub struct ScreenMeltPlugin;

impl Plugin for ScreenMeltPlugin {
    fn build(&self, app: &mut App) {
        schedules::setup(app);

        app.add_event::<InitMeltScreen>()
            .add_systems(
                Update,
                systems::check_state_transition
                    .run_if(on_event::<StateTransitionEvent<PlayingState>>),
            )
            .add_systems(
                InitStartScreen,
                systems::init_start_screen.run_if(on_event::<InitMeltScreen>),
            )
            .add_systems(
                InitEndScreen,
                systems::init_end_screen.run_if(on_event::<InitMeltScreen>),
            )
            .add_systems(OnEnter(PlayingState::ScreenMelt), systems::init_columns)
            .add_systems(
                FixedUpdate,
                systems::move_columns
                    .after(systems::init_columns)
                    .run_if(in_screen_melt_state()),
            );
    }
}
