use super::{Handler, HandlerStrategy};
use crate::error::Result;
use crate::sync;
use crate::sync::bundle::message::heartbeat::HeartbeatMessage;
use crate::sync::service::ZarbSync;

pub struct HeartbeatHandler {
}

impl HeartbeatHandler {
    pub fn new() -> Self {
        Self {}
    }
}
