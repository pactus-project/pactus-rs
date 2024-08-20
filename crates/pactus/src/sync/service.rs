use super::handler::hello::HelloHandler;
use super::handler::Handler;
use super::bundle::bundle::Bundle;
use super::bundle::message::hello::HelloMessage;
use super::bundle::message::{Message, Type as MessageType};
use super::SyncService;
use super::{config::Config, firewall::firewall::Firewall};
use crate::error::{Result};
use crate::network::NetworkEvent;
use crate::network::{NetworkMessage, NetworkService};
use async_std::channel::{Receiver, Sender};
use async_std::stream;
use async_trait::async_trait;
use futures::select;
use futures_util::stream::StreamExt;
use log::{error, info, warn};

use pactus_types::crypto::signer::Signer;

use std::collections::BTreeMap;


use pactus_types::hash::Hash32;
use libp2p::{PeerId};

pub(super) struct PactusSync {
    pub config: Config,
    pub self_id: PeerId,
    pub signer: Signer,
    firewall: Firewall,
    handlers: BTreeMap<MessageType, Handler>,
    network_message_sender: Sender<NetworkMessage>,
    network_event_receiver: Receiver<NetworkEvent>,
}

impl SyncService for PactusSync {}

impl PactusSync {
    pub fn new(
        config: Config,
        signer: Signer,
        network: &mut dyn NetworkService,
    ) -> Result<Self> {
        let mut handlers: BTreeMap<MessageType, Handler> = BTreeMap::new();

        let slm = HelloHandler::new();

        handlers.insert(MessageType::Hello, Handler::new(Box::new(slm)));

        Ok(Self {
            self_id: network.self_id(),
            signer,
            firewall: Firewall::new(&config.firewall)?,
            config,
            handlers,
            network_message_sender: network.message_sender(),
            network_event_receiver: network.event_receiver(),
        })
    }

    pub fn say_hello(&self, need_response: bool) {
        let h = hex::decode("2b1187d5f470de8d186634e3e227c528a292c0a3ba95fae924be27e8e7f46fa0").unwrap();
        let mut flags = 0;
        if need_response {
            flags |= 1;
        }
        let msg = HelloMessage::new(
            self.self_id,
            self.config.moniker.clone(),
            0,
            flags,
            Hash32::from_bytes(&h).unwrap(),
        );
        self.broadcast(Box::new(msg));
    }

    fn broadcast(&self, msg : Box<dyn Message>) {
        let bdl  = self.prepare_bundle(msg).unwrap();
        let msg_data = NetworkMessage::GeneralMessage {
            data: bdl.to_bytes().unwrap(),
        };
        self.network_message_sender.try_send(msg_data).unwrap();
    }

    fn prepare_bundle(&self, msg: Box<dyn Message>) -> Result<Bundle> {
        let handler =  self.handlers.get(&msg.message_type()).unwrap();
        handler.do_prepare_bundle(msg, self)
    }
}

#[async_trait]
impl crate::Service for PactusSync {
    async fn start(self) {
        let mut heartbeat_ticker = stream::interval(self.config.heartbeat_timeout).fuse();
        let mut network_stream = self.network_event_receiver.clone().fuse();

        self.say_hello(true);

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
                        NetworkEvent::MessageReceived{source: _, data} =>{
                            match self.firewall.open_bundle(&data) {
                                Ok(bdl) => {
                                    match self.handlers.get(&bdl.message_type()) {
                                        Some(handler) => {
                                            handler.do_pars_message(bdl.message, &self);
                                        }
                                        None => {
                                            error!("invalid message type: {:?}", bdl.message_type())
                                        }
                                    }
                                }
                                Err(err) => {
                                    warn!("invalid bundle: {}", err);
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

