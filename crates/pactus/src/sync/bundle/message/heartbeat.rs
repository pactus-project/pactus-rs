use super::Message;
use crate::error::{Error, Result};
use minicbor::{Decode, Encode};
use std::any::Any;
use pactus_types::hash::Hash32;

#[derive(Debug, Encode, Decode)]
#[cbor(map)]

pub struct HeartbeatMessage {
    #[n(1)]
    height: i32,
    #[n(2)]
    round: i32,
    #[n(3)]
    prev_block_hash: Hash32,
}

impl Message for HeartbeatMessage {
    fn basic_check(&self) -> super::Result<()> {
        if self.height < 0 {
            return Err(Error::InvalidMessage(format!(
                "invalid height: {}",
                self.height
            )));
        }
        if self.round < 0 {
            return Err(Error::InvalidMessage(format!(
                "invalid round: {}",
                self.height
            )));
        }
        Ok(())
    }

    fn message_type(&self) -> super::Type {
        super::Type::Heartbeat
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
