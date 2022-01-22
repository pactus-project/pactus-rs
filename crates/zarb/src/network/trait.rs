use async_std::channel::{Receiver, Sender};
pub use libp2p::gossipsub::{Topic, TopicHash};
use libp2p::{PeerId};
use crate::error::Result;
use std::fmt::Debug;

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


pub trait Network {
    fn register_topic(&mut self, topic_name: String) -> Result<bool>;
    fn message_sender(&self) -> Sender<NetworkMessage>;
    fn event_receiver(&self) -> Receiver<NetworkEvent>;
}