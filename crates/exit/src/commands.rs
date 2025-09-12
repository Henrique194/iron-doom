use crate::Exit;
use anyhow::Error;
use bevy::prelude::{Command, NextState, StateTransition, World};
use game_state::GameState;

pub struct ExitSuccessCommand;

impl Command for ExitSuccessCommand {
    fn apply(self, world: &mut World) {
        world.send_event(Exit::Success);

        let mut game_state = world.resource_mut::<NextState<GameState>>();
        game_state.set(GameState::Exiting);
        world.run_schedule(StateTransition);
    }
}

pub struct ExitErrorCommand(pub Error);

impl Command for ExitErrorCommand {
    fn apply(self, world: &mut World) {
        let error = self.0;
        world.send_event(Exit::Error(error));

        let mut game_state = world.resource_mut::<NextState<GameState>>();
        game_state.set(GameState::Exiting);
        world.run_schedule(StateTransition);
    }
}
