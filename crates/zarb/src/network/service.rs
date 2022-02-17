use super::config::Config;
use super::transport;
use super::{behaviour, NetworkService};
use super::{NetworkEvent, NetworkMessage};
use crate::error::{Error, Result};
use async_std::channel::{Receiver, Sender};
use async_std::stream;
use async_trait::async_trait;
use behaviour::{Behaviour, BehaviourEventOut};
use futures::select;
use futures_util::stream::StreamExt;
use libp2p::gossipsub::IdentTopic;
pub use libp2p::gossipsub::{Topic, TopicHash};
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use libp2p::{identity, PeerId};
use log::{debug, error, info, warn};
use std::time::Duration;

pub(super) struct ZarbNetwork {
    config: Config,
    swarm: Swarm<Behaviour>,
    message_receiver: Receiver<NetworkMessage>,
    message_sender: Sender<NetworkMessage>,
    event_receiver: Receiver<NetworkEvent>,
    event_sender: Sender<NetworkEvent>,
}

async fn emit_event(sender: &Sender<NetworkEvent>, event: NetworkEvent) {
    if sender.send(event).await.is_err() {
        error!("network channel receiver has been dropped");
    }
}

impl NetworkService for ZarbNetwork {
    fn self_id(&self) -> PeerId {
        self.swarm.local_peer_id().clone()
    }
    fn message_sender(&self) -> Sender<NetworkMessage> {
        self.message_sender.clone()
    }

    fn event_receiver(&self) -> Receiver<NetworkEvent> {
        self.event_receiver.clone()
    }
}

impl ZarbNetwork {
    pub fn new(config: Config) -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_public = local_key.public();
        let local_peer_id = local_public.clone().to_peer_id();
        info!("local node identity is: {}", local_peer_id.to_base58());

        let transport = transport::build_transport(&local_key);
        let behaviour = Behaviour::new(&local_key, &config);

        let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

        Swarm::listen_on(&mut swarm, config.listening_addr.clone()).unwrap();

        for to_dial in &config.bootstrap_peers {
            libp2p::Swarm::dial(&mut swarm, to_dial.clone()).map_err(|err| {
                Error::NetworkError(format!("dial {:?} failed: {:?}", to_dial, err))
            })?;
        }

        // Bootstrap with Kademlia
        if let Err(e) = swarm.behaviour_mut().bootstrap() {
            warn!("failed to bootstrap with Kademlia: {}", e);
        }

        let (message_sender, message_receiver) = async_std::channel::unbounded();
        let (event_sender, event_receiver) = async_std::channel::unbounded();

        Ok(Self {
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

    fn get_topic(&self, topic_name: &str) -> IdentTopic {
        let topic_name = format!("/{}/topic/{}/v1", self.config.network_name, topic_name);
        Topic::new(topic_name)
    }

    fn get_general_topic(&self) -> IdentTopic {
        self.get_topic("general")
    }

    fn get_consensus_topic(&self) -> IdentTopic {
        self.get_topic("consensus")
    }

    // fn join_general_topic(&mut self) -> Result<bool> {
    //     self.join_topic(&self.get_general_topic())
    // }

    // fn join_consensus_topic(&mut self) -> Result<bool> {
    //     self.join_topic(&self.get_consensus_topic())
    // }

    // pub fn join_topic(&mut self, topic: &IdentTopic) -> Result<bool> {
    //     self.swarm
    //         .behaviour_mut()
    //         .subscribe(&topic)
    //         .map_err(|err| Error::NetworkError(format!("{:?}", err)))
    // }
}


#[async_trait]
impl crate::Service for ZarbNetwork {
    async fn start(self) {
        let general_topic = self.get_general_topic();
        let consensus_topic = self.get_consensus_topic();
        let mut swarm_stream = self.swarm.fuse();
        let mut network_stream = self.message_receiver.fuse();
        let mut interval = stream::interval(Duration::from_secs(10)).fuse();
        let mut has_joined_consensus_topic = false;

        // Join general topic by starting the service,
        // We will join consensus topic later (after syncing)
        swarm_stream
            .get_mut()
            .behaviour_mut()
            .subscribe(&general_topic)
            .expect("Unable to join general topic");

        loop {
            select! {
                swarm_event = swarm_stream.next() => match swarm_event {
                    Some(event) => match event {
                        SwarmEvent::Behaviour(BehaviourEventOut::PeerConnected(peer_id)) => {
                            info!("peer dialed {:?}", peer_id);
                            emit_event(&self.event_sender, NetworkEvent::PeerConnected(peer_id)).await;
                        }
                        SwarmEvent::Behaviour(BehaviourEventOut::PeerDisconnected(peer_id)) => {
                            info!("peer disconnected {:?}", peer_id);
                            emit_event(&self.event_sender, NetworkEvent::PeerDisconnected(peer_id)).await;
                        }
                        SwarmEvent::Behaviour(BehaviourEventOut::MessageReceived {
                            source,
                            data,
                        }) => {
                            debug!("got a Gossip message from {:?}", source);
                            emit_event(&self.event_sender, NetworkEvent::MessageReceived {
                                source,  data
                            }).await;
                        }
                        _ => {
                            continue;
                        }
                    }
                    None => { break; }
                },
                message = network_stream.next() => match message {
                    Some(msg) => match msg {
                        NetworkMessage::GeneralMessage{data} =>{
                            if let Err(e) = swarm_stream.get_mut().behaviour_mut().publish(general_topic.clone(), data) {
                                warn!("failed to publish message: {:?}", e);
                            }
                        }
                        NetworkMessage::ConsensusMessage{data} =>{
                            if !has_joined_consensus_topic {
                                swarm_stream.get_mut().behaviour_mut().subscribe(&consensus_topic).expect("Unable to join consensus topic");
                                has_joined_consensus_topic = true;
                            }
                            if let Err(e) = swarm_stream.get_mut().behaviour_mut().publish(consensus_topic.clone(), data) {
                                warn!("failed to publish message: {:?}", e);
                            }
                        }
                        NetworkMessage::StreamMessage{target, data} =>{
                        }
                    },
                    None => { break; }
                },
                interval_event = interval.next() => if interval_event.is_some() {
                    debug!("connected peers: {}", swarm_stream.get_mut().behaviour_mut().peers().len());
                }
            }
        }
    }
}


#[cfg(test)]
#[path = "./service_test.rs"]
mod service_test;
