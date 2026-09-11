use std::io::Write;

/// Trait that allows manipulating data in stream by writing to separate stream
/// and than swapping atomicaly.
pub trait AtomicSwap {
    /// Temporary writer.
    type TempWrite: Write;

    /// Create new temporary writer.
    fn temp_write(&self) -> Result<Self::TempWrite, std::io::Error>;

    /// Swap the contents of this stream to that of the given writer
    /// atomicaly.
    fn replace(&mut self, f: Self::TempWrite) -> Result<(), std::io::Error>;
}
