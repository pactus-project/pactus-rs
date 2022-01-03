pub mod public_key;
pub mod secret_key;
pub mod signature;

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
                e.bytes(&self.to_bytes())?;
                Ok(())
            }
        }

        impl<'a> minicbor::Decode<'a> for $ty {
            fn decode(
                d: &mut minicbor::Decoder<'a>,
            ) -> core::result::Result<$ty, minicbor::decode::Error> {
                let data = d.bytes()?.try_into().map_err(|_| {
                    minicbor::decode::Error::Message(
                        "byte slice length does not match expected length",
                    )
                })?;
                <$ty>::from_bytes(data)
                    .map_err(|_| minicbor::decode::Error::Message("invalid signature data"))
            }
        }
    };
}

pub(crate) use impl_cbor;
