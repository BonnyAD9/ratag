#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod basic_tag;
mod bread;
mod comment;
mod data_type;
mod err;
/// Module for reading metadata from flac files.
pub mod flac;
/// Module for reading ID3v1 and ID3v2 tags.
pub mod id3;
/// Module for reading tags for mp4 files.
pub mod mp4;
mod parsers;
mod picture;
mod picture_kind;
mod picture_tag;
mod tag_read;
mod tag_store;
/// Module for managing how to handle errors.
pub mod trap;
/// Module for parsing vorbis comments.
pub mod vorbis;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use crate::{bread::Bread, flac::Flac, id3::Id3, mp4::Mp4, trap::*};

pub use self::{
    basic_tag::*, comment::*, data_type::*, err::*, picture::*,
    picture_kind::*, picture_tag::*, tag_read::*, tag_store::*,
};

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
    let tags: [&dyn TagRead<R, S, T>; _] = [&Id3, &Flac, &Mp4];
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
    let tags: [&dyn TagRead<_, S, T>; _] = [&Id3, &Flac, &Mp4];
    read_any_tag_from_file(tags, f, store, trap)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        assert!(true);
    }
}
