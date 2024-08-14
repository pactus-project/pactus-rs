pub mod config;
pub mod firewall;
pub mod handler;
pub mod bundle;
pub mod service;

use pactus_types::crypto::signer::Signer;
use self::service::PactusSync;
use crate::error::Result;
use crate::network::NetworkService;

pub trait SyncService: crate::Service {}

pub fn create_sync_service(
    config: config::Config,
    signer: Signer,
    network: &mut dyn NetworkService,
) -> Result<impl SyncService> {
    PactusSync::new(config, signer, network)
}
