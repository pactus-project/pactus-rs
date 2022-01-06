use crate::error::{Error, Result};

const ADDRESS_SIZE: usize = 21;

pub struct Address([u8; ADDRESS_SIZE]);

impl Address {
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        let data = buf.try_into().map_err(|_| Error::InvalidLength {
            expected: ADDRESS_SIZE,
            found: buf.len(),
        })?;
        Ok(Self(data))
    }

    pub fn to_bytes(&self) -> [u8; ADDRESS_SIZE] {
        self.0
    }
}

crate::impl_cbor!(Address);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        assert!(Address::from_bytes(&[]).is_err());
    }
}
