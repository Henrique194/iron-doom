use bevy::prelude::*;
use clap::Parser;
use response_file::process_response_files;
use std::{collections::VecDeque, env, path::PathBuf};

mod response_file;

#[derive(Default)]
pub struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandLine>();
    }
}

#[derive(Resource, Parser, Debug)]
#[command(version, next_line_help = true)]
pub struct CommandLine {
    /// Specify an IWAD file to use
    #[arg(long, value_name = "FILE")]
    pub iwad: Option<PathBuf>,

    /// Load extra command line arguments from the given response file
    #[arg(long, num_args = 1.., value_name = "FILES")]
    pub response: Vec<PathBuf>,
}

impl FromWorld for CommandLine {
    fn from_world(_world: &mut World) -> Self {
        let mut args: VecDeque<String> = env::args().collect();
        process_response_files(&mut args).unwrap();

        CommandLine::parse_from(args)
    }
}
