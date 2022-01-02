use super::public_key::PublicKey;
use crate::error::{Error, Result};
use bls12_381_plus::{Scalar, G2Projective};

const SECRET_KEY_SIZE: usize = 32;

#[derive(Debug, PartialEq, Eq)]
pub struct SecretKey {
    key: Scalar,
}

impl crate::secret_key::SecretKey for SecretKey {
    type PublicKey = super::public_key::PublicKey;


    fn public_key(&self) -> PublicKey {
        PublicKey {
            key: G2Projective::generator() * self.key,
        }
    }
}

impl SecretKey {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; SECRET_KEY_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: SECRET_KEY_SIZE,
            found: data.len(),
        })?;
        let key_opt = Scalar::from_bytes(bytes);
        Ok(SecretKey {
            key: key_opt.unwrap(),
        })
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_SIZE] {
        self.key.to_bytes()
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
        Ok(SecretKey::from_bytes(d.bytes()?).map_err(|_| {
            minicbor::decode::Error::Message("error")
        })?)
    }
}
#[cfg(test)]
mod tests {
    use crate::secret_key::SecretKey;

    #[test]
    fn test_decoding() {
        // let buf1 =
        //     hex!("d0c6a560de2e60b6ac55386defefdf93b0c907290c2ad1b4dbd3338186bfdc68").to_vec();
        let buf1 =
            hex::decode("d0c6a560de2e60b6ac55386defefdf93b0c907290c2ad1b4dbd3338186bfdc68").unwrap().to_vec();

        let sec_key = super::SecretKey::from_bytes(buf1.as_slice()).unwrap();
        let pub_key = sec_key.public_key();
        println!("{}", hex::encode(pub_key.to_bytes()));
    }
}


// af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65







