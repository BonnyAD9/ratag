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
    /// Invalid digit when parsing string to number. This is usually
    /// recoverable.
    #[error("Invalid digit in numeric string.")]
    InvalidDigit,
    /// String is nor properly terminated. This is usually recoverable.
    #[error("String is not properly terminated.")]
    StringNotTerminated,
    /// Invalid vorbis comment. Missing `=` in comment.
    #[error("Invalid vorbis comment. (no `=` in comment)")]
    InvalidVorbisComment,
    /// Vorbis framing bit is not set.
    #[error("Vorbis framing bit is not set.")]
    InvalidVorbisFramingBit,
    /// Invalid date format.
    #[error("Invalid date format.")]
    InvalidDate,
    /// Feature required to parse the tag is not supported.
    #[error("Not supported: {0}.")]
    Unsupported(&'static str),
    /// Any IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Failed to parse date/time
    #[error(transparent)]
    DateTime(#[from] time::error::Parse),
}
