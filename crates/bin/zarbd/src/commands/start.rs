use crate::file::{load_config_file, load_genesis_file};
use anyhow::Result;
use async_std::task;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{env, thread};
use structopt::StructOpt;
use zarb::config::Config;
use zarb::network::create_network_service;
use zarb::sync::create_sync_service;
use zarb::Service;

lazy_static! {
    static ref DEFAULT_WORKING_DIR: String =
        format!("{}/zarb", env::var("HOME").as_deref().unwrap_or("."));
}

/// The `generate-node-key` command
#[derive(Debug, StructOpt)]
#[structopt(name = "start", about = "run the node")]
pub struct StartCmd {
    #[structopt(short = "w", default_value = &DEFAULT_WORKING_DIR)]
    working_dir: String,
}

impl Default for StartCmd {
    fn default() -> Self {
        Self {
            working_dir: format! {"{:?}/zarb", dirs::home_dir().unwrap().to_str()},
        }
    }
}

impl StartCmd {
    /// Run the command
    pub fn execute(&self) -> Result<()> {
        let dir = self.working_dir.clone();
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        pretty_env_logger::init();

        //load the configuration file
        let config: Config = crate::file::load_config_file(dir.clone() + "/config.toml")?;

        //load the genesis file
        //let genesis: Genesis = file::load_genesis_file(dir.clone() + "/genesis.json").unwrap();

        let mut network = create_network_service(config.network)?;
        let sync = create_sync_service(config.sync, &mut network).unwrap();

        let network_task = task::spawn(async {
            network.start().await;
        });

        let sync_task = task::spawn(async {
            sync.start().await;
        });

        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_secs(1));
        }

        println!("Exiting...");

        let _handle = task::spawn(async {
            network_task.cancel().await;
            sync_task.cancel().await;
        });

        // futures::executor::block_on(handle);

        Ok(())
    }
}
