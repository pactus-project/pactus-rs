use super::public_key::BLSPublicKey;
use super::signature::BLSSignature;
use crate::crypto::public_key::PublicKey;
use crate::crypto::signatory::Signatory;
use crate::crypto::signature::Signature;

#[derive(Debug, Clone, PartialEq, Eq)]
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
