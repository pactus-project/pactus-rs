use crate::file::{load_config_file, load_genesis_file};
use anyhow::Result;
use async_std::task;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;
use zarb::config::Config;
use zarb::network::network::Network;

/// The `generate-node-key` command
#[derive(Debug, StructOpt)]
#[structopt(name = "start", about = "run the node")]
pub struct StartCmd {
    #[structopt(short = "o", default_value = "")]
    output_dir: String,
}

impl Default for StartCmd {
    fn default() -> Self {
        Self {
            output_dir: format! {"{:?}/zarb", dirs::home_dir().unwrap().to_str()},
        }
    }
}

impl StartCmd {
    /// Run the command
    pub fn execute(&self) -> Result<()> {
        let dir = self.output_dir.clone();
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        pretty_env_logger::init();

        //load the configuration file
        let config: Config = crate::file::load_config_file(dir.clone() + "/config.toml").unwrap();

        //load the genesis file
        //let genesis: Genesis = file::load_genesis_file(dir.clone() + "/genesis.json").unwrap();

        let network = Network::new(config.network).ok().unwrap();

        let network_task = task::spawn(async {
            network.run().await;
        });

        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_secs(1));
        }

        println!("Exiting...");

        let _handle = task::spawn(async {
            network_task.cancel().await;
        });

        // futures::executor::block_on(handle);

        Ok(())
    }
}


