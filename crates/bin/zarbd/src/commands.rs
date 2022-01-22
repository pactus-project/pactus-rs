use dirs::home_dir;
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;

pub mod init;
pub mod start;

use crate::commands::init::InitCmd;
use crate::commands::start::StartCmd;

pub trait ZarbdCommand {
    /// Returns the result of the command execution.
    fn execute(self) -> Result<()>;
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "init")]
    Init(InitCmd),

    #[structopt(name = "start")]
    Start(StartCmd),
}

impl Command {
    /// Wrapper around `StructOpt::from_args` method.
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }

    pub fn get_homedir() -> Option<PathBuf> {
        let path = home_dir();
        return path;
    }
}

impl ZarbdCommand for Command {
    fn execute(self) -> Result<()> {
        match self {
            Self::Init(command) => command.execute(),
            Self::Start(command) => command.execute(),
        }
    }
}
