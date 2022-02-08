use crate::file::{save_text_file, save_toml_file};
use anyhow::Result;
use std::net::SocketAddr;
use std::path::PathBuf;
use structopt::StructOpt;
use zarb::config::Config;
use zarb_types::crypto::bls::secret_key::BLSSecretKey;

#[derive(Debug, StructOpt)]
#[structopt(name = "init", about = "Initializing the working directory")]

pub struct InitCmd {
    #[structopt(short = "w", default_value = &super::DEFAULT_WORKING_DIR)]
    pub working_dir: String,
    #[structopt(short, long, default_value = "127.0.0.1:5333")]
    pub peer_address: String,
    #[structopt(long, short = "l", default_value = "127.0.0.1:6333")]
    pub listen_address: SocketAddr,
}

impl InitCmd {
    /// Run the command
    pub fn execute(&self) -> Result<()> {
        let dir = self.working_dir.clone();
        let sec = BLSSecretKey::random();
        let node_config = Config::default();

        save_toml_file(&node_config, dir.clone() + super::CONFIG_FILE_NAME)?;
        save_text_file(
            &sec.to_string(),
            dir.clone() + super::VALIDATOR_KEY_FILE_NAME,
        )?;
        Ok(())
    }
}
