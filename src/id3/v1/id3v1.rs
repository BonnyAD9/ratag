use std::io::{Read, Seek};

use crate::{Result, TagRead, TagStore, id3::v1::from_seek, trap::Trap};

#[derive(Debug)]
pub struct Id3v1;

impl<R: Read + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Id3v1 {
    fn extensions(&self) -> &[&str] {
        &["mp3"]
    }

    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}
