use anyhow::Error;
use bevy::prelude::*;
use commands::{ExitErrorCommand, ExitSuccessCommand};

pub mod macros {
    pub use sys_fail::sys_fail;
}

mod commands;

#[derive(Default)]
pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Exit>();
    }
}

#[derive(Event, Debug, Default)]
pub enum Exit {
    #[default]
    Success,
    Error(Error),
}

pub trait ExitCommands {
    fn exit(&mut self);

    fn exit_error(&mut self, error: Error);
}

impl ExitCommands for Commands<'_, '_> {
    fn exit(&mut self) {
        self.queue(ExitSuccessCommand);
    }

    fn exit_error(&mut self, error: Error) {
        self.queue(ExitErrorCommand(error));
    }
}
