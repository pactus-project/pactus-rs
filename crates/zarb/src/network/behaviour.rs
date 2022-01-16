use super::config::Config;
use super::swarm_api::{SwarmApi, SwarmEvent};
use libp2p::{
    core::{identity::Keypair, PeerId},
    gossipsub::{
        error::{PublishError, SubscriptionError},
        Gossipsub, GossipsubConfigBuilder, GossipsubEvent, GossipsubMessage, IdentTopic,
        MessageAuthenticity, MessageId, TopicHash,
    },
    identify::{Identify, IdentifyConfig, IdentifyEvent},
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryId},
    mdns::{Mdns, MdnsEvent},
    multiaddr::Protocol,
    ping::{Ping, PingEvent, PingFailure, PingSuccess},
    swarm::{
        toggle::Toggle, NetworkBehaviour, NetworkBehaviourAction, NetworkBehaviourEventProcess,
        PollParameters,
    },
    NetworkBehaviour,
};
use log::{debug, trace, warn};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{task::Context, task::Poll};

#[derive(NetworkBehaviour)]
#[behaviour(
    out_event = "BehaviourEventOut",
    poll_method = "poll",
    event_process = true
)]
pub struct Behaviour {
    swarm_api: SwarmApi,
    gossipsub: Gossipsub,
    mdns: Toggle<Mdns>,
    ping: Ping,
    identify: Identify,
    kademlia: Toggle<Kademlia<MemoryStore>>,
    #[behaviour(ignore)]
    peers: HashSet<PeerId>,
    #[behaviour(ignore)]
    events: Vec<BehaviourEventOut>,
}

#[derive(Debug)]
pub enum BehaviourEventOut {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    GossipMessage {
        source: PeerId,
        topic: TopicHash,
        message: Vec<u8>,
    },
}

impl Behaviour {
    fn poll(
        &mut self,
        _: &mut Context,
        _: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self as NetworkBehaviour>::OutEvent,
            <Self as NetworkBehaviour>::ProtocolsHandler,
        >,
    > {
        if !self.events.is_empty() {
            return Poll::Ready(NetworkBehaviourAction::GenerateEvent(self.events.remove(0)));
        }
        Poll::Pending
    }

    pub fn new(local_key: &Keypair, config: &Config) -> Self {
        // To content-address message, we can take the hash of message and use it as an ID.
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };

        let local_peer_id = local_key.public().to_peer_id();
        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
            .build();

        // Kademlia config
        let store = MemoryStore::new(local_peer_id.to_owned());
        let mut kad_config = KademliaConfig::default();
        let network = format!("/zarb/kad/{}/kad/1.0.0", config.network_name);
        kad_config.set_protocol_name(network.as_bytes().to_vec());
        let kademlia_opt = if config.kademlia {
            let mut kademlia = Kademlia::with_config(local_peer_id.to_owned(), store, kad_config);
            for multiaddr in config.bootstrap_peers.iter() {
                let mut addr = multiaddr.to_owned();
                if let Some(Protocol::P2p(mh)) = addr.pop() {
                    let peer_id = PeerId::from_multihash(mh).unwrap();
                    kademlia.add_address(&peer_id, addr);
                } else {
                    warn!("Could not add addr {} to Kademlia DHT", multiaddr)
                }
            }
            if let Err(e) = kademlia.bootstrap() {
                warn!("Kademlia bootstrap failed: {}", e);
            }
            Some(kademlia)
        } else {
            None
        };

        let mdns_opt = if config.mdns {
            Some(async_std::task::block_on(async {
                Mdns::new(Default::default())
                    .await
                    .expect("Could not start mDNS")
            }))
        } else {
            None
        };

        let identify = Identify::new(IdentifyConfig::new("zarb/0.1.0".into(), local_key.public()));

        let mut gs_config_builder = GossipsubConfigBuilder::default();
        // gs_config_builder.message_id_fn(|msg: &GossipsubMessage| {
        //     let s = blake2b_256(&msg.data);
        //     MessageId::from(s)
        // });
        let gossipsub_config = gs_config_builder.build().unwrap();
        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .unwrap();

        let swarm_api = SwarmApi::new();
        Behaviour {
            swarm_api: swarm_api,
            gossipsub,
            mdns: mdns_opt.into(),
            ping: Ping::default(),
            identify,
            kademlia: kademlia_opt.into(),
            events: vec![],
            peers: Default::default(),
        }
    }

    /// Bootstrap Kademlia network
    pub fn bootstrap(&mut self) -> Result<QueryId, String> {
        if let Some(active_kad) = self.kademlia.as_mut() {
            active_kad.bootstrap().map_err(|e| e.to_string())
        } else {
            Err("Kademlia is not activated".to_string())
        }
    }

    /// Publish data over the gossip network.
    pub fn publish(
        &mut self,
        topic: IdentTopic,
        data: impl Into<Vec<u8>>,
    ) -> Result<MessageId, PublishError> {
        self.gossipsub.publish(topic, data)
    }

    /// Subscribe to a gossip topic.
    pub fn subscribe(&mut self, topic: &IdentTopic) -> Result<bool, SubscriptionError> {
        self.gossipsub.subscribe(topic)
    }

    /// Adds peer to the peer set.
    pub fn add_peer(&mut self, peer_id: PeerId) {
        self.peers.insert(peer_id.clone());
    }

    /// Adds peer to the peer set.
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peers.remove(peer_id);
    }

    /// Adds peer to the peer set.
    pub fn peers(&self) -> &HashSet<PeerId> {
        &self.peers
    }
}

impl NetworkBehaviourEventProcess<IdentifyEvent> for Behaviour {
    fn inject_event(&mut self, event: IdentifyEvent) {
        match event {
            IdentifyEvent::Received { peer_id, info } => {
                trace!("Identified Peer {}", peer_id);
                trace!("protocol_version {}", info.protocol_version);
                trace!("agent_version {}", info.agent_version);
                trace!("listening_ addresses {:?}", info.listen_addrs);
                trace!("observed_address {}", info.observed_addr);
                trace!("protocols {:?}", info.protocols);
            }
            IdentifyEvent::Sent { .. } => (),
            IdentifyEvent::Pushed { .. } => (),
            IdentifyEvent::Error { .. } => (),
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, addr) in list {
                    trace!("mdns: Discovered peer {}", peer.to_base58());
                    self.add_peer(peer.clone());
                    self.kademlia.as_mut().unwrap().add_address(&peer, addr);
                }
            }
            MdnsEvent::Expired(list) => {
                if self.mdns.is_enabled() {
                    for (peer, _) in list {
                        if !self.mdns.as_ref().unwrap().has_node(&peer) {
                            self.remove_peer(&peer);
                        }
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::RoutingUpdated { peer, .. } => {
                self.add_peer(peer);
            }
            event => {
                trace!("kad: {:?}", event);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    fn inject_event(&mut self, message: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source,
            message,
            message_id: _,
        } = message
        {
            self.events.push(BehaviourEventOut::GossipMessage {
                source: propagation_source,
                topic: message.topic,
                message: message.data,
            })
        }
    }
}

impl NetworkBehaviourEventProcess<PingEvent> for Behaviour {
    fn inject_event(&mut self, event: PingEvent) {
        match event.result {
            Result::Ok(PingSuccess::Ping { rtt }) => {
                trace!(
                    "PingSuccess::Ping rtt to {} is {} ms",
                    event.peer.to_base58(),
                    rtt.as_millis()
                );
            }
            Result::Ok(PingSuccess::Pong) => {
                trace!("PingSuccess::Pong from {}", event.peer.to_base58());
            }
            Result::Err(PingFailure::Timeout) => {
                debug!("PingFailure::Timeout {}", event.peer.to_base58());
            }
            Result::Err(PingFailure::Unsupported) => {
                debug!("PingFailure::Unsupported {}", event.peer.to_base58());
            }
            Result::Err(PingFailure::Other { error }) => {
                debug!("PingFailure::Other {}: {}", event.peer.to_base58(), error);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<<SwarmApi as libp2p::swarm::NetworkBehaviour>::OutEvent>
    for Behaviour
{
    fn inject_event(&mut self, event: <SwarmApi as libp2p::swarm::NetworkBehaviour>::OutEvent) {
        match event {
            SwarmEvent::PeerConnected(peer_id) => {
                self.events.push(BehaviourEventOut::PeerConnected(peer_id))
            }
            SwarmEvent::PeerDisconnected(peer_id) => self
                .events
                .push(BehaviourEventOut::PeerDisconnected(peer_id)),
        }
    }
}
