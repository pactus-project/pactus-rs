use crate::error::{Error, Result};

const SEED_SIZE: usize = 48;
pub struct Seed([u8; SEED_SIZE]);

impl Seed {
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let data = buf.try_into().map_err(|_| Error::InvalidLength {
            expected: SEED_SIZE,
            found: buf.len(),
        })?;
        Ok(Self(data))
    }

    pub fn to_bytes(&self) -> [u8; SEED_SIZE] {
        self.0
    }
}

crate::impl_cbor!(Seed);
