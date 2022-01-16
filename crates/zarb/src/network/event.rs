use libp2p::PeerId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
}
