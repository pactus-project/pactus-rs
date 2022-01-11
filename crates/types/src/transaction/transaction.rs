use zarb_crypto::hash::Hash32;
use zarb_crypto::public_key::PublicKey;
use zarb_crypto::signature::Signature;

pub struct Transaction<'de, S, P>
where
    S: Signature<'de>,
    P: PublicKey<'de>,
{
    pub version: i32,
    pub stamp: Hash32,
    pub sequence: i32,
    pub fee: i64,
    pub payload_type: i32,
    pub payload: Vec<u8>,
    pub memo: String,
    pub public_key: Option<&'de P>,
    pub signature: Option<&'de S>,
}
