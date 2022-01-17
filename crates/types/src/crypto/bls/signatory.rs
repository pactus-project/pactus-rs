use super::BLSPublicKey;
use super::BLSSignature;
use crate::crypto::PublicKey;
use crate::crypto::Signature;
use crate::crypto::Signatory;

#[derive(Debug, PartialEq, Eq)]
pub struct BLSSignatory {
    pub pub_key: BLSPublicKey,
    pub sig: BLSSignature,
}

impl Signatory for BLSSignatory {
    fn verify(&self, msg: &[u8]) -> bool {
        self.pub_key.verify(&self.sig, msg)
    }

    fn public_key(&self) -> &dyn PublicKey {
        &self.pub_key
    }

    fn signature(&self) -> &dyn Signature {
        &self.sig
    }
}

impl BLSSignatory {
    pub fn new(pub_key: BLSPublicKey, sig: BLSSignature) -> Self {
        Self { pub_key, sig }
    }
}
