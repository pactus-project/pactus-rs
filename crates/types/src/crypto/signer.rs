use super::{bls::signature, public_key::PublicKey, secret_key::SecretKey, signature::Signature};
use crate::address::Address;

pub trait Signable {
    fn sign_bytes(&self) -> Vec<u8>;
    fn set_public_key(&self, pk: Box<dyn PublicKey>);
    fn set_signature(&self, sig: Box<dyn Signature>);
}

#[derive(Debug)]
pub struct Signer {
    secret: Box<dyn SecretKey>,
}

impl Signer {
    pub fn new(secret: Box<dyn SecretKey>) -> Self {
        Signer {
            secret: secret,
        }
    }

    pub fn public_key(&self) -> Box<dyn PublicKey> {
        self.secret.public_key()
    }

    pub fn address(&self) -> Address {
        self.secret.public_key().address()
    }

    pub fn sign(&self, s: &dyn Signable) {
        let sb = s.sign_bytes();
        let sig = self.secret.sign(&sb);
        s.set_signature(sig);
        s.set_public_key(self.public_key());
    }
}
