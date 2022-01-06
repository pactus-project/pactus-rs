use minicbor::{Decode, Encode};
use zarb_crypto::bls::signature::Signature;
use zarb_crypto::hash::Hash32;

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
    signature: Signature,
}
