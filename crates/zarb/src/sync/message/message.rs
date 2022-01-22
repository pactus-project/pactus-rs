use crate::error::Result;
use libp2p::PeerId;
use minicbor::{Decode, Encode, bytes::ByteVec};
use std::vec::Vec;

use super::payload::{Payload, PayloadType};

#[derive(Debug)]
pub struct Message {
    pub payload: Box<dyn Payload>,
}

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct RawMessage {
    #[n(1)]
    pub version: i32,
    #[n(2)]
    pub flags: u32,
    #[n(3)]
    pub initiator_data: ByteVec,
    #[n(4)]
    pub payload_type: PayloadType,
    #[n(5)]
    pub payload_data: ByteVec,
}

impl RawMessage {
    pub fn new(initiator: &PeerId, payload: &dyn Payload) -> Result<Self> {
        let initiator_data = initiator.to_bytes().into();
        let payload_data = payload.to_bytes()?.into();
        Ok(RawMessage {
            version: 1,
            flags: 0,
            initiator_data,
            payload_type: payload.payload_type(),
            payload_data,
        })
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(minicbor::decode(data)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }
}
