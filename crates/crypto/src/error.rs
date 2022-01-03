use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid data length")]
    InvalidLength { expected: usize, found: usize },
}

pub type Result<T> = std::result::Result<T, Error>;
