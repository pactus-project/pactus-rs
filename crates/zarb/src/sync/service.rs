use super::SyncService;
use super::message::payload::Payload;
use super::message::payload::salam::SalamPayload;
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
use zarb_types::crypto::signer::Signer;
use std::time::Duration;

pub(crate) struct ZarbSync {
    config: Config,
    signer: Signer,
    firewall: Firewall,
    network_message_sender: Sender<NetworkMessage>,
    network_event_receiver: Receiver<NetworkEvent>,
}

impl SyncService for ZarbSync {}

#[async_trait]
impl crate::Service for ZarbSync {
    async fn start(self) {
        let mut heartbeat_ticker =
            stream::interval(self.config.heartbeat_timeout).fuse();
        let mut network_stream = self.network_event_receiver.fuse();

        // let pld = SalamPayload::new(self.config.moniker, );
        // let msg = NetworkMessage::GeneralMessage{
        //     data: pld.to_bytes().unwrap()
        // };
        // self.network_message_sender.send(msg);

        loop {
            select! {
                network_event = network_stream.next() => match network_event {
                    Some(event) => match event {
                        NetworkEvent::PeerConnected(peer_id) =>{
                            info!("peer connected {:?}", peer_id);
                        }
                        NetworkEvent::PeerDisconnected(peer_id) =>{
                            info!("peer disconnected {:?}", peer_id);
                        }
                        NetworkEvent::MessageReceived{source, data} =>{
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
                heartbeat_timeout = heartbeat_ticker.next() => if heartbeat_timeout.is_some() {
                }
            }
        }
    }
}

impl ZarbSync {
    pub fn new(config: Config, signer: Signer, network: &mut dyn NetworkService) -> Result<Self> {
        Ok(Self {
            signer,
            firewall: Firewall::new(&config.firewall)?,
            config: config,
            network_message_sender: network.message_sender(),
            network_event_receiver: network.event_receiver(),
        })
    }
}
