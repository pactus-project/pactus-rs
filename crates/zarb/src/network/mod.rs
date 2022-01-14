pub mod behaviour;
pub mod config;
pub mod service;
pub mod transport;
///pub mod discovery;

use crate::error::Result;
use async_trait::async_trait;
use libp2p::PeerId;

#[async_trait]
pub trait Network {
    fn start(&self) -> Result<()>;
    fn stop(&self);
    fn close_connection(&self, pid: PeerId);
    fn self_id(&self) -> PeerId;
}
