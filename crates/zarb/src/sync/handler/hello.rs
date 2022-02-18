use std::rc::Rc;
use std::sync::Arc;

use log::info;

use super::{Handler, HandlerStrategy};
use crate::error::Result;
use crate::sync::message::message::Message;
use crate::sync::message::payload::hello::HelloPayload;
use crate::sync::message::payload::Payload;
use crate::sync::service::ZarbSync;

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

    fn prepare_message(&self, pld: Box<dyn Payload>, sync: &ZarbSync) -> Result<Message> {
        let p = pld.as_any().downcast_ref::<HelloPayload>().unwrap();
        Message::new(sync.self_id,pld)
    }
}
