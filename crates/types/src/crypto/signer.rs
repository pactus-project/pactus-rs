use super::{public_key::PublicKey, signature::Signature, secret_key::SecretKey};
use crate::address::Address;

pub trait Signable {
    fn sign_bytes(&self) -> Vec<u8>;
    fn set_public_key(&self, pk: PublicKey);
    fn set_signature(&self, sig: Signature);
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

    fn sign(&self, s: &dyn Signable) {
        let sb = s.sign_bytes();
        let sig = self.secret.sign(&sb);
        s.set_signature(sig);
        s.set_public_key(self.public.clone());
    }
}
