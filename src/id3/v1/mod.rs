use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{
    Error, Result, TagRetrieve, TagStore, WriteMode, WriteModeKind,
    set_length::SetLength, trap::Trap,
};

mod id3v1;
mod id3v1_tag;
mod part_writer;

pub use self::{id3v1::*, id3v1_tag::*};

// Implementation is based on:
// - https://id3.org/ID3v1
// - https://www.birdcagesoft.com/ID3v12.txt

/// Read ID3v1 tag from reader, assuming that it is already at the correct
/// position.
pub fn from_read(
    r: impl Read,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    Id3v1Tag::from_read(r, trap)?.store(store, trap)?;
    Ok(())
}

/// Read ID3v1 tag from reader without assuming that it is already at the
/// correct position.
pub fn from_seek(
    r: impl Read + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    Id3v1Tag::from_seek(r, trap)?.store(store, trap)?;
    Ok(())
}

/// Read ID3v1 tag from file.
pub fn from_file(
    f: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    Id3v1Tag::from_file(f, trap)?.store(store, trap)?;
    Ok(())
}

/// Write the ID3v1 tag to the stream.
///
/// Doesn't try to read the stream contents or seek within it.
pub fn raw_write_to(
    w: impl Write,
    r: &impl TagRetrieve,
    trap: &impl Trap,
) -> Result<u8> {
    let mut t = Id3v1Tag::default();
    t.retrieve(r);
    t.write_to(w, None, trap)
}

/// Write the tag to the given stream according to the mode.
///
/// This will seek and read from the stream to detect presence of this tag and
/// updates it according to the mode.
///
/// The version of the tag is chosen automatically (usually same or greater
/// than the existing version).
pub fn write_to<R: Read + Write + Seek>(
    w: &mut R,
    r: &impl TagRetrieve,
    mode: WriteMode,
    trap: &impl Trap,
) -> Result<u8> {
    let (tag, mut ver) = prepare_mode_write(w, r, mode, trap)?;
    if ver.is_some_and(|v| v < 2) {
        ver = None;
    }
    tag.write_to(&mut *w, ver, trap)
}

/// Write the tag to the given file according to the mode.
///
/// This will seek and read from the file to detect presence of this tag and
/// updates it according to the mode.
///
/// The version of the tag is chosen automatically (usually same or greater
/// than the existing version).
pub fn write_to_file(
    path: impl AsRef<Path>,
    r: &impl TagRetrieve,
    mode: WriteMode,
    trap: &impl Trap,
) -> Result<u8> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    write_to(&mut f, r, mode, trap)
}

/// Write the tag to the given stream according to the mode and with the given
/// version.
///
/// This will seek and read from the stream to detect presence of this tag and
/// updates it according to the mode.
pub fn write_version_to<R: Read + Write + Seek + SetLength>(
    w: &mut R,
    r: &impl TagRetrieve,
    mode: WriteMode,
    trap: &impl Trap,
    version: u8,
) -> Result<()> {
    if version > 2 {
        return Err(Error::InvalidVersion);
    }

    let (tag, ver) = prepare_mode_write(w, r, mode, trap)?;
    tag.write_to(&mut *w, Some(version), trap)?;

    if version < 2 && ver.is_some_and(|v| v >= 2) {
        let pos = w.stream_position()?;
        w.set_length(pos)?;
    }

    Ok(())
}

/// Write the tag to the given file according to the mode and with the given
/// version.
///
/// This will seek and read from the file to detect presence of this tag and
/// updates it according to the mode.
pub fn write_version_to_file(
    path: impl AsRef<Path>,
    r: &impl TagRetrieve,
    mode: WriteMode,
    trap: &impl Trap,
    version: u8,
) -> Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    write_version_to(&mut f, r, mode, trap, version)
}

/// Migrate ID3v1 tag within the stream to the given version.
///
/// If there is no ID3v1 tag, if `new` is set to true the tag will be created,
/// otherwise [`Error::NoTag`] is produced.
pub fn migrate_version<R: Read + Write + Seek + SetLength>(
    w: &mut R,
    new: bool,
    trap: &impl Trap,
    version: u8,
) -> Result<()> {
    if version > 2 {
        return Err(Error::InvalidVersion);
    }

    let (tag, ver) = read_and_seek_to_start(&mut *w, trap)?;
    if ver.is_none() && !new {
        return Err(Error::NoTag);
    }
    if ver.is_none_or(|v| v != version) {
        tag.write_to(&mut *w, Some(version), trap)?;
    }

    if version < 2 && ver.is_some_and(|v| v >= 2) {
        let pos = w.stream_position()?;
        w.set_length(pos)?;
    }

    Ok(())
}

/// Migrate ID3v1 tag within the file to the given version.
///
/// If there is no ID3v1 tag, if `new` is set to true the tag will be created,
/// otherwise [`Error::NoTag`] is produced.
pub fn migrate_version_file(
    path: impl AsRef<Path>,
    new: bool,
    trap: &impl Trap,
    version: u8,
) -> Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    migrate_version(&mut f, new, trap, version)
}

/// Remove ID3v1 tag from the stream.
///
/// # Returns
/// The tag version that was in the stream. If there was no ID3v1 tag,
/// [`Error::NoTag`] is returned.
pub fn remove<R: Read + Seek + SetLength>(w: &mut R) -> Result<u8> {
    let (Some(ver), len) = seek_to_start(&mut *w)? else {
        return Err(Error::NoTag);
    };
    w.set_length(len)?;
    Ok(ver)
}

/// Remove ID3v1 tag from the file.
///
/// # Returns
/// The tag version that was in the stream. If there was no ID3v1 tag,
/// [`Error::NoTag`] is returned.
pub fn remove_from_file(path: impl AsRef<Path>) -> Result<u8> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(path)?;
    remove(&mut f)
}

fn prepare_mode_write<R: Read + Write + Seek>(
    w: &mut R,
    r: &impl TagRetrieve,
    mode: WriteMode,
    trap: &impl Trap,
) -> Result<(Id3v1Tag, Option<u8>)> {
    let (mut tag, ver);
    match mode.kind {
        WriteModeKind::Fill => {
            (tag, ver) = read_and_seek_to_start(&mut *w, trap)?;
            tag.retrieve_fill(r);
        }
        WriteModeKind::Update => {
            (tag, ver) = read_and_seek_to_start(&mut *w, trap)?;
            tag.retrieve(r);
        }
        WriteModeKind::Overwrite => {
            ver = seek_to_start(&mut *w)?.0.take_if(|v| *v >= 2);
            tag = Id3v1Tag::default();
            tag.retrieve(r);
        }
    };
    if ver.is_none() && !mode.add {
        return Err(Error::NoTag);
    }
    Ok((tag, ver))
}

fn read_and_seek_to_start<R: Read + Seek>(
    r: &mut R,
    trap: &impl Trap,
) -> Result<(Id3v1Tag, Option<u8>)> {
    let tag = Id3v1Tag::try_from_seek_ver(&mut *r, trap)?;
    let (tag, mut ver) = if let Some((tag, ver)) = tag {
        (tag, Some(ver))
    } else {
        (Id3v1Tag::default(), None)
    };
    ver = match ver {
        None => {
            r.seek(SeekFrom::End(0))?;
            None
        }
        Some(0..2) => {
            r.seek(SeekFrom::End(-(Id3v1Tag::LEN0 as i64)))?;
            Some(1)
        }
        Some(_) => {
            r.seek(SeekFrom::End(-(Id3v1Tag::LEN2 as i64)))?;
            Some(2)
        }
    };
    Ok((tag, ver))
}

fn seek_to_start(mut r: impl Read + Seek) -> Result<(Option<u8>, u64)> {
    let len = r.seek(SeekFrom::End(0))?;
    if len < 128 {
        return Ok((None, len));
    }

    let mut buf = [0; 3];
    r.seek(SeekFrom::End(-(Id3v1Tag::LEN0 as i64)))?;
    r.read_exact(&mut buf)?;
    if buf != *b"TAG" {
        let len = r.seek(SeekFrom::End(0))?;
        return Ok((None, len));
    }

    if len < 256 {
        let pos = r.seek(SeekFrom::End(-(Id3v1Tag::LEN0 as i64)))?;
        return Ok((Some(0), pos));
    }

    r.seek(SeekFrom::End(-(Id3v1Tag::LEN2 as i64)))?;
    r.read_exact(&mut buf)?;
    if buf != *b"EXT" {
        let pos = r.seek(SeekFrom::End(-(Id3v1Tag::LEN0 as i64)))?;
        Ok((Some(1), pos))
    } else {
        let pos = r.seek(SeekFrom::End(-(Id3v1Tag::LEN2 as i64)))?;
        Ok((Some(2), pos))
    }
}
