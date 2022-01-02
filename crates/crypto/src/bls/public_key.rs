use crate::error::{Error, Result};
use bls12_381_plus::{G2Affine, G2Projective};
use group::Curve;

const PUBLIC_KEY_SIZE: usize = 96;

#[derive(Debug, PartialEq, Eq)]
pub struct PublicKey {
    pub(super) key: G2Projective,
}

impl crate::public_key::PublicKey for PublicKey {}

impl PublicKey {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; PUBLIC_KEY_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: PUBLIC_KEY_SIZE,
            found: data.len(),
        })?;
        let key_opt = G2Affine::from_compressed(bytes);
        Ok(PublicKey {
            key: G2Projective::from(&key_opt.unwrap()),
        })
    }

    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_SIZE] {
        self.key.to_affine().to_compressed()
    }
}

impl minicbor::Encode for PublicKey {
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

impl<'a> minicbor::Decode<'a> for PublicKey {
    fn decode(
        d: &mut minicbor::Decoder<'a>,
    ) -> core::result::Result<PublicKey, minicbor::decode::Error> {
        Ok(PublicKey::from_bytes(d.bytes()?)
            .map_err(|_| minicbor::decode::Error::Message("error"))?)
    }
}
