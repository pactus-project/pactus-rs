pub mod config;
pub mod firewall;
pub mod message;
pub mod service;

use self::service::ZarbSync;
use crate::error::Result;
use crate::network::NetworkService;

pub trait SyncService: crate::Service {}

pub fn create_sync_service(
    config: config::Config,
    network: &mut dyn NetworkService,
) -> Result<impl SyncService> {
    Ok(ZarbSync::new(config, network)?)
}
