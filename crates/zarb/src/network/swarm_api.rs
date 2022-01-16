use core::task::{Context, Poll};
use libp2p::swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters};
use libp2p::{
    core::{connection::ConnectionId, Multiaddr, PeerId},
    swarm::{
        protocols_handler::{DummyProtocolsHandler, IntoProtocolsHandler, ProtocolsHandler},
        toggle::{Toggle, ToggleIntoProtoHandler},
    },
};
use log::trace;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum SwarmEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
}

#[derive(Debug, Default)]
pub struct SwarmApi {
    events: VecDeque<SwarmEvent>,
}

impl SwarmApi {
    pub fn new() -> Self {
        SwarmApi {
            events: VecDeque::new(),
        }
    }
}

impl NetworkBehaviour for SwarmApi {
    type ProtocolsHandler = ToggleIntoProtoHandler<DummyProtocolsHandler>;
    type OutEvent = SwarmEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        trace!("new_handler");
        todo!()
    }

    fn addresses_of_peer(&mut self, _peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {
        self.events
            .push_back(SwarmEvent::PeerConnected(peer_id.clone()))
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        self.events
            .push_back(SwarmEvent::PeerDisconnected(peer_id.clone()))
    }

    fn inject_event(
        &mut self,
        _peer_id: PeerId,
        _connection: ConnectionId,
        _event: <<Self::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::OutEvent,
    ) {
    }

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
        if let Some(event) = self.events.pop_front() {
            trace!("Polling swarm event");
            Poll::Ready(NetworkBehaviourAction::GenerateEvent(event))
        } else {
            Poll::Pending
        }
    }
}
