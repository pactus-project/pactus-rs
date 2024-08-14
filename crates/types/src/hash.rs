use crate::error::{Error, Result};
use blake2b_simd::Params;

const HASH32_SIZE: usize = 32;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hash32([u8; HASH32_SIZE]);

impl Hash32 {
    pub fn calculate(data: &[u8]) -> Self {
        let digest = Params::new()
            .hash_length(32)
            .to_state()
            .update(data)
            .finalize();

        Hash32::from_bytes(digest.as_bytes()).unwrap()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; HASH32_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: HASH32_SIZE,
            found: data.len(),
        })?;
        Ok(Self(*bytes))
    }

    pub fn as_bytes(&self) -> &[u8; HASH32_SIZE] {
        &self.0
    }
}

crate::impl_cbor!(Hash32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        assert!(Hash32::from_bytes(&[]).is_err());
    }

    #[test]
    fn test_calc() {
        let buf = hex::decode("12b38977f2d67f06f0c0cd54aaf7324cf4fee184398ea33d295e8d1543c2ee1a")
            .unwrap();
        assert_eq!(Hash32::calculate("pactus".as_bytes()).0.to_vec(), buf.to_vec());
    }
}
