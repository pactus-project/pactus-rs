use std::any::Any;

use super::Payload;
use crate::error::{Error, Result};
use libp2p::PeerId;
use minicbor::{Decode, Encode};
use zarb_types::crypto::bls::public_key;
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::crypto::signature::Signature;
use zarb_types::crypto::signer::Signable;
use zarb_types::crypto::KeyPairType;
use zarb_types::hash::Hash32;

#[derive(Debug, Encode, Decode)]
#[cbor(map)]
pub struct HelloPayload {
    #[cbor(n(1), with = "minicbor::bytes")]
    pub peer_id_data: Vec<u8>,
    #[n(2)]
    pub agent: String,
    #[n(3)]
    pub moniker: String,
    #[cbor(n(4), with = "minicbor::bytes")]
    pub public_key_data: Vec<u8>,
    #[cbor(n(5), with = "minicbor::bytes")]
    pub signature_data: Vec<u8>,
    #[n(6)]
    pub height: i32,
    #[n(7)]
    pub flags: u32,
    #[n(8)]
    pub genesis_hash: Hash32,
}

impl HelloPayload {
    pub fn new(
        peer_id: PeerId,
        moniker: String,
        height: i32,
        flags: u32,
        genesis_hash: Hash32,
    ) -> Self {
        HelloPayload {
            peer_id_data: peer_id.to_bytes(),
            agent: crate::agent(),
            public_key_data: Vec::new(),
            signature_data: Vec::new(),
            moniker,
            height,
            flags,
            genesis_hash,
        }
    }

    pub fn peer_id(&self) -> PeerId {
        PeerId::from_bytes(&self.peer_id_data).unwrap()
    }

    fn public_key(&self) -> PublicKey {
        PublicKey::from_bytes(KeyPairType::KeyPairBLS, &self.public_key_data).unwrap()
        // TODO; no unwrap here
    }

    fn signature(&self) -> Signature {
        Signature::from_bytes(KeyPairType::KeyPairBLS, &self.signature_data).unwrap()
        // TODO; no unwrap here
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

        if !self
            .public_key()
            .verify(&self.signature(), &self.sign_bytes())
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Signable for HelloPayload {
    fn sign_bytes(&self) -> Vec<u8> {
        format!("{}:{}:{}", self.payload_type(), self.agent, self.peer_id()).into_bytes()
    }
    fn set_public_key(&mut self, pk: PublicKey) {
        self.public_key_data = pk.to_bytes()
    }
    fn set_signature(&mut self, sig: Signature) {
        self.signature_data = sig.to_bytes()
    }
}
