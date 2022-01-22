use crate::error::Result;

pub trait PublicKey {
    fn to_bytes(&self) -> Vec<u8>;
    fn sanity_check(&self) -> Result<()>;
}
