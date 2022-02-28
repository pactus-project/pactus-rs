use super::HandlerStrategy;
use crate::error::Result;
use crate::sync::bundle::bundle::Bundle;
use crate::sync::bundle::message::hello::HelloMessage;
use crate::sync::bundle::message::Message;
use crate::sync::service::ZarbSync;
use log::info;

pub struct HelloHandler {}

impl HelloHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandlerStrategy for HelloHandler {
    fn pars_message(&self, msg: Box<dyn Message>, _sync: &ZarbSync) -> Result<()> {
        let msg = msg.as_any().downcast_ref::<HelloMessage>().unwrap();
        info!("Hello message: {}", msg.moniker);

        todo!()
    }

    fn prepare_bundle(&self, mut msg: Box<dyn Message>, sync: &ZarbSync) -> Result<Bundle> {
        let hello_msg = msg.as_any_mut().downcast_mut::<HelloMessage>().unwrap();
        sync.signer.sign(hello_msg);
        Bundle::new(sync.self_id, msg)
    }
}
