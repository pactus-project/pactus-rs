use crate::error::Result;
use minicbor::{Decode, Encode};
use zarb_crypto::bls::public_key::PublicKey;

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct Validator {
    #[n(1)]
    public_key: PublicKey,
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
    pub fn from_bytes(buf: &[u8]) -> Result<Self> {
        Ok(minicbor::decode(buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
                    //"a7015860dff46fbce5ae1ba4837de551206176c0a74deb5dfca803228f570f7c9ba093ea109700559b72fe1d385492f0d5a10f17a4cec41eb2e552f51e1f7f48ab311d4e195b1563c1fcba8ee201173e4e6362cabedaccee541f9efc9c4140d9fb268102021901b4031902f7041a2af78f2105140618640700",
                 "a7015860af0f74917f5065af94727ae9541b0ddcfb5b828a9e016b02498f477ed37fb44d5d882495afb6fd4f9773e4ea9deee436030c4d61c6e3a1151585e1d838cae1444a438d089ce77e10c492a55f6908125c5be9b236a246e4082d08de564e111e65021901b4031902f7041a2af78f2105140618640700",
        )
        .unwrap()
        .to_vec();
        let val = Validator::from_bytes(buf1.as_slice()).unwrap();
        let mut buf2 = Vec::new();
        minicbor::encode(&val, &mut buf2).unwrap();
        assert_eq!(buf1, buf2);
    }
}
