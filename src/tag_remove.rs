use crate::{Result, TagFormat, TagType};

/// Trait for tags that can remouve themself from a stream.
pub trait TagRemove<W, T>: TagFormat {
    /// Remove the tag from the stream.
    ///
    /// Returns the tag that was removed. If there is no tag, returns
    /// [`crate::Error::NoTag`].
    fn tag_remove(&self, w: &mut W, trap: &T) -> Result<TagType>;
}
