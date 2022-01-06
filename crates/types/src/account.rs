use minicbor::{Decode, Encode};
use zarb_crypto::address::Address;

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct Account {
    #[n(1)]
    pub address: Address,
    #[n(2)]
    pub number: i32,
    #[n(3)]
    pub sequence: i32,
    #[n(4)]
    pub balance: i64,
}

impl Account {
    crate::impl_from_to_bytes!(Account);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
            "a40155010c9819c4d4b1edb7b70e6665287d4ce95401a37702191bd7031823041a007f5535",
        )
        .unwrap()
        .to_vec();
        let acc = Account::from_bytes(buf1.as_slice()).unwrap();
        let mut buf2 = Vec::new();
        minicbor::encode(&acc, &mut buf2).unwrap();
        assert_eq!(buf1, buf2);
    }
}
