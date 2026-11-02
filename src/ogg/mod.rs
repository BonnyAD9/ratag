mod ogg_header;
mod ogg_header_type;
mod ogg_packet_stream;
mod ogg_stream;
mod ogg_tag;

use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::Path,
};

use crate::{
    Error, Result, TagFormat, TagRead, TagStore, TagType, bread::Bread,
    trap::Trap, vorbis::OggVorbis,
};

pub use self::{
    ogg_header::*, ogg_header_type::*, ogg_packet_stream::*, ogg_stream::*,
    ogg_tag::*,
};

// Implementation is based on: https://www.rfc-editor.org/info/rfc3533

macro_rules! all_ogg_tags {
    () => {
        [&OggVorbis]
    };
}

/// TagReader for ogg files.
///
/// Since ogg can contain many codecs, the codec implementation has to be
/// specified inside the struct itself. It is expected to be some kind of
/// container that can be turned into iterator of `&dyn OggTag`. Using `()`
/// will default to the codecs implemented by this crate.
///
/// Containers implemented by this library are `[&dyn OggTag; N]` and
/// `Vec<Box<dyn OggTag>>`.
pub struct Ogg<C = ()>(pub C);

impl<C> TagFormat for Ogg<C> {
    fn extensions(&self) -> &[&str] {
        &["ogg"]
    }
}

impl<R, S, T> TagRead<R, S, T> for Ogg
where
    R: BufRead + Seek,
    S: TagStore,
    T: Trap,
{
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

impl<const N: usize, R, S, T> TagRead<R, S, T> for Ogg<[&dyn OggTag<S, T>; N]>
where
    R: BufRead + Seek,
    S: TagStore,
    T: Trap,
{
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        any_from_seek(self.0.iter().copied(), r, store, trap)
    }
}

impl<R, S, T> TagRead<R, S, T> for Ogg<Vec<Box<dyn OggTag<S, T>>>>
where
    R: BufRead + Seek,
    S: TagStore,
    T: Trap,
{
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        any_from_seek(self.0.iter().map(|a| &**a), r, store, trap)
    }
}

/// Read metadata from ogg file.
///
/// This will use the codecs supported by this crate.
pub fn from_file<'a, S: TagStore + 'a, T: Trap + 'a>(
    f: impl AsRef<Path>,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    from_read(BufReader::new(File::open(f)?), store, trap)
}

/// Read metadata from ogg stream. Don't assume the correct position within the
/// stream.
///
/// This will use the codecs supported by this crate.
pub fn from_seek<'a, S: TagStore + 'a, T: Trap + 'a>(
    mut r: impl BufRead + Seek,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    r.seek(SeekFrom::Start(0))?;
    from_read(r, store, trap)
}

/// Read meatadata from ogg stream. Assume that the position is correct.
///
/// This will use the codecs supported by this crate.
pub fn from_read<'a, S: TagStore + 'a, T: Trap + 'a>(
    r: impl BufRead,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let tags: [&dyn OggTag<S, T>; _] = all_ogg_tags!();
    any_from_read(tags.iter().copied(), r, store, trap)
}

/// Read metadata from ogg file.
///
/// This will try the supplied codecs in the order that they are presented.
pub fn any_from_file<
    'a,
    S: TagStore + 'a,
    T: Trap + 'a,
    I: Iterator<Item = &'a dyn OggTag<S, T>> + Clone,
>(
    tags: I,
    f: impl AsRef<Path>,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    any_from_read(tags, BufReader::new(File::open(f)?), store, trap)
}

/// Read metadata from ogg stream. Don't assume the correct position within the
/// stream.
///
/// This will try the supplied codecs in the order that they are presented.
pub fn any_from_seek<
    'a,
    S: TagStore + 'a,
    T: Trap + 'a,
    I: Iterator<Item = &'a dyn OggTag<S, T>> + Clone,
>(
    tags: I,
    mut r: impl BufRead + Seek,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    r.seek(SeekFrom::Start(0))?;
    any_from_read(tags, r, store, trap)
}

/// Read meatadata from ogg stream. Assume that the position is correct.
///
/// This will try the supplied codecs in the order that they are presented.
pub fn any_from_read<
    'a,
    S: TagStore + 'a,
    T: Trap + 'a,
    I: Iterator<Item = &'a dyn OggTag<S, T>> + Clone,
>(
    tags: I,
    r: impl BufRead,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let mut r = Bread::new(r);

    let mut streams = HashMap::new();
    let mut stream_cnt = 0;
    let mut err_cnt = 0;
    let mut identified = false;

    loop {
        if !r.expect(b"OggS")? {
            if !identified {
                return Err(Error::NoTag);
            } else if streams.is_empty() {
                break;
            } else {
                return Err(Error::OggDesynchronized);
            }
        }

        if !identified {
            store.set_tag_type(TagType::Ogg);
            identified = true;
        }

        let header: OggHeader = r.get()?;
        if header.version != 0 {
            return Err(Error::Unsupported("Ogg version other than 0"));
        }
        let len = r
            .read_exact(header.page_segments as usize)?
            .iter()
            .map(|&a| a as usize)
            .sum();
        let page = r.read_exact(len)?;

        if header.header_type.is_bos() {
            if let Some(s) =
                tags.clone().find_map(|t| t.get_stream(&header, page))
            {
                streams.insert(header.bitstream_serial_number, s);
                stream_cnt += 1;
            }
        } else if streams.is_empty() {
            break;
        }

        let Entry::Occupied(mut se) =
            streams.entry(header.bitstream_serial_number)
        else {
            continue;
        };

        let rem = match se.get_mut().read_page(&header, page, store, trap) {
            Ok(v) => !v,
            Err(Error::NoTag) => {
                err_cnt += 1;
                true
            }
            Err(e) => {
                err_cnt += 1;
                trap.error(e)?;
                true
            }
        };

        if rem || header.header_type.is_eos() {
            se.remove();
        }
    }

    if !identified {
        return Err(Error::NoTag);
    }

    if stream_cnt == 0 {
        return Err(Error::Unsupported("codec in Ogg container"));
    }

    if err_cnt == stream_cnt {
        return Err(Error::NoSuccessfulStream);
    }

    Ok(())
}
