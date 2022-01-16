use super::behaviour;
use super::config::Config;
use super::event::Event;
use super::message::Message;
use super::transport;
use crate::error::Error;
use async_std::channel::{Receiver, Sender};
use async_std::stream;
use futures_util::stream::StreamExt;
use behaviour::{Behaviour, BehaviourEventOut};
use futures::select;
use libp2p::core::network::NetworkEvent;
pub use libp2p::gossipsub::{Topic, TopicHash};
use libp2p::identity;
use libp2p::Swarm;
use libp2p::swarm::SwarmEvent;
use log::{error, info, trace, warn};
use std::time::Duration;

pub struct Network {
    config: Config,
    swarm: Swarm<Behaviour>,
    message_receiver: Receiver<Message>,
    message_sender: Sender<Message>,
    event_receiver: Receiver<Event>,
    event_sender: Sender<Event>,
}

async fn emit_event(sender: &Sender<Event>, event: Event) {
    if sender.send(event).await.is_err() {
        error!("Failed to emit event: Network channel receiver has been dropped");
    }
}


impl Network {
    pub fn new(config: Config) -> Result<Self, Error> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_public = local_key.public();
        let local_peer_id = local_public.clone().to_peer_id();
        info!("Local node identity is: {}", local_peer_id.to_base58());

        let transport = transport::build_transport(&local_key);
        let behaviour = Behaviour::new(&local_key, &config);

        let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

        Swarm::listen_on(&mut swarm, config.listening_multiaddr.clone()).unwrap();

        for to_dial in &config.bootstrap_peers {
            match libp2p::Swarm::dial(&mut swarm, to_dial.clone()) {
                Ok(_) => info!("Dialed {:?}", to_dial),
                Err(e) => error!("Dial {:?} failed: {:?}", to_dial, e),
            }
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

    pub fn sender(&self) -> Sender<Message> {
        self.message_sender.clone()
    }

    pub fn register_topic(&mut self, topic_name: String) -> Receiver<Message> {
        let topic = Topic::new(topic_name.clone());
        self.swarm.behaviour_mut().subscribe(&topic);
        let (notifier_sender, notifier_receiver) = async_std::channel::unbounded();
        notifier_receiver
    }

    pub fn event_receiver(&self) -> Receiver<Event> {
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
                            emit_event(&self.event_sender, Event::PeerConnected(peer_id)).await;
                        }
                        SwarmEvent::Behaviour(BehaviourEventOut::PeerDisconnected(peer_id)) =>{
                            info!("Peer disconnected {:?}", peer_id);
                            emit_event(&self.event_sender, Event::PeerDisconnected(peer_id)).await;
                        }
                        SwarmEvent::Behaviour(BehaviourEventOut::GossipMessage {
                            source,
                            topic,
                            message,
                        }) => {
                            trace!("Got a Gossip message from {:?}: {:?}", source, message);

                        }
                        _ => {
                            continue;
                        }
                    }
                    None => { break; }
                },
                message = network_stream.next() => match message {
                    Some(mut entry) => {
                        // if let Some(topic_name) = entry.topic_name.clone() {
                        //     let topic = Topic::new(topic_name);
                        //     entry.from = Some(self.node_id.clone());
                        //     if let Err(e) = swarm_stream.get_mut().publish(&topic, entry) {
                        //         warn!("Failed to send gossipsub message: {:?}", e);
                        //     }
                        // }
                    }
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
