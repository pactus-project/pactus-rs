pub mod heartbeat;
pub mod hello;


use super::{
    bundle::{bundle::Bundle, message::Message},
    service::PactusSync,
};
use crate::error::Result;
use minicbor::Decode;


pub(super) trait HandlerStrategy: Send {
    fn pars_message(&self, msg: Box<dyn Message>, sync: &PactusSync) -> Result<()>;
    fn prepare_bundle(&self, msg: Box<dyn Message>, sync: &PactusSync) -> Result<Bundle>;
}

pub(super) struct Handler {
    strategy: Box<dyn HandlerStrategy>,
}

impl Handler {
    pub fn new(strategy: Box<dyn HandlerStrategy>) -> Self {
        Self { strategy }
    }

    pub fn do_pars_message(&self, msg: Box<dyn Message>, sync: &PactusSync) {
        self.strategy.pars_message(msg, sync).unwrap();
    }

    pub fn do_prepare_bundle(&self, msg: Box<dyn Message>, sync: &PactusSync) -> Result<Bundle> {
        let bdl = self.strategy.prepare_bundle(msg, sync)?;
        bdl.basic_check()?;
        Ok(bdl)
    }
}

fn decode_message<T, C>(data: &[u8]) -> Result<T>
where
    T: for<'r> Decode<'r, ()>,
{
    Ok(minicbor::decode::<T>(data)?)
}
