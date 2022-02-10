use super::{Handler, HandlerStrategy};
use crate::error::Result;
use crate::sync;
use crate::sync::message::payload::heartbeat::HeartbeatPayload;
use crate::sync::service::ZarbSync;

pub struct HeartbeatHandler {
}

impl HeartbeatHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandlerStrategy for HeartbeatHandler {

    fn pars_payload(&self, data: &[u8], sync: &ZarbSync) -> Result<()> {
        let pld = super::decode_payload::<HeartbeatPayload>(data)?;
        todo!()
    }
}
