use std::{
    io::{Read, Seek},
    path::Path,
};

use crate::{Result, TagStore, trap::Trap};

mod id3v1;
mod id3v1_tag;

pub use self::{id3v1::*, id3v1_tag::*};

pub fn from_read(
    r: impl Read,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    Id3v1Tag::from_read(r, trap)?.store(store);
    Ok(())
}

pub fn from_seek(
    r: impl Read + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    Id3v1Tag::from_seek(r, trap)?.store(store);
    Ok(())
}

pub fn from_file(
    f: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    Id3v1Tag::from_file(f, trap)?.store(store);
    Ok(())
}
