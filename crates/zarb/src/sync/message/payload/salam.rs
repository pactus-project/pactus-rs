use super::Payload;
use crate::error::{Error, Result};
use minicbor::{Decode, Encode};
use zarb_types::crypto::bls::public_key;
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::{crypto::bls::public_key::BLSPublicKey, hash::Hash32};

#[derive(Debug, Encode, Decode)]
#[cbor(map)]

pub struct SalamPayload {
    #[n(1)]
    agent: String,
    #[n(2)]
    moniker: String,
    #[n(3)]
    public_key: BLSPublicKey,
    #[n(4)]
    genesis_hash: Hash32,
    #[n(5)]
    height: i32,
    #[n(6)]
    flags: u32,
}

impl SalamPayload {
    pub fn new(
        moniker: String,
        public_key: BLSPublicKey,
        genesis_hash: Hash32,
        height: i32,
        flags: u32,
    ) -> Self {
        SalamPayload {
            agent: format!("zarb-rs/{}", env!("CARGO_PKG_VERSION")),
            moniker,
            public_key,
            genesis_hash,
            height,
            flags,
        }
    }
}

impl Payload for SalamPayload {
    fn sanity_check(&self) -> super::Result<()> {
        if self.height < 0 {
            return Err(Error::InvalidMessage(format!(
                "invalid height: {}",
                self.height
            )));
        }
        if let Err(err) = self.public_key.sanity_check() {
            return Err(Error::InvalidMessage(format!(
                "invalid public key: {}",
                err
            )));
        }
        Ok(())
    }

    fn payload_type(&self) -> super::Type {
        super::Type::Salam
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }
}
