use thiserror::Error;

/// Result type of the ratag crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type of the ratag crate.
#[derive(Debug, Error)]
pub enum Error {
    /// There is no tag.
    #[error("No tag.")]
    NoTag,
    /// Encoding is invalid. This is usually recoverable.
    #[error("Invalid encoding.")]
    InvalidEncoding,
    /// Expected different length of data.
    #[error("Invalid length of data.")]
    InvalidLength,
    /// Invalid genre reference in id3. This is usually recoverable.
    #[error("Invalid id3v1 genre reference.")]
    InvalidGenreRef,
    /// Invalid picture kind.
    #[error("Invalid picture kind.")]
    InvalidPictureKind,
    /// String is nor properly terminated. This is usually recoverable.
    #[error("String is not properly terminated.")]
    StringNotTerminated,
    /// Invalid vorbis comment. Missing `=` in comment.
    #[error("Invalid vorbis comment. (no `=` in comment)")]
    InvalidVorbisComment,
    /// Vorbis framing bit is not set.
    #[error("Vorbis framing bit is not set.")]
    InvalidVorbisFramingBit,
    /// Invalid data type.
    #[error("Invalid data type.")]
    InvalidDataType,
    /// Invalid time format.
    #[error("Invalid date format.")]
    InvalidTime,
    /// Invalid date format.
    #[error("Invalid date format.")]
    InvalidDate,
    /// Missing BOM.
    #[error("Missing BOM.")]
    MissingBom,
    /// Feature required to parse the tag is not supported.
    #[error("Not supported: {0}")]
    Unsupported(&'static str),
    /// Failed to parse number.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    /// Any IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Failed to convert slice to array.
    #[error(transparent)]
    TryFromSlice(#[from] std::array::TryFromSliceError),
    /// Failed to convert integer types.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}
