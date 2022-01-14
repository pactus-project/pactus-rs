pub mod send;

use crate::error::{Error, Result};
use minicbor::{Decode, Encode};
use zarb_crypto::address::Address;

#[derive(Debug, Clone, Decode, Encode, Eq, PartialEq)]
#[cbor(index_only)]
pub enum Type {
    #[n(1)]
    Send,
    #[n(2)]
    Bond,
    #[n(3)]
    Sortition,
    #[n(4)]
    Unbond,
    #[n(5)]
    Withdraw,
}

// impl TryFrom<i8> for Type {
//     type Error = Error;
//     fn try_from(code: i8) -> Result<Self> {
//         match code {
//             1 => Ok(Self::Send),
//             2 => Ok(Self::Bond),
//             3 => Ok(Self::Sortition),
//             4 => Ok(Self::Unbond),
//             5 => Ok(Self::Withdraw),
//             _ => Err(Error::InvalidPayload(code)),
//         }
//     }
// }

pub trait Payload {
    fn to_bytes(&self) -> Result<Vec<u8>>;
    fn signer(&self) -> &Address;
    fn value(&self) -> i64;
    fn payload_type(&self) -> Type;
    fn sanity_check(&self) -> Result<()>;
    fn fingerprint(&self) -> String;
}
