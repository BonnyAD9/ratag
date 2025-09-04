mod bread;
mod err;
pub mod id3;
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
};

use crate::id3::Id3;

use self::bread::Bread;
mod basic_tag;
mod data_type;
mod tag_read;
mod tag_store;
pub mod trap;

pub use place_macro::place;

pub use self::{
    basic_tag::*, data_type::*, err::*, tag_read::*, tag_store::*,
};

use self::trap::*;

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

pub fn read_tag<R: BufRead + Seek, S: TagStore, T: Trap>(
    r: &mut R,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let tags: [&dyn TagRead<R, S, T>; _] = [&Id3];
    read_any_tag(tags, r, store, trap)
}

pub fn read_tag_from_file<S: TagStore, T: Trap>(
    f: impl AsRef<Path>,
    store: &mut S,
    trap: &T,
) -> Result<()> {
    let tags: [&dyn TagRead<_, S, T>; _] = [&Id3];
    read_any_tag_from_file(tags, f, store, trap)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        assert!(true);
    }
}
