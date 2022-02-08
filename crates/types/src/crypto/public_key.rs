use crate::{address::Address, error::Result};
use std::fmt::Debug;

pub trait PublicKey: Debug {
    fn address(&self) -> Address;
    fn to_bytes(&self) -> Vec<u8>;
    fn sanity_check(&self) -> Result<()>;
}
