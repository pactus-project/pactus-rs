pub mod aleyk;
pub mod heartbeat;
pub mod salam;

pub use crate::error::Result;
use core::fmt::Debug;
use minicbor::{Decode, Encode};

#[derive(Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[cbor(index_only)]
pub enum Type {
    #[n(1)]
    Salam,
    #[n(2)]
    Aleyk,
    #[n(3)]
    LatestBlocksRequest,
    #[n(4)]
    LatestBlocksResponse,
    #[n(5)]
    QueryTransactions,
    #[n(6)]
    Transactions,
    #[n(7)]
    QueryProposal,
    #[n(8)]
    Proposal,
    #[n(9)]
    Heartbeat,
    #[n(10)]
    QueryVotes,
    #[n(11)]
    Vote,
    #[n(12)]
    BlockAnnounce,
    #[n(13)]
    DownloadRequest,
    #[n(14)]
    DownloadResponse,
}

pub trait Payload: Debug {
    fn sanity_check(&self) -> Result<()>;
    fn payload_type(&self) -> Type;
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

const ResponseCodeNone: i32 = -1;
const ResponseCodeOK: i32 = 0;
const ResponseCodeRejected: i32 = 1;
const ResponseCodeBusy: i32 = 2;
const ResponseCodeMoreBlocks: i32 = 3;
const ResponseCodeNoMoreBlocks: i32 = 4;
const ResponseCodeSynced: i32 = 5;
