use crate::error::{Error, Result};
use bls12_381_plus::{multi_miller_loop, G2Affine, G2Prepared, G2Projective};
use group::{Curve, Group};
use std::ops::Neg;

use super::signature::Signature;

const PUBLIC_KEY_SIZE: usize = 96;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey(pub(super) G2Projective);


impl PublicKey {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; PUBLIC_KEY_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: PUBLIC_KEY_SIZE,
            found: data.len(),
        })?;
        let key_opt = G2Affine::from_compressed(bytes);
        Ok(Self(G2Projective::from(&key_opt.unwrap())))
    }

    pub fn to_fixed_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.0.to_affine().to_compressed()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_fixed_bytes().to_vec()
    }

    pub fn basic_check(&self) -> Result<()> {
        Ok(())
    }

    pub fn verify(&self, sig: &Signature, msg: &[u8]) -> bool {
        let hash = Signature::hash_msg(msg);
        let g2 = G2Affine::generator().neg();

        multi_miller_loop(&[
            (&hash.to_affine(), &G2Prepared::from(self.0.to_affine())),
            (&sig.0.to_affine(), &G2Prepared::from(g2)),
        ])
        .final_exponentiation()
        .is_identity()
        .into()
    }

    super::impl_common!();
}


