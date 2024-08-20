use core::task::{Context, Poll};
use libp2p::core::{connection::ConnectionId, Multiaddr, PeerId};
use libp2p::swarm::handler::DummyConnectionHandler;
use libp2p::swarm::{
    ConnectionHandler, NetworkBehaviour, NetworkBehaviourAction, PollParameters, SwarmEvent,

};
use log::trace;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum SwarmEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
}

#[derive(Debug, Default)]
pub struct Swarm {
    events: VecDeque<SwarmEvent>,
}

impl Swarm {
    pub fn new() -> Self {
        Swarm {
            events: VecDeque::new(),
        }
    }
}

impl NetworkBehaviour for Swarm {
    type ConnectionHandler = DummyConnectionHandler;
    type OutEvent = SwarmEvent;

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        trace!("new_handler");
        Default::default()
    }

    fn addresses_of_peer(&mut self, _peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    // fn inject_connected(&mut self, peer_id: &PeerId) {
    //     self.events
    //         .push_back(SwarmEvent::PeerConnected(*peer_id))
    // }

    // fn inject_disconnected(&mut self, peer_id: &PeerId) {
    //     self.events
    //         .push_back(SwarmEvent::PeerDisconnected(*peer_id))
    // }

    fn inject_event(
        &mut self,
        _peer_id: PeerId,
        _connection: ConnectionId,
        _event: <<Self::ConnectionHandler as ConnectionHandler>::Handler as ProtocolsHandler>::OutEvent,
    ) {
    }

    fn poll(
        &mut self,
        _: &mut Context,
        _: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self as NetworkBehaviour>::OutEvent,
            <Self as NetworkBehaviour>::ConnectionHandler,
        >,
    > {
        if let Some(event) = self.events.pop_front() {
            trace!("Polling swarm event");
            Poll::Ready(NetworkBehaviourAction::GenerateEvent(event))
        } else {
            Poll::Pending
        }
    }
}
