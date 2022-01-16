use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub network_name: String,
    pub listening_multiaddr: Multiaddr,
    pub bootstrap_peers: Vec<Multiaddr>,
    pub mdns: bool,
    pub kademlia: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network_name: "zarb_testnet".to_string(),
            listening_multiaddr: "/ip4/0.0.0.0/tcp/1347".parse().unwrap(),
            bootstrap_peers: vec!["/ip4/127.0.0.1/tcp/1347".parse().unwrap()],
            mdns: true,
            kademlia: true,
        }
    }
}
