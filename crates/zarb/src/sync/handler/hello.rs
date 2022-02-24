use super::HandlerStrategy;
use crate::error::Result;
use crate::sync::message::message::Message;
use crate::sync::message::payload::hello::HelloPayload;
use crate::sync::message::payload::Payload;
use crate::sync::service::ZarbSync;
use log::info;

pub struct HelloHandler {}

impl HelloHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandlerStrategy for HelloHandler {
    fn pars_payload(&self, pld: Box<dyn Payload>, sync: &ZarbSync) -> Result<()> {
        let pld = pld.as_any().downcast_ref::<HelloPayload>().unwrap();
        info!("Hello payload: {}", pld.moniker);

        todo!()
    }

    fn prepare_message(&self, mut pld: Box<dyn Payload>, sync: &ZarbSync) -> Result<Message> {
        let hello_pld = pld.as_any_mut().downcast_mut::<HelloPayload>().unwrap();
        sync.signer.sign(hello_pld);
        Message::new(sync.self_id, pld)
    }
}
