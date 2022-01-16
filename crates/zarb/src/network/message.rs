use crate::error::Result;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub version: i32,
    pub flags: u32,
    pub initiator: PeerId,
    pub message: Vec<u8>,
}

impl Message {
    pub fn new(initiator: PeerId, message: Vec<u8>) -> Self {
        Message {
            version: 1,
            flags: 0,
            initiator,
            message,
        }
    }
}

// impl TryInto<Vec<u8>> for Message {
//     type Error = crate::error::Error;

//     fn try_into(self) -> Result<Vec<u8>> {
//         Ok(minicbor::to_vec(&self)?)
//     }
// }

// impl TryInto<Message> for Vec<u8> {
//     type Error = crate::error::Error;

//     fn try_into(self) -> Result<Message> {
//         Ok(minicbor::decode(&self)?)
//     }
// }
