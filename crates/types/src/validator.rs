use minicbor::{Decode, Encode};
use crate::crypto::bls::public_key::BLSPublicKey;

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct Validator {
    #[n(1)]
    public_key: BLSPublicKey,
    #[n(2)]
    number: i32,
    #[n(3)]
    sequence: i32,
    #[n(4)]
    stake: i64,
    #[n(5)]
    last_bonding_height: i32,
    #[n(6)]
    unbonding_height: i32,
    #[n(7)]
    last_joined_height: i32,
}

impl Validator {
    crate::impl_from_to_bytes!(Validator);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
                 "a7015860af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65021901b4031902f7041a2af78f2105140618640700",
        )
        .unwrap()
        .to_vec();
        let val = Validator::from_bytes(buf1.as_slice()).unwrap();
        let buf2 = val.to_bytes().unwrap();
        assert_eq!(buf1, buf2);
    }
}
