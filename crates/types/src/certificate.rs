use crate::crypto::bls::signature::BLSSignature;
use crate::hash::Hash32;
use minicbor::{Decode, Encode};

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct Certificate {
    #[n(1)]
    block_hash: Hash32,
    #[n(2)]
    round: i32,
    #[n(3)]
    committers: Vec<i32>,
    #[n(4)]
    absentees: Vec<i32>,
    #[n(5)]
    signature: BLSSignature,
}

impl Certificate {
    crate::impl_from_to_bytes!(Certificate);
}
