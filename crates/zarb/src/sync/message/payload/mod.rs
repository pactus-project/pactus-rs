pub mod salam;

pub use crate::error::Result;
use core::fmt::Debug;
use minicbor::{Decode, Encode};

#[derive(Debug, Decode, Encode)]
#[cbor(index_only)]
pub enum PayloadType {
    #[n(1)]
    Salam,
}

pub trait Payload: Debug {
    fn sanity_check(&self) -> Result<()>;
    fn payload_type(&self) -> PayloadType;
    fn to_bytes(&self) -> Result<Vec<u8>>;
}
