use crate::certificate::Certificate;
use zarb_crypto::sortition::seed::Seed;
use minicbor::{Decode, Encode};
use zarb_crypto::address::Address;
use zarb_crypto::hash::Hash32;

#[derive(Encode, Decode)]
#[cbor(map)]
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

#[derive(Encode, Decode)]
pub struct TxIDs {
    #[n(1)]
    ids: Vec<Hash32>,
}

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct Block {
    #[n(1)]
    header: BlockHeader,
    #[n(2)]
    prev_cert: Certificate,
    #[n(3)]
    tx_ids: TxIDs,
}

impl Block {
    crate::impl_from_to_bytes!(Block);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
            "a301a80101021a6077b9fe035820d846ef49a6c72390645f12970987865a795a55fa19c92dbb9cbe24d6503eca9f0458208ae0a4883808290510bb77678bb24a2527d22d7dcf2d5d605ea57595260bfdf00558208e442e0f18a7797d7c289ead53b7c02d9f77147003bebbf7b0572a72fb004bbb06582085c4963c28750eef54ba1b14dd03fc85dbe482a280d06e0eefb427fcb15b616c075830db66ddce5cd16ec9710294769c7386977e48eef2bc38c5a93b49ea06ac9fa8fc502976397abc00c5df21d2d1c757d80d08543bfc7df5c9915c56e399fbe47be7d25aeff238b302a5015820d846ef49a6c72390645f12970987865a795a55fa19c92dbb9cbe24d6503eca9f020603840a12020604810a05583085c368e9e6df4ea1b16e29aebbf74a3da45a033683e753c93130336e035c2181bf469dab5e0448064fb64f6282b2829603a10184582005d5455c116d98a90c452365e7e9a4cd03847bf7ba0eabab4cd8acaa417a47125820f1c6fd7464bb6d57c3b4b56995bfbb864b64ec82c8e77b454449637b5be98ff558203de71e737f5aa31d148db8a9d29be3662584276edd3530c92643d220997fcf405820b65b38afd18345db104a11ac06ac6fa692abd808f7f7615f69182ad636b1ea08",
        )
        .unwrap()
        .to_vec();
        let blk = Block::from_bytes(buf1.as_slice()).unwrap();
        let buf2 = blk.to_bytes().unwrap();
        assert_eq!(buf1, buf2);
    }
}
