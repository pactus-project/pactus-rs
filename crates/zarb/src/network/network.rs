use super::behaviour;
use super::config::Config;
use super::transport;
use crate::error::{Error, Result};
use async_std::channel::{Receiver, Sender};
use async_std::stream;
use behaviour::{Behaviour, BehaviourEventOut};
use futures::select;
use futures_util::stream::StreamExt;
use libp2p::gossipsub::IdentTopic;
pub use libp2p::gossipsub::{Topic, TopicHash};
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use libp2p::{identity, PeerId};
use log::{error, info, trace, warn};
use std::fmt::Debug;
use std::time::Duration;

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
pub struct PubsubMessage {
    topic_name: String,
    data: Vec<u8>,
}

pub struct Network {
    config: Config,
    swarm: Swarm<Behaviour>,
    message_receiver: Receiver<PubsubMessage>,
    message_sender: Sender<PubsubMessage>,
    event_receiver: Receiver<NetworkEvent>,
    event_sender: Sender<NetworkEvent>,
}

async fn emit_event(sender: &Sender<NetworkEvent>, event: NetworkEvent) {
    if sender.send(event).await.is_err() {
        error!("Failed to emit event: Network channel receiver has been dropped");
    }
}

impl Network {
    pub fn new(config: Config) -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_public = local_key.public();
        let local_peer_id = local_public.clone().to_peer_id();
        info!("Local node identity is: {}", local_peer_id.to_base58());

        let transport = transport::build_transport(&local_key);
        let behaviour = Behaviour::new(&local_key, &config);

        let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

        Swarm::listen_on(&mut swarm, config.listening_addr.clone()).unwrap();

        for to_dial in &config.bootstrap_peers {
            libp2p::Swarm::dial(&mut swarm, to_dial.clone()).map_err(|err| {
                Error::NetworkError(format!("Dial {:?} failed: {:?}", to_dial, err))
            })?;
        }

        // Bootstrap with Kademlia
        if let Err(e) = swarm.behaviour_mut().bootstrap() {
            warn!("Failed to bootstrap with Kademlia: {}", e);
        }

        let (message_sender, message_receiver) = async_std::channel::unbounded();
        let (event_sender, event_receiver) = async_std::channel::unbounded();

        Ok(Network {
            config,
            swarm,
            message_sender,
            message_receiver,
            event_sender,
            event_receiver,
        })
    }

    pub fn close(&self) {}

    pub fn name(&self) -> String {
        self.config.network_name.clone()
    }

    pub fn sender(&self) -> Sender<PubsubMessage> {
        self.message_sender.clone()
    }

    pub fn register_topic(&mut self, topic_name: String) -> Result<bool> {
        let topic = Topic::new(topic_name.clone());
        self.swarm
            .behaviour_mut()
            .subscribe(&topic)
            .map_err(|err| Error::NetworkError(format!("{:?}", err)))
    }

    pub fn event_receiver(&self) -> Receiver<NetworkEvent> {
        self.event_receiver.clone()
    }

    pub async fn run(self) {
        let mut swarm_stream = self.swarm.fuse();
        let mut network_stream = self.message_receiver.fuse();
        let mut interval = stream::interval(Duration::from_secs(10)).fuse();

        loop {
            select! {
                swarm_event = swarm_stream.next() => match swarm_event {
                    Some(event) => match event {
                        SwarmEvent::Behaviour(BehaviourEventOut::PeerConnected(peer_id)) =>{
                            info!("Peer dialed {:?}", peer_id);
                            emit_event(&self.event_sender, NetworkEvent::PeerConnected(peer_id)).await;
                        }
                        SwarmEvent::Behaviour(BehaviourEventOut::PeerDisconnected(peer_id)) =>{
                            info!("Peer disconnected {:?}", peer_id);
                            emit_event(&self.event_sender, NetworkEvent::PeerDisconnected(peer_id)).await;
                        }
                        SwarmEvent::Behaviour(BehaviourEventOut::MessageReceived {
                            source,
                            topic,
                            data,
                        }) => {
                            trace!("Got a Gossip message from {:?}: {:?}", source, data);
                            emit_event(&self.event_sender, NetworkEvent::MessageReceived {
                                source, topic, data
                            }).await;
                        }
                        _ => {
                            continue;
                        }
                    }
                    None => { break; }
                },
                message = network_stream.next() => match message {
                    Some(msg) => {
                        let topic = Topic::new(msg.topic_name);
                        if let Err(e) = swarm_stream.get_mut().behaviour_mut().publish(topic, msg.data) {
                            warn!("Failed to send gossipsub message: {:?}", e);
                        }
                    },
                    None => { break; }
                },
                interval_event = interval.next() => if interval_event.is_some() {
                    trace!("Peers connected: {}", swarm_stream.get_mut().behaviour_mut().peers().len());
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "./network_test.rs"]
mod network_test;
