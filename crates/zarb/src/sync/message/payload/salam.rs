use super::Payload;
use crate::error::{Error, Result};
use minicbor::{Decode, Encode};
use zarb_types::crypto::bls::public_key;
use zarb_types::crypto::bls::signature::BLSSignature;
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::{crypto::bls::public_key::BLSPublicKey, hash::Hash32};

#[derive(Debug, Encode, Decode)]
#[cbor(map)]

pub struct SalamPayload {
    #[n(1)]
    pub agent: String,
    #[n(2)]
    pub moniker: String,
    #[n(3)]
    pub public_key: BLSPublicKey,
    #[n(4)]
    pub signature: BLSSignature,
    #[n(5)]
    pub height: i32,
    #[n(6)]
    pub flags: u32,
    #[n(7)]
    pub genesis_hash: Hash32,
}

impl SalamPayload {
    pub fn new(
        moniker: String,
        public_key: BLSPublicKey,
        signature: BLSSignature,
        height: i32,
        flags: u32,
        genesis_hash: Hash32,
    ) -> Self {
        SalamPayload {
            agent: crate::agent(),
            moniker,
            public_key,
            signature,
            height,
            flags,
            genesis_hash,
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
        if !self
            .public_key
            .verify(&self.signature, &self.public_key.to_bytes())
        {
            return Err(Error::InvalidMessage("invalid public key".to_string()));
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
