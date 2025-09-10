mod metadata_block_header;
mod streaminfo;

use encoding::{
    Encoding,
    all::{ASCII, UTF_8},
};

use self::{metadata_block_header::*, streaminfo::*};

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
    time::Duration,
};

use crate::{
    DataType, Error, Picture, PictureKind, Result, TagRead, TagStore,
    bread::Bread, trap::Trap, vorbis,
};

// Implementation is based on: https://www.rfc-editor.org/rfc/rfc9639.html

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
    if !r.expect(b"fLaC")? {
        return Err(Error::NoTag);
    }

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
                store.set_length(Duration::from_secs_f64(secs));
            }
            MetadataBlockHeader::VORBISCOMMENT => {
                vorbis::from_bread(&mut r, store, trap, false)?;
            }
            MetadataBlockHeader::PICTURE => {
                read_picture(&mut r, store, trap, header.length as i64)?;
            }
            _ => r.seek_by(header.length as i64)?,
        }
    }

    Ok(())
}

fn read_picture(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    mut len: i64,
) -> Result<()> {
    let typ: u32 = r.get_be()?;
    len -= 4;
    let kind = if typ < u8::MAX as u32
        && let Some(pk) = PictureKind::from_id3(typ as u8)
    {
        pk
    } else {
        trap.error(Error::InvalidPictureKind)?;
        PictureKind::default()
    };

    if !store.stores_data(DataType::Picture(kind)) {
        r.seek_by(len)?;
        return Ok(());
    }

    let mut l: u32 = r.get_be()?;
    len -= 4;
    let mime = r.witht(l as usize, trap, |d, t| {
        ASCII
            .decode(d, t.decoder_trap())
            .map_err(|_| Error::InvalidEncoding)
    })?;
    len -= l as i64;

    let is_uri = mime.as_deref() == Some("-->");

    l = r.get_be()?;
    len -= 4;
    let description = r.witht(l as usize, trap, |d, t| {
        UTF_8
            .decode(d, t.decoder_trap())
            .map_err(|_| Error::InvalidEncoding)
    })?;
    len -= l as i64;

    let width: u32 = r.get_be()?;
    let height: u32 = r.get_be()?;
    let color_depth: u32 = r.get_be()?;
    len -= 12;

    let palette_size: u32 = r.get_be()?;
    let palette_size = if palette_size == 0 {
        None
    } else {
        Some(palette_size)
    };
    len -= 4;

    l = r.get_be()?;
    len -= 4;
    let data = r.read_exact_owned(l as usize)?;
    len -= l as i64;

    store.add_picture(Picture {
        mime,
        description,
        kind,
        is_uri,
        size: Some((width as usize, height as usize)),
        color_depth: Some(color_depth),
        palette_size: palette_size.map(|a| a as usize),
        data,
    });

    r.seek_by(len)?;

    Ok(())
}
