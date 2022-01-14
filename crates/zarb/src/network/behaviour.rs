use futures::channel::oneshot::{self, Sender as OneShotSender};
use futures::{prelude::*, stream::FuturesUnordered};
use libp2p::ping::{Ping, PingEvent};
use libp2p::request_response::{
    ProtocolSupport, RequestId, RequestResponse, RequestResponseConfig, RequestResponseEvent,
    RequestResponseMessage, ResponseChannel,
};
use libp2p::swarm::{
    NetworkBehaviour, NetworkBehaviourAction, NetworkBehaviourEventProcess, PollParameters,
};
use libp2p::NetworkBehaviour;
use libp2p::{core::identity::Keypair, kad::QueryId};
use libp2p::{core::PeerId, gossipsub::GossipsubMessage};
use libp2p::{
    gossipsub::{
        error::PublishError, error::SubscriptionError, Gossipsub, GossipsubConfigBuilder,
        GossipsubEvent, IdentTopic as Topic, MessageAuthenticity, MessageId, TopicHash,
        ValidationMode,
    },
    Multiaddr,
};
use libp2p::{
    identify::{Identify, IdentifyConfig, IdentifyEvent},
    ping::{PingFailure, PingSuccess},
};
use log::{debug, trace, warn};
use zarb_crypto::hash::Hash32;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::pin::Pin;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, convert::TryInto};
use std::{task::Context, task::Poll};
//use tiny_cid::Cid as Cid2;

use super::config::Config;
//use super::discovery::{DiscoveryBehaviour, DiscoveryOut};


/// Libp2p behaviour for the Forest node. This handles all sub protocols needed for a Filecoin node.
#[derive(NetworkBehaviour)]
#[behaviour(
    out_event = "ForestBehaviourEvent",
    poll_method = "poll",
    event_process = true
)]
pub(crate) struct ZarbBehaviour {
    gossipsub: Gossipsub,
  //  discovery: DiscoveryBehaviour,
    ping: Ping,
    identify: Identify,
    #[behaviour(ignore)]
    events: Vec<ForestBehaviourEvent>,
}


/// Event type which is emitted from the [ForestBehaviour] into the libp2p service.
#[derive(Debug)]
pub(crate) enum ForestBehaviourEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    GossipMessage {
        source: PeerId,
        topic: TopicHash,
        message: Vec<u8>,
    },
}


impl NetworkBehaviourEventProcess<GossipsubEvent> for ZarbBehaviour {
    fn inject_event(&mut self, message: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source,
            message,
            message_id: _,
        } = message
        {
            self.events.push(ForestBehaviourEvent::GossipMessage {
                source: propagation_source,
                topic: message.topic,
                message: message.data,
            })
        }
    }
}

impl NetworkBehaviourEventProcess<PingEvent> for ZarbBehaviour {
    fn inject_event(&mut self, event: PingEvent) {
        match event.result {
            Ok(PingSuccess::Ping { rtt }) => {
                trace!(
                    "PingSuccess::Ping rtt to {} is {} ms",
                    event.peer.to_base58(),
                    rtt.as_millis()
                );
            }
            Ok(PingSuccess::Pong) => {
                trace!("PingSuccess::Pong from {}", event.peer.to_base58());
            }
            Err(PingFailure::Timeout) => {
                debug!("PingFailure::Timeout {}", event.peer.to_base58());
            }
            Err(PingFailure::Other { error }) => {
                debug!("PingFailure::Other {}: {}", event.peer.to_base58(), error);
            }
            Err(PingFailure::Unsupported) => {
                debug!("PingFailure::Unsupported {}", event.peer.to_base58());
            }
        }
    }
}

impl NetworkBehaviourEventProcess<IdentifyEvent> for ZarbBehaviour {
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


impl ZarbBehaviour {
    /// Consumes the events list when polled.
    // fn poll(
    //     &mut self,
    //     cx: &mut Context,
    //     _: &mut impl PollParameters,
    // ) -> Poll<
    //     NetworkBehaviourAction<
    //         <Self as NetworkBehaviour>::OutEvent,
    //         <Self as NetworkBehaviour>::ProtocolsHandler,
    //     >,
    // > {
    //     // Poll to see if any response is ready to be sent back.
    //     while let Poll::Ready(Some(outcome)) = self.cx_pending_responses.poll_next_unpin(cx) {
    //         let RequestProcessingOutcome {
    //             inner_channel,
    //             response,
    //         } = match outcome {
    //             Some(outcome) => outcome,
    //             // The response builder was too busy and thus the request was dropped. This is
    //             // later on reported as a `InboundFailure::Omission`.
    //             None => break,
    //         };
    //         if self
    //             .chain_exchange
    //             .send_response(inner_channel, response)
    //             .is_err()
    //         {
    //             // TODO can include request id from RequestProcessingOutcome
    //             warn!("failed to send chain exchange response");
    //         }
    //     }
    //     if !self.events.is_empty() {
    //         return Poll::Ready(NetworkBehaviourAction::GenerateEvent(self.events.remove(0)));
    //     }
    //     Poll::Pending
    // }

    pub fn new(local_key: &Keypair, config: &Config) -> Self {
        let mut gs_config_builder = GossipsubConfigBuilder::default();
        // TODO
        // gs_config_builder.max_transmit_size(1 << 20);
        // gs_config_builder.validation_mode(ValidationMode::Strict);
        // gs_config_builder.message_id_fn(|msg: &GossipsubMessage| {
        //     let s = Hash32::new(&msg.data);
        //     MessageId::from(s.to_bytes())
        // });

        let gossipsub_config = gs_config_builder.build().unwrap();
        let mut gossipsub = Gossipsub::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .unwrap();

        // gossipsub
        //     .with_peer_score(
        //         build_peer_score_params(network_name),
        //         build_peer_score_threshold(),
        //     )
        //     .unwrap();

        // let mut discovery_config = DiscoveryConfig::new(local_key.public(), network_name);
        // discovery_config
        //     .with_mdns(config.mdns)
        //     .with_kademlia(config.kademlia)
        //     .with_user_defined(config.bootstrap_peers.clone())
        //     // TODO allow configuring this through config.
        //     .discovery_limit(config.target_peer_count as u64);


        let mut req_res_config = RequestResponseConfig::default();
        req_res_config.set_request_timeout(Duration::from_secs(20));
        req_res_config.set_connection_keep_alive(Duration::from_secs(20));

        ZarbBehaviour {
            gossipsub,
           // discovery: discovery_config.finish(),
            ping: Ping::default(),
            identify: Identify::new(IdentifyConfig::new("ipfs/0.1.0".into(), local_key.public())),
            events: vec![],
        }
    }

    /// Bootstrap Kademlia network
    // pub fn bootstrap(&mut self) -> Result<QueryId, String> {
    //     self.discovery.bootstrap()
    // }

    /// Publish data over the gossip network.
    pub fn publish(
        &mut self,
        topic: Topic,
        data: impl Into<Vec<u8>>,
    ) -> Result<MessageId, PublishError> {
        self.gossipsub.publish(topic, data)
    }

    /// Subscribe to a gossip topic.
    pub fn subscribe(&mut self, topic: &Topic) -> Result<bool, SubscriptionError> {
        self.gossipsub.subscribe(topic)
    }

    // /// Returns a set of peer ids
    // pub fn peers(&mut self) -> &HashSet<PeerId> {
    //     self.discovery.peers()
    // }

    // /// Returns a map of peer ids and their multiaddresses
    // pub fn peer_addresses(&mut self) -> &HashMap<PeerId, Vec<Multiaddr>> {
    //     self.discovery.peer_addresses()
    // }
}