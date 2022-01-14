use super::signature::BLSSignature;
use crate::error::{Error, Result};
use bls12_381_plus::{multi_miller_loop, G2Affine, G2Prepared, G2Projective};
use group::{Curve, Group};
use std::ops::Neg;

const PUBLIC_KEY_SIZE: usize = 96;

#[derive(Debug, PartialEq, Eq)]
pub struct BLSPublicKey(pub(super) G2Projective);

impl crate::public_key::PublicKey for BLSPublicKey {
    type Signature = super::signature::BLSSignature;

    fn verify(&self, sig: &BLSSignature, msg: &[u8]) -> bool {
        // if self.key.is_identity().bitor(self.is_invalid()).unwrap_u8() == 1 {
        //     return false;
        // }
        let a = BLSSignature::hash_msg(msg);
        let g2 = G2Affine::generator().neg();

        multi_miller_loop(&[
            (&a.to_affine(), &G2Prepared::from(self.0.to_affine())),
            (&sig.0.to_affine(), &G2Prepared::from(g2)),
        ])
        .final_exponentiation()
        .is_identity()
        .into()
    }
    fn to_bytes(&self) -> Vec<u8> {
        BLSPublicKey::to_bytes(self).to_vec()
    }
}

impl BLSPublicKey {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; PUBLIC_KEY_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: PUBLIC_KEY_SIZE,
            found: data.len(),
        })?;
        let key_opt = G2Affine::from_compressed(bytes);
        Ok(Self(G2Projective::from(&key_opt.unwrap())))
    }

    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.0.to_affine().to_compressed()
    }
}

crate::impl_cbor!(BLSPublicKey);
