use super::public_key::PublicKey;
use super::signature::Signature;
use crate::error::{Error, Result};
use bls12_381_plus::{G2Projective, Scalar};

const SECRET_KEY_SIZE: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretKey(pub(super) Scalar);

impl crate::secret_key::SecretKey for SecretKey {
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
        self.0.to_bytes().to_vec()
    }
}

impl SecretKey {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; SECRET_KEY_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: SECRET_KEY_SIZE,
            found: data.len(),
        })?;
        let key_opt = Scalar::from_bytes(bytes);
        Ok(SecretKey(key_opt.unwrap()))
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.0.to_bytes()
    }
}

impl minicbor::Encode for SecretKey {
    fn encode<W>(
        &self,
        e: &mut minicbor::Encoder<W>,
    ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
    where
        W: minicbor::encode::Write,
    {
        e.bytes(&self.to_bytes())?;
        Ok(())
    }
}

impl<'a> minicbor::Decode<'a> for SecretKey {
    fn decode(
        d: &mut minicbor::Decoder<'a>,
    ) -> core::result::Result<SecretKey, minicbor::decode::Error> {
        Ok(SecretKey::from_bytes(d.bytes()?)
            .map_err(|_| minicbor::decode::Error::Message("error"))?)
    }
}
#[cfg(test)]
mod tests {
    use crate::{signature::Signature, secret_key::SecretKey, public_key::PublicKey};

    #[test]
    fn test_decoding() {
        // let buf1 =
        //     hex!("d0c6a560de2e60b6ac55386defefdf93b0c907290c2ad1b4dbd3338186bfdc68").to_vec();
        let buf1 = hex::decode("d0c6a560de2e60b6ac55386defefdf93b0c907290c2ad1b4dbd3338186bfdc68")
            .unwrap()
            .to_vec();

        let sec_key = super::SecretKey::from_bytes(buf1.as_slice()).unwrap();
        let pub_key = sec_key.public_key();
        let msg = "zarb".as_bytes();
        let sig = sec_key.sign(msg);
        println!("{}", hex::encode(pub_key.to_bytes()));
        println!("{}", hex::encode(sig.to_bytes()));

        let buf2 = hex::decode("a2d06b33af2c9e7ca878da85a96b2c2346f4306d0473bdabc38be87c19dae5e67e08724a5220d0e372fb080bbd2fbde9")
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
