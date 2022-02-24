use minicbor::{decode, encode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Decoding error")]
    DecodeError(String),
    #[error("Encoding error")]
    EncodeError(String),
    #[error("Invalid data length")]
    InvalidLength { expected: usize, found: usize },
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

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::DecodeError(format!("{}", err))
    }
}


