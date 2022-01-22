use crate::file::{save_config_file, save_genesis_file, save_json_file};
use anyhow::Result;
use std::net::SocketAddr;
use std::path::PathBuf;
use structopt::StructOpt;
use zarb::config::Config;

pub const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "init",
    about = "init command Initialized the config file in root dir"
)]

pub struct InitCmd {
    #[structopt(short = "o", default_value = "")]
    pub output_dir: String,
    #[structopt(short, long, default_value = "127.0.0.1:5333")]
    pub peer_address: String,
    #[structopt(long, short = "l", default_value = "127.0.0.1:6333")]
    pub listen_address: SocketAddr,
}

impl Default for InitCmd {
    fn default() -> Self {
        Self {
            output_dir: dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/zarb",
            peer_address: String::from("127.0.0.1:5333"),
            listen_address: ("127.0.0.1:6333")
                .parse()
                .expect("Unable to parse socket address"),
        }
    }
}

impl InitCmd {
    /// Run the command
    pub fn execute(&self) -> Result<()> {
        let mut config_path = PathBuf::new();
        let mut path = self.output_dir.clone();
        let cp = InitCmd::default().output_dir;
        if path.is_empty() {
            path = cp;
        }
        config_path = config_path.join(&path).join(CONFIG_FILE_NAME);

        let node_config = Config::default();

        save_config_file(&node_config, &config_path).expect("config file already exist");
        Ok(())
    }
}
