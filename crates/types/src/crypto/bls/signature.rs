use crate::error::{Error, Result};
use elliptic_curve::hash2curve::ExpandMsgXmd;
use bls12_381_plus::{elliptic_curve, G1Affine, G1Projective};
use group::Curve;

const SIGNATURE_KEY_SIZE: usize = 48;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature(pub(super) G1Projective);


impl Signature {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; SIGNATURE_KEY_SIZE] =
            data.try_into().map_err(|_| Error::InvalidLength {
                expected: SIGNATURE_KEY_SIZE,
                found: data.len(),
            })?;
        let key_opt = G1Affine::from_compressed(bytes);
        Ok(Self(G1Projective::from(&key_opt.unwrap())))
    }

    pub fn to_fixed_bytes(&self) -> [u8; SIGNATURE_KEY_SIZE] {
        self.0.to_affine().to_compressed()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_fixed_bytes().to_vec()
    }

    pub fn basic_check(&self) -> Result<()> {
        Ok(())
    }

    /// The domain separation tag
    const DST: &'static [u8] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_POP_";

    pub(super) fn hash_msg(msg: &[u8]) -> G1Projective {
        G1Projective::hash::<ExpandMsgXmd<sha2::Sha256>>(msg, Self::DST)
    }

    super::impl_common!();
}


