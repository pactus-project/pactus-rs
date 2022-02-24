pub mod heartbeat;
pub mod hello;

pub use crate::error::Result;
use core::fmt::Debug;
use minicbor::{Decode, Encode};
use std::{any::Any, fmt};

#[derive(Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[cbor(index_only)]
pub enum Type {
    #[n(1)]
    Hello,
    #[n(2)]
    Heartbeat,
    #[n(3)]
    QueryTransactions,
    #[n(4)]
    Transactions,
    #[n(5)]
    QueryProposal,
    #[n(6)]
    Proposal,
    #[n(7)]
    QueryVotes,
    #[n(8)]
    Vote,
    #[n(9)]
    BlockAnnounce,
    #[n(13)]
    BlockRequest,
    #[n(14)]
    BlockResponse,
}

pub trait Payload: Debug {
    fn sanity_check(&self) -> Result<()>;
    fn payload_type(&self) -> Type;
    fn to_bytes(&self) -> Result<Vec<u8>>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Hello => write!(f, "hello"),
            Type::Heartbeat => write!(f, "heartbeat"),
            Type::QueryTransactions => write!(f, "querytransactions"),
            Type::Transactions => write!(f, "transactions"),
            Type::QueryProposal => write!(f, "queryproposal"),
            Type::Proposal => write!(f, "proposal"),
            Type::QueryVotes => write!(f, "queryvotes"),
            Type::Vote => write!(f, "vote"),
            Type::BlockAnnounce => write!(f, "blockannounce"),
            Type::BlockRequest => write!(f, "blockrequest"),
            Type::BlockResponse => write!(f, "blockresponse"),
        }
    }
}

const ResponseCodeNone: i32 = -1;
const ResponseCodeOK: i32 = 0;
const ResponseCodeRejected: i32 = 1;
const ResponseCodeBusy: i32 = 2;
const ResponseCodeMoreBlocks: i32 = 3;
const ResponseCodeNoMoreBlocks: i32 = 4;
const ResponseCodeSynced: i32 = 5;
