use super::{message::payload::Payload, service::ZarbSync};
use crate::error::Result;
use minicbor::Decode;

pub mod salam;
pub mod heartbeat;

pub(super) trait HandlerStrategy: Send {
    fn pars_payload(&self, data: &[u8], sync: &ZarbSync) -> Result<()>;

}


pub(super) struct Handler {
    strategy: Box<dyn HandlerStrategy>,
}

impl Handler {
    pub fn new(strategy: Box<dyn HandlerStrategy>) -> Self {
        Self { strategy }
    }

    pub fn do_pars_payload(&self, data: &[u8], sync: &ZarbSync) {
        self.strategy.pars_payload(data, sync).unwrap();
    }
}


fn decode_payload<T>(data: &[u8]) -> Result<T>
where
    T: for<'r> Decode<'r>,
{
    Ok(minicbor::decode::<T>(data)?)
}
