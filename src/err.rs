use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No tag.")]
    NoTag,
    #[error("Invalid length of data.")]
    InvalidLength,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
