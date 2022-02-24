use super::Message;
use crate::error::{Error, Result};
use libp2p::PeerId;
use minicbor::{bytes::ByteVec, Decode, Encode};
use std::any::Any;
use zarb_types::crypto::public_key::PublicKey;
use zarb_types::crypto::signature::Signature;
use zarb_types::crypto::signer::Signable;
use zarb_types::crypto::KeyPairType;
use zarb_types::hash::Hash32;

#[derive(Debug, Encode, Decode)]
#[cbor(map)]
pub struct HelloMessage {
    #[n(1)]
    pub peer_id_data: ByteVec,
    #[n(2)]
    pub agent: String,
    #[n(3)]
    pub moniker: String,
    #[n(4)]
    pub public_key_data: ByteVec,
    #[n(5)]
    pub signature_data: ByteVec,
    #[n(6)]
    pub height: i32,
    #[n(7)]
    pub flags: u32,
    #[n(8)]
    pub genesis_hash: Hash32,
}

impl HelloMessage {
    pub fn new(
        peer_id: PeerId,
        moniker: String,
        height: i32,
        flags: u32,
        genesis_hash: Hash32,
    ) -> Self {
        HelloMessage {
            peer_id_data: ByteVec::from(peer_id.to_bytes()),
            agent: crate::agent(),
            public_key_data: ByteVec::from(Vec::new()),
            signature_data: ByteVec::from(Vec::new()),
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

impl Message for HelloMessage {
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

    fn message_type(&self) -> super::Type {
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

impl Signable for HelloMessage {
    fn sign_bytes(&self) -> Vec<u8> {
        format!("{}:{}:{}", self.message_type(), self.agent, self.peer_id()).into_bytes()
    }
    fn set_public_key(&mut self, pk: PublicKey) {
        self.public_key_data = ByteVec::from(pk.to_bytes())
    }
    fn set_signature(&mut self, sig: Signature) {
        self.signature_data = ByteVec::from(sig.to_bytes())
    }
}
