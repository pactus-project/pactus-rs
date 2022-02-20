use super::{bls, KeyPairType};
use crate::error::Result;

/// The secret key
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signature {
    /// A BLS secret key.
    BLS(bls::signature::Signature),
}

impl Signature {
    pub fn from_bytes(key_type: KeyPairType, data: &[u8]) -> Result<Self> {
        Ok(match key_type {
            KeyPairType::KeyPairBLS => Signature::BLS(bls::signature::Signature::from_bytes(data)?),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Signature::BLS(sig) => sig.to_bytes(),
        }
    }
    pub fn sanity_check(&self) -> Result<()> {
        match self {
            Signature::BLS(sig) => sig.sanity_check(),
        }
    }

    super::impl_common!();
}

