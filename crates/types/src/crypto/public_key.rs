use crate::{address::Address, error::Result};
use blake2b_simd::Params;
use ripemd::{Digest, Ripemd160};
use std::fmt::Debug;

pub trait PublicKey: Debug {
    fn to_bytes(&self) -> Vec<u8>;
    fn sanity_check(&self) -> Result<()>;

    fn address(&self) -> Address {
        let digest256 = Params::new()
            .hash_length(32)
            .to_state()
            .update(&self.to_bytes())
            .finalize();

        let mut hasher = Ripemd160::new();
        hasher.update(digest256.as_bytes());
        let digest160 = hasher.finalize();
        let mut data = digest160.to_vec();
        data.insert(0, 1);
        Address::from_bytes(&data).unwrap()
    }
}
