use crate::public_key::PublicKey;
use super::public_key::BLSPublicKey;
use super::signature::BLSSignature;

#[derive(Debug, PartialEq, Eq)]
pub struct BLSSignatory {
    pub pub_key: BLSPublicKey,
    pub sig: BLSSignature,
}

impl crate::signatory::Signatory for BLSSignatory {
    fn verify(&self, msg: &[u8]) -> bool {
        self.pub_key.verify(&self.sig, msg)
    }
}

impl BLSSignatory {
    pub fn new(pub_key: BLSPublicKey, sig: BLSSignature) -> Self {
        Self { pub_key, sig }
    }
}
