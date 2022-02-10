pub mod config;
pub mod firewall;
pub mod handler;
pub mod message;
pub mod service;

use self::service::ZarbSync;
use crate::error::Result;
use crate::network::NetworkService;
use zarb_types::crypto::bls::signer::BLSSigner;

pub trait SyncService: crate::Service {}

pub fn create_sync_service(
    config: config::Config,
    signer: BLSSigner,
    network: &mut dyn NetworkService,
) -> Result<impl SyncService> {
    Ok(ZarbSync::new(config, signer, network)?)
}
