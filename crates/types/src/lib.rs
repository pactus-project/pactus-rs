pub mod account;
pub mod address;
pub mod block;
pub mod certificate;
pub mod crypto;
pub mod error;
pub mod hash;
pub mod params;
pub mod sortition;
pub mod stamp;
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

macro_rules! impl_cbor {
    ($ty:ty) => {
        impl minicbor::Encode for $ty {
            fn encode<W>(
                &self,
                e: &mut minicbor::Encoder<W>,
            ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
            where
                W: minicbor::encode::Write,
            {
                e.bytes(self.as_bytes())?;
                Ok(())
            }
        }

        impl<'a> minicbor::Decode<'a> for $ty {
            fn decode(
                d: &mut minicbor::Decoder<'a>,
            ) -> core::result::Result<$ty, minicbor::decode::Error> {
                <$ty>::from_bytes(d.bytes()?)
                    .map_err(|_| minicbor::decode::Error::Message("invalid data"))
            }
        }
    };
}

pub(crate) use impl_cbor;
pub(crate) use impl_from_to_bytes;
