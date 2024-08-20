use super::public_key::PublicKey;
use super::signature::Signature;
use crate::error::{Error, Result};
use bls12_381_plus::{G2Projective, Scalar};
use group::ff::Field;
use rand::rngs::OsRng;

const SECRET_KEY_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretKey(pub(super) Scalar);

impl SecretKey {
    pub fn random() -> Self {
        let rng = &mut OsRng::default();
        Self(Scalar::random(rng))
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut data_be = data.to_vec();
        data_be.reverse(); // converting to big-endian
        let bytes: &[u8; SECRET_KEY_SIZE] =
            data_be
                .as_slice()
                .try_into()
                .map_err(|_| Error::InvalidLength {
                    expected: SECRET_KEY_SIZE,
                    found: data.len(),
                })?;
        let key_opt = Scalar::from_be_bytes(bytes);
        Ok(Self(key_opt.unwrap()))
    }

    pub fn to_fixed_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        let mut data = self.0.to_be_bytes();
        data.reverse();
        data
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_fixed_bytes().to_vec()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(G2Projective::GENERATOR * self.0)
    }

    pub fn basic_check(&self) -> Result<()> {
        Ok(())
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        let g1 = Signature::hash_msg(msg);
        Signature(g1 * self.0)
    }

    super::impl_common!();
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_decoding() {
        let sec_hex = "68dcbf868133d3dbb4d12a0c2907c9b093dfefef6d3855acb6602ede60a5c6d0";
        let pk_hex = "af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65";
        let sig_hex = "a2d06b33af2c9e7ca878da85a96b2c2346f4306d0473bdabc38be87c19dae5e67e08724a5220d0e372fb080bbd2fbde9";
        let msg = "pactus".as_bytes();

        let sec = super::SecretKey::from_string(sec_hex).unwrap();
        let pk = super::PublicKey::from_string(pk_hex).unwrap();
        let sig = super::Signature::from_string(sig_hex).unwrap();

        assert_eq!(sec.public_key(), pk);
        assert_eq!(sec.sign(msg), sig);
        assert!(pk.verify(&sig, msg));
    }
}
