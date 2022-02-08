use super::{public_key::PublicKey, signature::Signature};
use crate::address::Address;

pub trait Signable {
    fn sign_bytes(&self) -> Vec<u8>;
    fn set_public_key(&self, pk: &dyn PublicKey);
    fn set_signature(&self, sig: &dyn Signature);
}

pub trait Signer {
    fn public_key(&self) -> &dyn PublicKey;
    fn address(&self) -> &Address;
    fn sign(&self, s: &dyn Signable);
}
