use crate::certificate::Certificate;
use crate::sortition::Seed;
use minicbor::{Decode, Encode};
use zarb_crypto::address::Address;
use zarb_crypto::hash::Hash32;

#[derive(Encode, Decode)]
pub struct Block {
    #[n(1)]
    header: BlockHeader,
    #[n(2)]
    prev_cert: Certificate,
    // tx_ids:
}

#[derive(Encode, Decode)]
pub struct BlockHeader {
    #[n(1)]
    version: i8,
    #[n(2)]
    unix_time: i64,
    #[n(3)]
    prev_block_hash: Hash32,
    #[n(4)]
    state_hash: Hash32,
    #[n(5)]
    tx_ids_hash: Hash32,
    #[n(6)]
    prev_cert_hash: Hash32,
    #[n(7)]
    sortition_seed: Seed,
    #[n(8)]
    proposer_address: Address,
}
