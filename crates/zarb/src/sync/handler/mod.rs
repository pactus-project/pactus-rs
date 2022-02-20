use std::any::Any;

use super::{
    message::{message::Message, payload::Payload},
    service::ZarbSync,
};
use crate::error::Result;
use minicbor::Decode;

pub mod heartbeat;
pub mod hello;

pub(super) trait HandlerStrategy: Send {
    fn pars_payload(&self, pld: Box<dyn Payload>, sync: &ZarbSync) -> Result<()>;
    fn prepare_message(&self, pld: Box<dyn Payload>, sync: &ZarbSync) -> Result<Message>;
}

pub(super) struct Handler {
    strategy: Box<dyn HandlerStrategy>,
}

impl Handler {
    pub fn new(strategy: Box<dyn HandlerStrategy>) -> Self {
        Self { strategy }
    }

    pub fn do_pars_payload(&self, pld: Box<dyn Payload>, sync: &ZarbSync) {
        self.strategy.pars_payload(pld, sync).unwrap();
    }

    pub fn do_prepare_message(&self, pld: Box<dyn Payload>, sync: &ZarbSync) -> Result<Message> {
        let msg = self.strategy.prepare_message(pld, sync)?;
        msg.sanity_check()?;
        Ok(msg)
    }
}

fn decode_payload<T>(data: &[u8]) -> Result<T>
where
    T: for<'r> Decode<'r>,
{
    Ok(minicbor::decode::<T>(data)?)
}
