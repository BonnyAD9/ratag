mod genres;
/// Module for reading ID3v1 tags.
pub mod v1;
/// Module for reading ID3v2 tags.
pub mod v2;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use crate::{Error, Result, TagRead, TagStore, trap::Trap};

use self::genres::*;

/// Read data from both ID3v1 and ID3v2. ID3v2 is prioritized.
#[derive(Debug)]
pub struct Id3;

impl<R: BufRead + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Id3 {
    fn extensions(&self) -> &[&str] {
        &["mp3"]
    }

    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

/// Read ID3 tags. This will seek to the proper positions within the file.
/// Data from ID3v2 is prioritized.
pub fn from_seek<R: BufRead + Seek>(
    r: &mut R,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let v1_res = { v1::from_seek(&mut *r, store, trap) };
    let v2_res = v2::from_seek(&mut *r, store, trap);
    match (v1_res, v2_res) {
        (_, Ok(_)) => Ok(()),
        (Err(Error::NoTag), e) => e,
        (e, Err(Error::NoTag)) => e,
        (_, e) => e,
    }
}

/// Read ID3 tags from file. Data from ID3v2 is prioritized.
pub fn from_file(
    path: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    from_seek(&mut BufReader::new(File::open(path)?), store, trap)
}
