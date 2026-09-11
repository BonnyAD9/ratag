#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]

/// Module for reading tags from asf files.
pub mod asf;
mod atomic_swap;
mod bread;
mod containers;
mod data_type;
mod err;
/// Module for reading metadata from flac files.
pub mod flac;
mod formatters;
/// Module for reading ID3v1 and ID3v2 tags.
pub mod id3;
/// Module for reading tags for mp4 files.
pub mod mp4;
mod named_file;
mod parsers;
/// Module for reading tags from riff files.
pub mod riff;
mod set_length;
/// Tagging containers.
pub mod tag;
mod tag_format;
mod tag_read;
mod tag_remove;
mod tag_retrieve;
mod tag_store;
mod tag_write;
/// Module for managing how to handle errors.
pub mod trap;
/// Module for parsing vorbis comments.
pub mod vorbis;
mod write_mode;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, Write},
    path::Path,
};

use crate::{
    asf::Asf, bread::Bread, flac::Flac, id3::Id3, mp4::Mp4,
    named_file::NamedFile, riff::Riff, tag_write::TagWrite, trap::*,
};

pub use self::{
    containers::*, data_type::*, err::*, set_length::*, tag_format::*,
    tag_read::*, tag_remove::*, tag_retrieve::*, tag_store::*, write_mode::*,
};

macro_rules! all_tags {
    () => {
        [&Id3, &Flac, &Mp4, &Asf, &Riff]
    };
}

/// Reads from reader with the first tag format that succeeds.
///
/// # Errors
/// - If reading tag fails for one of the formats.
/// - [`Error::NoTag`] if none of the tags are included in the file.
pub fn read_any_tag<
    'a,
    R: 'a,
    S: 'a,
    T: 'a,
    I: IntoIterator<Item = &'a dyn TagRead<R, S, T>>,
>(
    tags: I,
    r: &mut R,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    for t in tags {
        match t.store(r, store, trap) {
            Ok(_) => return Ok(()),
            Err(Error::NoTag) => continue,
            e => return e,
        }
    }
    Err(Error::NoTag)
}

/// Reads from reader with the first tag format that succeeds.
///
/// This also prioritizes tag formats based on the extension of the file.
///
/// # Errors
/// - If reading tag fails for one of the formats.
/// - [`Error::NoTag`] if none of the tags are included in the file.
pub fn read_any_tag_from_file<
    'a,
    S: 'a,
    T: 'a,
    I: IntoIterator<Item = &'a dyn TagRead<BufReader<File>, S, T>>,
>(
    tags: I,
    path: impl AsRef<Path>,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let path = path.as_ref();
    let mut r = BufReader::new(File::open(path)?);
    let Some(ext) = path.extension() else {
        return read_any_tag(tags, &mut r, store, trap);
    };

    let mut primary = vec![];
    let mut secondary = vec![];

    for t in tags {
        if t.extensions().iter().any(|e| *e == ext) {
            primary.push(t);
        } else {
            secondary.push(t);
        }
    }

    read_any_tag(primary.into_iter().chain(secondary), &mut r, store, trap)
}

/// Reads tag with any of the tags supported by this crate.
///
/// # Errors
/// - Reading tag fails.
/// - [`Error::NoTag`] if the file contains no supported tags.
pub fn read_tag<R: BufRead + Seek, S: TagStore, T: Trap>(
    r: &mut R,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let tags: [&dyn TagRead<R, S, T>; _] = all_tags!();
    read_any_tag(tags, r, store, trap)
}

/// Reads tag with any of the tag formats supported by this crate.
///
/// This also prioritizes the formats based on the file extension.
///
/// # Errors
/// - Reading tag fails.
/// - [`Error::NoTag`] if the file contains no supported tags.
pub fn read_tag_from_file<S: TagStore, T: Trap>(
    f: impl AsRef<Path>,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let tags: [&dyn TagRead<_, S, T>; _] = all_tags!();
    read_any_tag_from_file(tags, f, store, trap)
}

/// Write the first tag that detects itself within the stream according to the
/// mode.
///
/// Returns [`Error::NoTag`] if the stream doesn't contain any recognized tag.
///
/// To write a new tag into a stream, use the dedicated functions of that tag.
pub fn write_any_tag<
    'a,
    W: 'a,
    R: 'a,
    T: 'a,
    I: IntoIterator<Item = &'a dyn TagWrite<W, R, T>>,
>(
    tags: I,
    w: &mut W,
    r: &R,
    mode: impl Into<WriteMode>,
    trap: &T,
) -> Result<TagType> {
    let mode = mode.into();
    for t in tags {
        match t.tag_write(w, r, mode, trap) {
            Ok(t) => return Ok(t),
            Err(Error::NoTag) => continue,
            e => return e,
        }
    }
    Err(Error::NoTag)
}

/// Write the first tag that detects itself within the file according to the
/// mode.
///
/// Returns [`Error::NoTag`] if the file doesn't contain any recognized tag.
///
/// To write a new tag into a file, use the dedicated functions of that tag.
pub fn write_any_tag_to_file<
    'a,
    R: 'a,
    T: 'a,
    I: IntoIterator<Item = &'a dyn TagWrite<NamedFile, R, T>>,
>(
    tags: I,
    path: impl AsRef<Path>,
    r: &R,
    mode: impl Into<WriteMode>,
    trap: &T,
) -> Result<TagType> {
    let path = path.as_ref();
    let mut w = NamedFile::modify(path)?;
    let Some(ext) = path.extension() else {
        return write_any_tag(tags, &mut w, r, mode, trap);
    };

    let mut primary = vec![];
    let mut secondary = vec![];

    for t in tags {
        if t.extensions().iter().any(|e| *e == ext) {
            primary.push(t);
        } else {
            secondary.push(t);
        }
    }

    write_any_tag(primary.into_iter().chain(secondary), &mut w, r, mode, trap)
}

/// Detect what tag is within the stream and write it with new information
/// according to the mode.
///
/// Returns [`Error::NoTag`] if no supported tag was detected.
///
/// Currently supports only `ID3v1`.
pub fn write_tag<W: Read + Write + Seek, R: TagRetrieve, T: Trap>(
    w: &mut W,
    r: &mut R,
    mode: impl Into<WriteMode>,
    trap: &T,
) -> Result<TagType> {
    let tags: [&dyn TagWrite<W, R, T>; _] = [&id3::v1::Id3v1];
    write_any_tag(tags, w, r, mode, trap)
}

/// Detect what tag is within the file and write it with new information
/// according to the mode.
///
/// Returns [`Error::NoTag`] if no supported tag was detected.
///
/// Currently supports only `ID3v1`.
pub fn write_tag_to_file<R: TagRetrieve, T: Trap>(
    path: impl AsRef<Path>,
    r: &mut R,
    mode: impl Into<WriteMode>,
    trap: &T,
) -> Result<TagType> {
    let tags: [&dyn TagWrite<NamedFile, R, T>; _] = [&id3::v1::Id3v1];
    write_any_tag_to_file(tags, path, r, mode, trap)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        assert!(true);
    }
}
