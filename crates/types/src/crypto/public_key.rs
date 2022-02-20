use super::{bls, signature::Signature, KeyPairType};
use crate::{address::Address, error::Result};
use blake2b_simd::Params;
use ripemd::{Digest, Ripemd160};

/// The public key
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicKey {
    /// A BLS Public key.
    BLS(bls::public_key::PublicKey),
}

impl PublicKey {
    pub fn from_bytes(key_type: KeyPairType, data: &[u8]) -> Result<Self> {
        Ok(match key_type {
            KeyPairType::KeyPairBLS => PublicKey::BLS(bls::public_key::PublicKey::from_bytes(data)?),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PublicKey::BLS(pk) => pk.to_bytes(),
        }
    }
    pub fn verify(&self, sig: &Signature, msg: &[u8]) -> bool {
        match self {
            PublicKey::BLS(pk) => match sig {
                Signature::BLS(sig) => pk.verify(sig, msg),
            },
        }
    }

    pub fn  sanity_check(&self) -> Result<()> {
        match self {
            PublicKey::BLS(pk) => pk.sanity_check(),
        }
    }

    pub fn address(&self) -> Address {
        let digest256 = Params::new()
            .hash_length(32)
            .to_state()
            .update(&self.to_bytes())
            .finalize();

        let mut hasher = Ripemd160::new();
        hasher.update(digest256.as_bytes());
        let digest160 = hasher.finalize();
        let mut data = digest160.to_vec();
        data.insert(0, 1);
        Address::from_bytes(&data).unwrap()
    }

    super::impl_common!();
}

