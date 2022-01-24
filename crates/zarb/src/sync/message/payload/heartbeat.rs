use super::Payload;
use crate::error::{Error, Result};
use minicbor::{Decode, Encode};
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::{crypto::bls::public_key::BLSPublicKey, hash::Hash32};

#[derive(Debug, Encode, Decode)]
#[cbor(map)]

pub struct HeartbeatPayload {
    #[n(1)]
    height: i32,
    #[n(2)]
    round: i32,
    #[n(3)]
    prev_block_hash: Hash32,
}

impl Payload for HeartbeatPayload {
    fn sanity_check(&self) -> super::Result<()> {
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

    fn payload_type(&self) -> super::Type {
        super::Type::Heartbeat
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }
}
