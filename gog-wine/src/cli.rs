use clap::{Parser, Subcommand};
use crate::commands;

#[derive(Parser)]
#[command(
    name = "gog-wine",
    version,
    about = "Install windows based drm free games on mac"
)]
pub struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        game_name: String,
        directory: Option<String>
    },
    Install,
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Init {
                game_name,
                directory
            } => commands::init::run(game_name, directory),

            Commands::Install => commands::install::run()
        }
    }
}
