use crate::error::{Error, Result};

const ADDRESS_SIZE: usize = 21;

pub struct Address([u8; ADDRESS_SIZE]);


impl minicbor::Encode for Address {
    fn encode<W>(
        &self,
        e: &mut minicbor::Encoder<W>,
    ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
    where
        W: minicbor::encode::Write,
    {
        e.bytes(&self.0)?;
        Ok(())
    }
}

impl<'a> minicbor::Decode<'a> for Address {
    fn decode(
        d: &mut minicbor::Decoder<'a>,
    ) -> core::result::Result<Address, minicbor::decode::Error> {
        let data = d.bytes()?.try_into().map_err(|_| {
            minicbor::decode::Error::Message("byte slice length does not match expected length")
        })?;
        Ok(Address (data))
    }
}

// impl TryFrom<&[u8]> for Address {
//     type Error = Error;

//     fn try_from(buf: &[u8]) -> Result<Self> {
//         let data = buf.try_into().map_err(|_| Error::InvalidLength {
//             expected: ADDRESS_SIZE,
//             found: buf.len(),
//         })?;
//         Ok(Address { data })
//     }
// }
