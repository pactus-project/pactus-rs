use super::{
    bls::{self},
    public_key::PublicKey,
    signature::Signature,
};
use crate::error::{Error, Result};

/// The secret key
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretKey {
    /// A BLS secret key.
    BLS(bls::secret_key::SecretKey),
}

impl SecretKey {
    pub fn from_bytes(key_type: super::KeyPairType, data: &[u8]) -> Result<Self> {
        Ok(match key_type {
            BLS => SecretKey::BLS(bls::secret_key::SecretKey::from_bytes(data)?),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            SecretKey::BLS(sec) => sec.to_bytes(),
        }
    }
    pub fn sanity_check(&self) -> Result<()> {
        match self {
            SecretKey::BLS(sec) => sec.sanity_check(),
        }
    }
    pub fn public_key(&self) -> PublicKey {
        match self {
            SecretKey::BLS(sec) => PublicKey::BLS(sec.public_key()),
        }
    }
    pub fn sign(&self, msg: &[u8]) -> Signature {
        match self {
            SecretKey::BLS(sec) => Signature::BLS(sec.sign(msg)),
        }
    }
}
