use std::rc::Rc;
use std::sync::Arc;

use log::info;

use super::{Handler, HandlerStrategy};
use crate::error::Result;
use crate::sync::message::payload::salam::SalamPayload;
use crate::sync::service::ZarbSync;

pub struct SalamHandler {
}

impl SalamHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandlerStrategy for SalamHandler {

    fn pars_payload(&self, data: &[u8], sync: &ZarbSync) -> Result<()> {
        let salam = super::decode_payload::<SalamPayload>(data)?;
        info!("salam payload: {}", salam.moniker);

        

        todo!()
    }
}
