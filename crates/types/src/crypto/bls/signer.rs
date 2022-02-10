use super::{public_key::BLSPublicKey, secret_key::BLSSecretKey};
use crate::{
    address::Address,
    crypto::{
        public_key::PublicKey,
        signer::{Signable, Signer}, secret_key::SecretKey,
    },
};

#[derive(Debug)]
pub struct BLSSigner {
    secret: BLSSecretKey,
    public: BLSPublicKey,
    address: Address,
}

impl BLSSigner {
    pub fn new(secret: BLSSecretKey) -> Self {
        Self {
            address: secret.public_key().address(),
            public: secret.public_key(),
            secret,
        }
    }

    pub fn public_key(&self) -> BLSPublicKey {
        self.public.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }
}

impl Signer for BLSSigner {
    fn public_key(&self) -> &dyn PublicKey {
        &self.public
    }

    fn address(&self) -> &Address {
        &self.address
    }

    fn sign(&self, s: &dyn Signable) {
        let sb = s.sign_bytes();
        let sig = self.secret.sign(&sb);
        s.set_signature(&sig);
        s.set_public_key(&self.public.clone());
    }
}
