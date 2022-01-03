use super::public_key::PublicKey;
use super::signature::Signature;
use crate::error::{Error, Result};
use bls12_381_plus::{G2Projective, Scalar};

const SECRET_KEY_SIZE: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretKey(pub(super) Scalar);

impl<'a> crate::secret_key::SecretKey<'a> for SecretKey {
    type PublicKey = super::public_key::PublicKey;
    type Signature = super::signature::Signature;

    fn public_key(&self) -> PublicKey {
        PublicKey(G2Projective::generator() * self.0)
    }

    fn sign(&self, msg: &[u8]) -> Signature {
        let g1 = Signature::hash_msg(msg.as_ref());
        Signature(g1 * self.0)
    }

    fn to_bytes(&self) -> Vec<u8> {
        SecretKey::to_bytes(&self).to_vec()
    }
}

impl SecretKey {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; SECRET_KEY_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
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

crate::impl_cbor!(SecretKey);


#[cfg(test)]
mod tests {
    use crate::{signature::Signature, secret_key::SecretKey, public_key::PublicKey};

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode("d0c6a560de2e60b6ac55386defefdf93b0c907290c2ad1b4dbd3338186bfdc68")
            .unwrap()
            .to_vec();

        let sec_key = super::SecretKey::from_bytes(buf1.as_slice()).unwrap();
        let pub_key = sec_key.public_key();
        let msg = "zarb".as_bytes();
        let sig = sec_key.sign(msg);
        println!("{}", hex::encode(pub_key.to_bytes()));
        println!("{}", hex::encode(sig.to_bytes()));

        let buf2 = hex::decode("a3a58ab4a1a15875aa8228376f5da88b6bd9839856d2b1fbd0763fdb73e89832d459109c791c3ce533fabac60028d9f9")
            .unwrap()
            .to_vec();
        let sig2 = super::Signature::from_bytes(buf2.as_slice()).unwrap();

        let buf3 = hex::decode("af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65")
            .unwrap()
            .to_vec();
        let pub_key2 = super::PublicKey::from_bytes(buf3.as_slice()).unwrap();

        let verified = pub_key2.verify(sig2, msg);
        assert!(verified);
    }
}

// af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65
