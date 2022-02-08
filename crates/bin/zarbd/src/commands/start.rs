use crate::file::{load_json_file, load_text_file, load_toml_file};
use anyhow::Result;
use async_std::task;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;
use zarb::config::Config;
use zarb::network::create_network_service;
use zarb::sync::create_sync_service;
use zarb::Service;
use zarb_types::crypto::bls::secret_key::BLSSecretKey;
use zarb_types::crypto::signer::Signer;

#[derive(Debug, StructOpt)]
#[structopt(name = "start", about = "run the node")]
pub struct StartCmd {
    #[structopt(short = "w", default_value = &super::DEFAULT_WORKING_DIR)]
    working_dir: String,
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

        let hex: String = load_text_file(dir.clone() + super::VALIDATOR_KEY_FILE_NAME)?;
        let validator_key = BLSSecretKey::from_string(&hex)?;
        let signer = Signer::new(Box::new(validator_key));

        //load the configuration file
        let config: Config = load_toml_file(dir.clone() + super::CONFIG_FILE_NAME)?;

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
