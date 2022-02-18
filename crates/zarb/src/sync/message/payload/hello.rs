use std::any::Any;

use super::Payload;
use crate::error::{Error, Result};
use minicbor::{Decode, Encode};
use zarb_types::crypto::bls::public_key;
use zarb_types::crypto::bls::signature::BLSSignature;
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::crypto::signature::Signature;
use zarb_types::crypto::signer::Signable;
use zarb_types::{crypto::bls::public_key::BLSPublicKey, hash::Hash32};
use libp2p::PeerId;

#[derive(Debug, Encode, Decode)]
#[cbor(map)]
pub struct HelloPayload {
    #[n(1)]
    pub peer_id_data: Vec<u8>,
    #[n(2)]
    pub agent: String,
    #[n(3)]
    pub moniker: String,
    #[n(4)]
    pub public_key: Option<BLSPublicKey>,
    #[n(5)]
    pub signature: Option<BLSSignature>,
    #[n(6)]
    pub height: i32,
    #[n(7)]
    pub flags: u32,
    #[n(8)]
    pub genesis_hash: Hash32,
}

impl HelloPayload {
    pub fn new(peer_id: PeerId, moniker: String, height: i32, flags: u32, genesis_hash: Hash32) -> Self {
        HelloPayload {
            peer_id_data: peer_id.to_bytes(),
            agent: crate::agent(),
            public_key: None,
            signature: None,
            moniker,
            height,
            flags,
            genesis_hash,
        }
    }

    pub fn peer_id(&self) -> PeerId {
        PeerId::from_bytes(&self.peer_id_data).unwrap()
    }
}

impl Payload for HelloPayload {
    fn sanity_check(&self) -> super::Result<()> {
        if self.height < 0 {
            return Err(Error::InvalidMessage(format!(
                "invalid height: {}",
                self.height
            )));
        }
        //if
        if self.public_key.is_none() {
            return Err(Error::InvalidMessage("public key is not set".to_string()));
        }
        if self.signature.is_none() {
            return Err(Error::InvalidMessage("signature is not set".to_string()));
        }
        if !self
            .public_key.as_ref().unwrap()
            .verify(&self.signature.as_ref().unwrap(), &self.sign_bytes())
        {
            return Err(Error::InvalidMessage("invalid public key".to_string()));
        }
        Ok(())
    }

    fn payload_type(&self) -> super::Type {
        super::Type::Hello
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Signable for HelloPayload {
    fn sign_bytes(&self) -> Vec<u8> {
        format!("{}:{}:{}", self.payload_type(), self.agent, self.peer_id()).into_bytes()
    }
    // fn set_public_key(&self, pk: &dyn PublicKey) {
    //     self.public_key = Some(pk)
    // }
    // fn set_signature(&self, sig: &dyn Signature) {
    //     self.signature = Some(sig)
    // }
}
