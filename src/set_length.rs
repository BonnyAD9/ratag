use std::fs::File;

/// Trait for stream which can be truncated or extended with zeros.
pub trait SetLength {
    /// Set new length of the steam.
    ///
    /// If `len` is less than the current length, it is truncated, if it is
    /// more it is appended with zeros.
    ///
    /// This doesn't change position of the cursor. It may stay behind the
    /// stream end.
    fn set_length(&self, len: u64) -> Result<(), std::io::Error>;
}

impl SetLength for File {
    fn set_length(&self, len: u64) -> Result<(), std::io::Error> {
        self.set_len(len)
    }
}
