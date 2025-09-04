use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No tag.")]
    NoTag,
    #[error("Invalid encoding.")]
    InvalidEncoding,
    #[error("Invalid length of data.")]
    InvalidLength,
    #[error("Invalid id3v1 genre reference.")]
    InvalidGenreRef,
    #[error("Invalid digit in numeric string.")]
    InvalidDigit,
    #[error("String is not properly terminated.")]
    StringNotTerminated,
    #[error("Not supported: {0}.")]
    Unsupported(&'static str),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
