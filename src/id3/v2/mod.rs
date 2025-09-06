mod frame;
mod header;
mod id3v2;
mod v2_3;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use crate::{Error, Result, TagStore, Trap, bread::Bread};

use self::header::*;
pub use self::id3v2::*;

// Implementation is based on: https://id3.org/id3v2.3.0

/// Read ID3v2 tag without assuming that the reader is already at the correct
/// position.
pub fn from_seek(
    mut r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    r.rewind()?;
    from_read(r, store, trap)
}

/// Read ID3v2 tag from file.
pub fn from_file(
    f: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let f = BufReader::new(File::open(f)?);
    from_read(f, store, trap)
}

/// Read ID3v2 tag, assuming that the reader is already at the correct
/// position.
pub fn from_read(
    r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let mut r = Bread::new(r);

    let header: Header = r.get()?;

    match header.version {
        Header::VERSION3 => v2_3::from_bread(r, store, trap, header),
        _ => Err(Error::Unsupported("ID3v2 other version than 3.")),
    }
}
