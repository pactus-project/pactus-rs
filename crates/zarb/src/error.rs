use minicbor::{decode, encode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("CBOR decoding error")]
    DecodeError(String),
    #[error("CBOR encoding error")]
    EncodeError(String),
    #[error("io Error")]
    IOError(#[from] std::io::Error),
    #[error("LibP2P multiaddr error")]
    LibP2PMultiaddr(#[from] libp2p::multiaddr::Error),
    #[error("LibP2P transport error")]
    LibP2PTransportError(#[from] libp2p::TransportError<std::io::Error>)
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
