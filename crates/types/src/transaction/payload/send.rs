use super::{Payload, Type};
use minicbor::{Decode, Encode};
use zarb_crypto::address::Address;

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct SendPayload {
    #[n(1)]
    sender: Address,
    #[n(2)]
    receiver: Address,
    #[n(3)]
    amount: i64,
}

impl Payload for SendPayload {
    fn signer(&self) -> &Address {
        &self.sender
    }
    fn value(&self) -> i64 {
        self.amount
    }
    fn payload_type(&self) -> Type {
        Type::Send
    }
    fn sanity_check(&self) -> crate::error::Result<()> {
        Ok(())
    }
    fn fingerprint(&self) -> String {
        "".to_string()
    }
}
