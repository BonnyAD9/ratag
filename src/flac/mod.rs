mod metadata_block_header;
mod streaminfo;

use self::{metadata_block_header::*, streaminfo::*};

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
    time::Duration,
};

use crate::{
    DataType, Error, Result, TagRead, TagStore, bread::Bread, trap::Trap,
    vorbis,
};

/// TagReader for flac files.
#[derive(Debug)]
pub struct Flac;

impl<R: BufRead + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Flac {
    fn extensions(&self) -> &[&str] {
        &["flac"]
    }

    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

/// Read metadata from flac file.
pub fn from_file(
    f: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    from_read(BufReader::new(File::open(f)?), store, trap)
}

/// Read flac metadata from stream. Don't assume correct position within file.
pub fn from_seek(
    mut r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    r.rewind()?;
    from_read(r, store, trap)
}

/// Read flac metadata from stream. Assume that the position is correct.
pub fn from_read(
    r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let mut r = Bread::new(r);
    r.expect(b"fLaC").map_err(|_| Error::NoTag)?;

    let mut next = true;
    while next && !store.done() {
        let header: MetadataBlockHeader = r.get()?;
        next = !header.last;
        match header.block_type {
            MetadataBlockHeader::STREAMINFO
                if store.stores_data(DataType::Length) =>
            {
                let si: Streaminfo = r.get()?;
                let secs = si.sample_cnt as f64 / si.sample_rate as f64;
                store.set_length(Some(Duration::from_secs_f64(secs)));
            }
            MetadataBlockHeader::VORBISCOMMENT => {
                vorbis::from_bread(&mut r, store, trap, false)?;
            }
            _ => _ = r.seek_by(header.length as i64)?,
        }
    }

    Ok(())
}
