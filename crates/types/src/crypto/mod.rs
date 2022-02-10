pub mod bls;
pub mod public_key;
pub mod secret_key;
pub mod signatory;
pub mod signature;
pub mod signer;



macro_rules! impl_common {
    ($ty:ty) => {
        impl $ty {
            pub fn from_string(hex: &str) -> Result<Self> {
                let data = hex::decode(hex)?;
                Self::from_bytes(&data)
            }

            pub fn to_string(&self) -> String {
                hex::encode(self.to_bytes())
            }
        }

        impl minicbor::Encode for $ty {
            fn encode<W>(
                &self,
                e: &mut minicbor::Encoder<W>,
            ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
            where
                W: minicbor::encode::Write,
            {
                e.bytes(&self.to_bytes())?;
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

pub(super) use impl_common;
