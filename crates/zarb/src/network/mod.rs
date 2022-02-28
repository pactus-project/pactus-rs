mod behaviour;
mod swarm;
mod transport;

pub mod config;
pub mod service;

pub use crate::error::Result;
use async_std::channel::{Receiver, Sender};
use libp2p::PeerId;
use service::ZarbNetwork;

#[derive(Debug)]
pub enum NetworkEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    MessageReceived { source: PeerId, data: Vec<u8> },
}

#[derive(Debug)]
pub enum NetworkMessage {
    GeneralMessage { data: Vec<u8> },
    ConsensusMessage { data: Vec<u8> },
    StreamMessage { target: PeerId, data: Vec<u8> },
}
pub trait NetworkService: crate::Service {
    fn self_id(&self) -> PeerId;
    fn message_sender(&self) -> Sender<NetworkMessage>;
    fn event_receiver(&self) -> Receiver<NetworkEvent>;
}

pub fn create_network_service(config: config::Config) -> Result<impl NetworkService> {
    ZarbNetwork::new(config)
}
