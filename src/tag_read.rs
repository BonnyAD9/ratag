use crate::{Result, TagFormat};

/// Generic tag format that can be read from file.
pub trait TagRead<R, S, T>: TagFormat {
    /// Read the tag from the given reader. The implementation must not assume
    /// that the reader is at the correct position within file and it should
    /// seek to the proper place within the file.
    ///
    /// # Errors
    /// - [`Error::NoTag`] if the given file doesn't contain this tag. In this
    ///   case, implementation must not write any data to `store`.
    /// - Other errors.
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()>;
}
