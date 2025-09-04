use crate::Result;

/// Generic tag format that can be read from file.
pub trait TagRead<R, S, T> {
    /// Extensions that are usually asociated with this format. This format
    /// will be prioritized for files with one of these extensions.
    fn extensions(&self) -> &[&str];

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
