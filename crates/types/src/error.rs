use minicbor::{decode, encode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid data length")]
    InvalidLength { expected: usize, found: usize },
    #[error("CBOR decoding error")]
    DecodeError(#[from] decode::Error),
    //#[error("CBOR decoding error")]
    //EncodeError(#[from] encode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
