use super::{public_key::PublicKey, signature::Signature, secret_key::SecretKey};
use crate::address::Address;

pub trait Signable {
    fn sign_bytes(&self) -> Vec<u8>;
    fn set_public_key(&mut self, pk: PublicKey);
    fn set_signature(&mut self, sig: Signature);
}



#[derive(Debug)]
pub struct Signer {
    secret: SecretKey,
    public: PublicKey,
    address: Address,
}

impl Signer {
    pub fn new(secret: SecretKey) -> Self {
        Self {
            address: secret.public_key().address(),
            public: secret.public_key(),
            secret,
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.public.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn sign(&self, signable: &mut dyn Signable) {
        let sb = signable.sign_bytes();
        let sig = self.secret.sign(&sb);
        signable.set_signature(sig);
        signable.set_public_key(self.public.clone());
    }
}
