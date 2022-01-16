use super::public_key::BLSPublicKey;
use super::signature::BLSSignature;
use crate::error::{Error, Result};
use crate::crypto::secret_key::SecretKey;
use bls12_381_plus::{G2Projective, Scalar};

const SECRET_KEY_SIZE: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct BLSSecretKey(pub(super) Scalar);

impl SecretKey for BLSSecretKey {
    type PublicKey = super::public_key::BLSPublicKey;
    type Signature = super::signature::BLSSignature;

    fn public_key(&self) -> BLSPublicKey {
        BLSPublicKey(G2Projective::generator() * self.0)
    }

    fn sign(&self, msg: &[u8]) -> BLSSignature {
        let g1 = BLSSignature::hash_msg(msg);
        BLSSignature(g1 * self.0)
    }

    fn to_bytes(&self) -> Vec<u8> {
        BLSSecretKey::to_bytes(self).to_vec()
    }
}

impl BLSSecretKey {
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
        let key_opt = Scalar::from_bytes(bytes);
        Ok(Self(key_opt.unwrap()))
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.0.to_bytes()
    }
}

super::impl_cbor!(BLSSecretKey);

#[cfg(test)]
mod tests {
    use crate::crypto::secret_key::SecretKey;

    #[test]
    fn test_decoding() {
        let sec_buf =
            hex::decode("68dcbf868133d3dbb4d12a0c2907c9b093dfefef6d3855acb6602ede60a5c6d0")
                .unwrap();
        let pub_buf = hex::decode("af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65").unwrap();
        let sig_buf = hex::decode("a2d06b33af2c9e7ca878da85a96b2c2346f4306d0473bdabc38be87c19dae5e67e08724a5220d0e372fb080bbd2fbde9").unwrap();
        let msg = "zarb".as_bytes();

        let sec_key = super::BLSSecretKey::from_bytes(sec_buf.as_slice()).unwrap();
        let pub_key = super::BLSPublicKey::from_bytes(pub_buf.as_slice()).unwrap();
        let sig = super::BLSSignature::from_bytes(sig_buf.as_slice()).unwrap();

        assert_eq!(sec_key.public_key(), pub_key);
        assert_eq!(sec_key.sign(msg), sig);
        assert!(pub_key.verify(&sig, msg));
    }
}
