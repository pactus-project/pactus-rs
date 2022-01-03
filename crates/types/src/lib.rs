pub mod account;
pub mod block;
pub mod certificate;
pub mod error;
pub mod tx;
pub mod validator;
pub mod params;

macro_rules! impl_from_to_bytes {
    ($ty:ty) => {
        pub fn from_bytes(buf: &[u8]) -> crate::error::Result<Self> {
            Ok(minicbor::decode(buf)?)
        }

        pub fn to_bytes(&self) -> crate::error::Result<Vec<u8>> {
            let mut buf = Vec::new();
            minicbor::encode(&self, &mut buf)?;
            Ok(buf)
        }
    };
}

pub(crate) use impl_from_to_bytes;
