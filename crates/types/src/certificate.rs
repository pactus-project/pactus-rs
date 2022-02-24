use crate::crypto::bls::signature::Signature;
use crate::hash::Hash32;
use minicbor::{Decode, Encode, bytes::ByteVec};

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
    signature_data: ByteVec,
}

impl Certificate {
    pub fn signature(&self) -> Signature {
        todo!()
    }

    crate::impl_from_to_bytes!(Certificate);
}
