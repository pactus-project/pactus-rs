use minicbor::{decode, encode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("CBOR decoding error")]
    DecodeError(String),
    #[error("CBOR encoding error")]
    EncodeError(String),
    #[error("Invalid data length")]
    InvalidLength { expected: usize, found: usize },
    #[error("Invalid signature or public key")]
    InvalidSignatory,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<W: std::fmt::Display> From<encode::Error<W>> for Error {
    fn from(err: encode::Error<W>) -> Self {
        Error::EncodeError(format!("{}", err))
    }
}

impl From<decode::Error> for Error {
    fn from(err: decode::Error) -> Self {
        Error::DecodeError(format!("{}", err))
    }
}
