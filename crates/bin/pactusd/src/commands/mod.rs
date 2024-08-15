pub mod init;
pub mod start;

use anyhow::Result;
use lazy_static::lazy_static;
use structopt::StructOpt;
use std::env;

pub const CONFIG_FILE_NAME: &str = "config.toml";
pub const VALIDATOR_KEY_FILE_NAME: &str = "validator_key";

lazy_static! {
    static ref DEFAULT_WORKING_DIR: String =
        format!("{}/pactus", env::var("HOME").as_deref().unwrap_or("."));
}

use crate::commands::init::InitCmd;
use crate::commands::start::StartCmd;

pub trait PactusDaemonCommand {
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
}

impl PactusDaemonCommand for Command {
    fn execute(self) -> Result<()> {
        match self {
            Self::Init(command) => command.execute(),
            Self::Start(command) => command.execute(),
        }
    }
}
