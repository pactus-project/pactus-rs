use std::rc::Rc;
use std::sync::Arc;

use log::info;

use super::{Handler, HandlerStrategy};
use crate::error::Result;
use crate::sync::message::payload::hello::HelloPayload;
use crate::sync::service::ZarbSync;

pub struct HelloHandler {
}

impl HelloHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandlerStrategy for HelloHandler {

    fn pars_payload(&self, data: &[u8], sync: &ZarbSync) -> Result<()> {
        let pld = super::decode_payload::<HelloPayload>(data)?;
        info!("Hello payload: {}", pld.moniker);



        todo!()
    }
}
