use std::io::{BufRead, Seek};

use crate::{TagFormat, TagRead, TagStore, id3::v2::from_seek, trap::Trap};

/// Tag reader for ID3v2.
#[derive(Debug)]
pub struct Id3v2;

impl TagFormat for Id3v2 {
    fn extensions(&self) -> &[&str] {
        &["mp3", "mpga"]
    }
}

impl<R: BufRead + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Id3v2 {
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> crate::Result<()> {
        from_seek(r, store, trap)
    }
}
