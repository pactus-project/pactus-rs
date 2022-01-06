use crate::error::{Error, Result};

const HASH32_SIZE: usize = 32;

pub struct Hash32([u8; HASH32_SIZE]);

impl Hash32 {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; HASH32_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: HASH32_SIZE,
            found: data.len(),
        })?;
        Ok(Self(*bytes))
    }

    pub fn to_bytes(&self) -> [u8; HASH32_SIZE] {
        self.0
    }
}

crate::impl_cbor!(Hash32);
