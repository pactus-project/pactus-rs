use super::handler::hello::HelloHandler;
use super::handler::Handler;
use super::message::message::Message;
use super::message::payload::hello::HelloPayload;
use super::message::payload::{Payload, Type as PayloadType};
use super::SyncService;
use super::{config::Config, firewall::firewall::Firewall};
use crate::error::{self, Result};
use crate::network::NetworkEvent;
use crate::network::{self, NetworkMessage, NetworkService};
use async_std::channel::{Receiver, Sender};
use async_std::stream;
use async_trait::async_trait;
use futures::select;
use futures_util::stream::StreamExt;
use log::{debug, error, info, trace, warn};
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::crypto::signer::Signer;
use std::collections::BTreeMap;
use std::thread::sleep;
use std::time::Duration;
use zarb_types::crypto::bls::signer::BLSSigner;
use zarb_types::hash::Hash32;
use libp2p::{identity, PeerId};

pub(super) struct ZarbSync {
    pub config: Config,
    pub self_id: PeerId,
    signer: BLSSigner,
    firewall: Firewall,
    handlers: BTreeMap<PayloadType, Handler>,
    network_message_sender: Sender<NetworkMessage>,
    network_event_receiver: Receiver<NetworkEvent>,
}

impl SyncService for ZarbSync {}

impl ZarbSync {
    pub fn new(
        config: Config,
        signer: BLSSigner,
        network: &mut dyn NetworkService,
    ) -> Result<Self> {
        let mut handlers: BTreeMap<PayloadType, Handler> = BTreeMap::new();

        let slm = HelloHandler::new();

        handlers.insert(PayloadType::Hello, Handler::new(Box::new(slm)));

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

    fn say_hello(&self) {
        let pld = HelloPayload::new(
            self.self_id,
            self.config.moniker.clone(),
            0,
            0,
            Hash32::calculate("zarb".as_bytes()),
        );
        self.broadcast(Box::new(pld));
    }

    fn broadcast(&self, pld : Box<dyn Payload>) {
        let msg  = self.prepare_message(pld).unwrap();
        let msg_data = NetworkMessage::GeneralMessage {
            data: msg.to_bytes().unwrap(),
        };
        self.network_message_sender.try_send(msg_data).unwrap();
    }

    fn prepare_message(&self, pld: Box<dyn Payload>) -> Result<Message> {
        let handler =  self.handlers.get(&pld.payload_type()).unwrap();
        handler.do_prepare_message(pld, self)
    }
}

#[async_trait]
impl crate::Service for ZarbSync {
    async fn start(self) {
        let mut heartbeat_ticker = stream::interval(self.config.heartbeat_timeout).fuse();
        let mut network_stream = self.network_event_receiver.clone().fuse();

        self.say_hello();

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
                                Ok(msg) => {
                                    match self.handlers.get(&msg.payload_type()) {
                                        Some(handler) => {
                                            handler.do_pars_payload(msg.payload, &self);
                                        }
                                        None => {
                                            error!("invalid payload type: {:?}", msg.payload_type())
                                        }
                                    }
                                }
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

