use std::io::{Read, Seek, Write};

use crate::{
    Result, TagFormat, TagRead, TagRemove, TagRetrieve, TagStore, TagType,
    WriteMode,
    id3::v1::{from_seek, remove, write_to},
    set_length::SetLength,
    tag_write::TagWrite,
    trap::Trap,
};

/// Tag reader for ID3v1.
#[derive(Debug)]
pub struct Id3v1;

impl TagFormat for Id3v1 {
    fn extensions(&self) -> &[&str] {
        &["mp3", "mpga", "bit"]
    }
}

impl<R: Read + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Id3v1 {
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

impl<W: Read + Seek + Write, R: TagRetrieve, T: Trap> TagWrite<W, R, T>
    for Id3v1
{
    fn tag_write(
        &self,
        w: &mut W,
        r: &R,
        mode: WriteMode,
        trap: &T,
    ) -> Result<crate::TagType> {
        write_to(w, r, mode, trap).map(TagType::Id3v1)
    }
}

impl<W: Read + Seek + SetLength, T> TagRemove<W, T> for Id3v1 {
    fn tag_remove(&self, w: &mut W, _: &T) -> Result<TagType> {
        remove(w).map(TagType::Id3v1)
    }
}
