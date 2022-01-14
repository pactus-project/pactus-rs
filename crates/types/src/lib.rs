pub mod account;
pub mod block;
pub mod certificate;
pub mod error;
pub mod params;
pub mod transaction;
pub mod validator;

macro_rules! impl_from_to_bytes {
    ($ty:ty) => {
        pub fn from_bytes(buf: &[u8]) -> crate::error::Result<Self> {
            Ok(minicbor::decode(buf)?)
        }

        pub fn to_bytes(&self) -> crate::error::Result<Vec<u8>> {
            Ok(minicbor::to_vec(&self)?)
        }
    };
}

pub(crate) use impl_from_to_bytes;
