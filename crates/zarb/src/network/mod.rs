mod behaviour;
mod swarm;
mod transport;

pub mod config;
pub mod service;

pub use crate::error::Result;
use async_std::channel::{Receiver, Sender};
use async_trait::async_trait;
use libp2p::{gossipsub::TopicHash, PeerId};
use service::ZarbNetwork;

#[derive(Debug)]
pub enum NetworkEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    MessageReceived {
        source: PeerId,
        topic: TopicHash,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub struct NetworkMessage {
    pub topic_name: String,
    pub data: Vec<u8>,
}

pub trait NetworkService: crate::Service {
    fn register_topic(&mut self, topic_name: String) -> Result<bool>;
    fn message_sender(&self) -> Sender<NetworkMessage>;
    fn event_receiver(&self) -> Receiver<NetworkEvent>;
}

pub fn create_network_service(config: config::Config) -> Result<impl NetworkService> {
    Ok(ZarbNetwork::new(config)?)
}
