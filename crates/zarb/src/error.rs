use minicbor::{decode, encode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("decoding error: {0}")]
    DecodeError(String),
    #[error("encoding error: {0}")]
    EncodeError(String),
    #[error("network Error: {0}")]
    NetworkError(String),
    #[error("invalid message: {0}")]
    InvalidMessage(String),
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
