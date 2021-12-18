use crate::error::{Error, Result};
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

impl TryFrom<&[u8]> for Account {
    type Error = Error;

    fn try_from(buf: &[u8]) -> Result<Self> {
        Ok(minicbor::decode(buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_decoding() {
        let buf1 =
            hex!("A40155010C9819C4D4B1EDB7B70E6665287D4CE95401A37702191BD7031823041A007F5535")
                .to_vec();
        let acc = Account::try_from(buf1.as_slice()).unwrap();
        let mut buf2 = Vec::new();
        minicbor::encode(&acc, &mut buf2).unwrap();
        assert_eq!(buf1, buf2);
    }
}
