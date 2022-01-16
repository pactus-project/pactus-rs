use crate::error::{Error, Result};

const PROOF_SIZE: usize = 48;
pub struct Proof([u8; PROOF_SIZE]);

impl Proof {
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let data = buf.try_into().map_err(|_| Error::InvalidLength {
            expected: PROOF_SIZE,
            found: buf.len(),
        })?;
        Ok(Self(data))
    }

    pub fn as_bytes(&self) -> &[u8; PROOF_SIZE] {
        &self.0
    }
}

crate::impl_cbor!(Proof);
