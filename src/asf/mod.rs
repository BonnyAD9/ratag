mod guid;
mod object;

use self::object::*;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    num::{ParseIntError, TryFromIntError},
    path::Path,
    str::FromStr,
    time::Duration,
};

use crate::{
    Comment, DataType, Error, Picture, PictureKind, Rating, Result, TagRead,
    TagStore, bread::Bread, parsers, trap::Trap,
};

const STR: u16 = 0;
const BYTES: u16 = 1;
const INT32: u16 = 3;
const INT64: u16 = 4;
const INT16: u16 = 5;

/// The Asf tag reader.
#[derive(Debug)]
pub struct Asf;

impl<R: BufRead + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Asf {
    fn extensions(&self) -> &[&str] {
        &["asf", "wma", "wmv"]
    }

    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

/// Read asf tag from file.
pub fn from_file(
    f: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    from_read(BufReader::new(File::open(f)?), store, trap)
}

/// Read asf file from stream. Seeks to the correct position before reading.
pub fn from_seek(
    mut r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    r.rewind()?;
    from_read(r, store, trap)
}

/// Read asf tags from stream. Assumes that the stream is seeked to the correct
/// position.
pub fn from_read(
    r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let mut r = Bread::new(r);
    let header: Object = r.get()?;

    if header.guid != guid::FILE_HEADER {
        return Err(Error::NoTag);
    }

    let mut count: u32 = r.get_le()?;
    r.seek_by(2)?;

    while !store.done() && count > 0 {
        count -= 1;
        let mut header: Object = r.get()?;
        header.size -= 24;
        let hsize = header.size as usize;

        match header.guid {
            guid::FILE_PROPERTIES if store.stores_data(DataType::Length) => {
                if let Some(l) = r.witht(hsize, trap, read_file_properties)? {
                    store.set_length(l);
                }
            }
            guid::CONTENT_DESCRIPTION
                if store.stores_data(DataType::Title)
                    || store.stores_data(DataType::Artists)
                    || store.stores_data(DataType::Copyright)
                    || store.stores_data(DataType::Comments)
                    || store.stores_data(DataType::Ratings) =>
            {
                read_content_description(&mut r, store, trap, header.size)?;
            }
            guid::EXTENDED_CONTENT_DESCRIPTION => {
                read_extended_content_description(
                    &mut r,
                    store,
                    trap,
                    header.size,
                )?;
            }
            _ => r.seek_by(header.size)?,
        }
    }

    Ok(())
}

fn read_extended_content_description(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    mut size: i64,
) -> Result<()> {
    let mut count: u16 = r.get_le()?;
    size -= 2;

    let mut genres = vec![];

    while !store.done() && count > 0 {
        count -= 1;
        let mut nlen: u16 = r.get_le()?;
        nlen += nlen & 1;
        let name = r.witht(nlen as usize, trap, parsers::utf_16_le_nt)?;
        let typ: u16 = r.get_le()?;
        let mut vlen: u16 = r.get_le()?;
        if typ == STR {
            vlen += vlen & 1;
        }

        size -= (nlen + vlen + 6) as i64;

        let Some((_, name)) = name else {
            r.seek_by(vlen as i64)?;
            continue;
        };

        let vsize = vlen as usize;

        match name.as_str() {
            "WM/AlbumTitle" if store.stores_data(DataType::Album) => {
                if let Some(a) =
                    r.witht(vsize, trap, |d, t| get_string(d, t, typ))?
                {
                    store.set_album(a);
                }
            }
            "WM/Year" if store.stores_data(DataType::Year) => {
                if let Some(y) =
                    r.witht(vsize, trap, |d, t| get_num(d, t, typ))?
                {
                    store.set_year(y);
                }
            }
            "WM/TrackNumber" if store.stores_data(DataType::Track) => {
                if let Some(t) =
                    r.witht(vsize, trap, |d, t| get_num(d, t, typ))?
                {
                    store.set_track(t);
                }
            }
            "WM/PartOfSet" if store.stores_data(DataType::Disc) => {
                if let Some(d) =
                    r.witht(vsize, trap, |d, t| get_num(d, t, typ))?
                {
                    store.set_disc(d);
                }
            }
            "WM/Genre" if store.stores_data(DataType::Genres) => {
                genres.extend(
                    r.witht(vsize, trap, |d, t| get_string(d, t, typ))?,
                );
            }
            "WM/Picture" => {
                read_picture(r, store, trap, typ, vlen as i64)?;
            }
            _ => r.seek_by(vlen as i64)?,
        }
    }

    if !genres.is_empty() {
        store.set_genres(genres);
    }

    r.seek_by(size)?;

    Ok(())
}

fn read_picture(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    typ: u16,
    mut size: i64,
) -> Result<()> {
    if typ != BYTES {
        r.seek_by(size)?;
        return Err(Error::InvalidDataType);
    }

    let kind = if let Some(k) = PictureKind::from_id3(r.next()?) {
        k
    } else {
        trap.error(Error::InvalidPictureKind)?;
        PictureKind::default()
    };
    let len: u32 = r.get_le()?;
    size -= 5;

    if size < 0 {
        r.seek_by(size)?;
        return trap.error(Error::InvalidLength);
    }

    let (mime, l) =
        r.witht_until_chunk(&[0, 0], size as usize, trap, |d, t| {
            parsers::utf_16_le_nt(&d[..d.len() - 2], t)
        })?;
    size -= l as i64;
    let Some((_, mime)) = mime else {
        r.seek_by(size)?;
        return Err(Error::StringNotTerminated);
    };

    if size < 0 {
        r.seek_by(size)?;
        return trap.error(Error::InvalidLength);
    }

    let is_uri = mime == "-->";

    let (desc, l) =
        r.witht_until_chunk(&[0, 0], size as usize, trap, |d, t| {
            parsers::utf_16_le_nt(&d[..d.len() - 2], t)
        })?;
    size -= l as i64;
    let Some((_, description)) = desc else {
        r.seek_by(size)?;
        return Err(Error::StringNotTerminated);
    };

    if size < 0 {
        r.seek_by(size)?;
        return trap.error(Error::InvalidLength);
    }

    let data = r.read_exact_owned(len as usize)?;

    store.add_picture(Picture::from_id3(
        Some(mime),
        Some(description),
        kind,
        data,
        is_uri,
    ));

    Ok(())
}

fn get_num<
    T: FromStr<Err = ParseIntError> + TryFrom<i64, Error = TryFromIntError>,
>(
    d: &[u8],
    trap: &impl Trap,
    typ: u16,
) -> Result<T> {
    match typ {
        STR => {
            let (_, s) = parsers::utf_16_le_nt(d, trap)?;
            Ok(s.parse()?)
        }
        INT64 => Ok(i64::from_le_bytes(d.try_into()?).try_into()?),
        INT32 => Ok((i32::from_le_bytes(d.try_into()?) as i64).try_into()?),
        INT16 => Ok((i16::from_le_bytes(d.try_into()?) as i64).try_into()?),
        _ => Err(Error::InvalidDataType),
    }
}

fn get_string(d: &[u8], trap: &impl Trap, typ: u16) -> Result<String> {
    if typ != STR {
        return Err(Error::InvalidDataType);
    }

    Ok(parsers::utf_16_le_nt(d, trap)?.1)
}

fn read_content_description(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    mut size: i64,
) -> Result<()> {
    let tlen: u16 = r.get_le()?;
    let alen: u16 = r.get_le()?;
    let cplen: u16 = r.get_le()?;
    let cmlen: u16 = r.get_le()?;
    let rlen: u16 = r.get_le()?;

    if let Some((_, t)) =
        r.witht(tlen as usize, trap, parsers::utf_16_le_mnt)?
        && !t.is_empty()
    {
        store.set_title(t);
    }
    if let Some((_, a)) =
        r.witht(alen as usize, trap, parsers::utf_16_le_mnt)?
        && !a.is_empty()
    {
        store.set_artists(vec![a]);
    }
    if let Some((_, c)) =
        r.witht(cplen as usize, trap, parsers::utf_16_le_mnt)?
        && !c.is_empty()
    {
        store.set_copyright(c);
    }
    if let Some((_, c)) =
        r.witht(cmlen as usize, trap, parsers::utf_16_le_mnt)?
        && !c.is_empty()
    {
        store.set_comments(vec![Comment::from_value(c)]);
    }
    if let Some((_, r)) =
        r.witht(rlen as usize, trap, parsers::utf_16_le_mnt)?
        && !r.is_empty()
    {
        store.set_ratings(vec![Rating::Text(r)]);
    }

    size -= (10 + tlen + alen + cplen + cmlen + rlen) as i64;

    r.seek_by(size)
}

fn read_file_properties(d: &[u8], _: &impl Trap) -> Result<Duration> {
    if d.len() < 64 {
        return Err(Error::InvalidLength);
    }

    let dur = u64::from_le_bytes(d[40..48].try_into().unwrap());
    let preroll = u64::from_le_bytes(d[56..64].try_into().unwrap());

    Ok(Duration::from_secs(dur / 10_000_000)
        + Duration::from_nanos(dur % 10_000_000)
        - Duration::from_millis(preroll))
}
