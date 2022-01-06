use crate::certificate::Certificate;
use minicbor::{Decode, Encode};
use zarb_crypto::address::Address;
use zarb_crypto::hash::Hash32;
use zarb_crypto::sortition::seed::Seed;

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
#[cbor(map)]
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
            "a301a80101021a61d57e49035820cc023e43c4bb7b111a7029ea224887537b4c70aa201054721d463091a4266698045820a3058dc8e31e3d908522287ef34b0fb8e7ae6e70cb376fca6bd113f74843ce3c05582007cf8efbba30d33a433937c9a88816187647b844e674cbee7231c8e3f7188a0d065820db5815dc702ea6ed3dd5e141dd90daac82c9b7ec7394e7b6f576cf65e843857c075830a54825bc7f77eac94e34203fe43a6d0222beea34224dcf6c86e9e55e479709fa3e53267860ba0fcc14dd911e8c0208ee085501b05ec5b163832ffe1b588470d49aff2a86be1f8e02a5015820cc023e43c4bb7b111a7029ea224887537b4c70aa201054721d463091a4266698020303840a120c10048112055830b7a82c851fd0b35a66683a1d4b0d7aac4f5b97d4c72e882e2b10c8b25ada75c1ab1c8f97ca2615f29b89bc8f25ddb6b903a101845820051ee8781a7352e5befc7183123ff1a6f436f6e3c5fdd4a6aaccdbaa4cec551a5820eb121430ecdb1dad019475b7c76d03a864f1b279ce9ae12ebdbf93eb8ea76e5c58202a6ae8b2c89d9f88b8ab7e7fa6db4058383d6de1aa7c8f8dca9c98fff8fe4f8958204c59bb0c74eb67c974aa8f69e856e2b6b02618e1e92b369efd97f761e05cdb70",
        )
        .unwrap()
        .to_vec();
        let blk = Block::from_bytes(buf1.as_slice()).unwrap();
        let buf2 = blk.to_bytes().unwrap();
        assert_eq!(buf1, buf2);
    }
}
