use crate::{Result, TagFormat, TagType, WriteMode};

/// The opposite of [`crate::TagRead`]. Tag for traits that can be written.
pub trait TagWrite<W, R, T>: TagFormat {
    /// Write the tag to the given file with the given mode.
    fn tag_write(
        &self,
        w: &mut W,
        r: &R,
        mode: WriteMode,
        trap: &T,
    ) -> Result<TagType>;
}
