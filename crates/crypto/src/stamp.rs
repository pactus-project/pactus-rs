use crate::error::{Error, Result};

const STAMP_SIZE: usize = 4;

pub struct Stamp([u8; STAMP_SIZE]);

impl Stamp {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let bytes: &[u8; STAMP_SIZE] = data.try_into().map_err(|_| Error::InvalidLength {
            expected: STAMP_SIZE,
            found: data.len(),
        })?;
        Ok(Self(*bytes))
    }

    pub fn to_bytes(&self) -> [u8; STAMP_SIZE] {
        self.0
    }
}

crate::impl_cbor!(Stamp);
