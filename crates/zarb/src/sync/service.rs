use super::SyncService;
use super::{config::Config, firewall::firewall::Firewall};
use crate::error::Result;
use crate::network::NetworkEvent;
use crate::network::{self, NetworkMessage, NetworkService};
use async_std::channel::{Receiver, Sender};
use async_std::stream;
use async_trait::async_trait;
use futures::select;
use futures_util::stream::StreamExt;
use log::{debug, error, info, trace, warn};
use std::time::Duration;

pub(crate) struct ZarbSync {
    config: Config,
    firewall: Firewall,
    network_message_sender: Sender<NetworkMessage>,
    network_event_receiver: Receiver<NetworkEvent>,
}

impl SyncService for ZarbSync {}

#[async_trait]
impl crate::Service for ZarbSync {
    async fn start(self) {
        let mut swarm_stream = self.network_event_receiver.fuse();
        let mut interval = stream::interval(Duration::from_secs(10)).fuse();

        loop {
            select! {
                swarm_event = swarm_stream.next() => match swarm_event {
                    Some(event) => match event {
                        NetworkEvent::PeerConnected(peer_id) =>{
                            info!("peer connected {:?}", peer_id);
                        }
                        NetworkEvent::PeerDisconnected(peer_id) =>{
                            info!("peer disconnected {:?}", peer_id);
                        }
                        NetworkEvent::MessageReceived{source, topic, data} =>{
                            info!("Message received {:?}", topic);
                            match self.firewall.open_message(&data) {
                                Ok(msg) => {}
                                Err(err) => {
                                    warn!("invalid message: {}", err);
                                }
                            };
                        }
                    }
                    None => { break; }
                },
                interval_event = interval.next() => if interval_event.is_some() {
                }
            }
        }
    }
}

impl ZarbSync {
    pub fn new(config: Config, network: &mut dyn NetworkService) -> Result<Self> {
        network.register_topic("general".to_string())?;
        Ok(Self {
            config: config.clone(),
            firewall: Firewall::new(&config.firewall)?,
            network_message_sender: network.message_sender(),
            network_event_receiver: network.event_receiver(),
        })
    }
}
