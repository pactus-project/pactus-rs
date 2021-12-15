use crate::error::{Error, Result};

const ADDRESS_SIZE: usize = 21;

pub struct Address {
    data: [u8; ADDRESS_SIZE],
}

impl TryFrom<&[u8]> for Address {
    type Error = Error;

    fn try_from(raw: &[u8]) -> Result<Self> {
        let data = raw.try_into().map_err(|_| Error::InvalidLength {
            expected: ADDRESS_SIZE,
            found: raw.len(),
        })?;
        Ok(Address { data })
    }
}
