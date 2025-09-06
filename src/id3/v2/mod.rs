mod frame;
mod frame_header;
mod header;
mod id3v2;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    num::ParseIntError,
    path::Path,
    str::FromStr,
    time::Duration,
};

use crate::{
    Bread, Comment, DataType, Error, Picture, PictureKind, Result, TagStore,
    TagStoreExt, Trap, TrapExt,
    id3::get_genre,
    parsers::{self, DateTime},
};

pub use self::id3v2::*;
use self::{frame_header::*, header::*};

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

    if header.version != Header::VERSION3 {
        return Err(Error::Unsupported("ID3v2 other version than 3."));
    }
    if header.unsynchronization() {
        return Err(Error::Unsupported("ID3v2 unsynchronization"));
    }

    let mut pos = 0;

    if header.extended_header() {
        let len = r.get_be::<u32>()? - 4;
        r.seek_by(len as i64)?;
        pos += 4;
    }

    let mut comments = vec![];

    while !store.done() && pos + 10 < header.size {
        let mut header: FrameHeader = r.get()?;
        pos += header.size + 10;
        if header.compression() || header.encryption() {
            r.seek_by(header.size as i64)?;
            continue;
        }
        if header.grouping() {
            r.seek_by(1)?;
            header.size -= 1;
        }

        let hsize = header.size as usize;

        match header.id {
            0 => break,
            frame::APIC
                if store.stores_data(DataType::Picture(
                    PictureKind::all_id3(),
                )) =>
            {
                read_picture(&mut r, store, trap, header.size as i64)?;
            }
            frame::TALB if store.stores_data(DataType::Album) => {
                if let Some(r) = r.witht(hsize, trap, read_string)? {
                    store.set_album(Some(r));
                }
            }
            frame::TCON if store.stores_data(DataType::Genres) => {
                if let Some(g) = r.witht(hsize, trap, read_genres)? {
                    store.set_genres(g);
                }
            }
            frame::TDAT if store.stores_data(DataType::Date) => {
                if let Some(d) = r.witht(hsize, trap, read_date)? {
                    store.set_date_time(d);
                }
            }
            frame::TIT2 if store.stores_data(DataType::Title) => {
                if let Some(t) = r.witht(hsize, trap, read_string)? {
                    store.set_title(Some(t));
                }
            }
            frame::TIME if store.stores_data(DataType::Time) => {
                if let Some(t) = r.witht(hsize, trap, read_time)? {
                    store.set_time(Some(t));
                }
            }
            frame::TLEN if store.stores_data(DataType::Length) => {
                if let Some(l) = r.witht(hsize, trap, read_length)? {
                    store.set_length(Some(l));
                }
            }
            frame::TPE1 if store.stores_data(DataType::Artists) => {
                if let Some(a) = r.witht(hsize, trap, read_string_list)? {
                    store.set_artists(a);
                }
            }
            frame::TPOS
                if store.stores_data(DataType::Disc)
                    || store.stores_data(DataType::DiscCount) =>
            {
                if let Some(a) = r.witht(hsize, trap, read_position)? {
                    store.set_disc(Some(a.0));
                    store.set_disc_count(a.1);
                }
            }
            frame::TRCK
                if store.stores_data(DataType::Track)
                    || store.stores_data(DataType::TrackCount) =>
            {
                if let Some(a) = r.witht(hsize, trap, read_position)? {
                    store.set_track(Some(a.0));
                    store.set_track_count(a.1);
                }
            }
            frame::TYER if store.stores_data(DataType::Year) => {
                if let Some(d) = r.witht(hsize, trap, read_year)? {
                    store.set_date_time(d);
                }
            }
            frame::COMM if store.stores_data(DataType::Comments) => {
                comments.extend(r.witht(hsize, trap, read_comment)?);
            }
            _ => {
                r.seek_by(header.size as i64)?;
            }
        }
    }

    store.set_comments(comments);

    Ok(())
}

fn read_picture(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    mut length: i64,
) -> Result<()> {
    let enc = r.next()?;
    length -= 1;

    let null = match encoding_null(enc) {
        Ok(n) => n,
        Err(e) => {
            r.seek_by(length)?;
            return trap.error(e);
        }
    };

    if length < 0 {
        r.seek_by(length)?;
        return trap.error(Error::InvalidLength);
    }

    let (res, len) =
        r.witht_until_chunk(&[0], length as usize, trap, |s, t| {
            read_string_enc_nonull(0, &s[..s.len() - 1], t)
        })?;
    length -= len as i64;

    let Some(mime) = res else {
        r.seek_by(length)?;
        return Ok(());
    };

    let kind = if let Some(p) = PictureKind::from_id3(r.next()?) {
        p
    } else {
        trap.error(Error::InvalidPictureKind)?;
        PictureKind::default()
    };
    length -= 1;

    if !store.stores_data(DataType::Picture(kind)) {
        r.seek_by(length)?;
        return Ok(());
    }

    if length < 0 {
        r.seek_by(length)?;
        return trap.error(Error::InvalidLength);
    }

    let (res, len) =
        r.witht_until_chunk(null, length as usize, trap, |s, t| {
            read_string_enc_nonull(enc, &s[..s.len() - null.len()], t)
        })?;
    length -= len as i64;

    let Some(description) = res else {
        r.seek_by(length)?;
        return Ok(());
    };

    if length < 0 {
        r.seek_by(length)?;
        return trap.error(Error::InvalidLength);
    }

    let data = r.read_exact_owned(length as usize)?;

    store.add_picture(Picture::from_id3(mime, description, kind, data));

    Ok(())
}

fn read_comment(mut data: &[u8], trap: &impl Trap) -> Result<Comment> {
    let enc = data[0];
    let language = trap.res(parsers::ascii(&data[1..4], trap))?;

    data = &data[4..];
    let (len, desc) = read_string_enc(enc, data, trap)?;
    let (_, value) = read_string_enc(enc, &data[len..], trap)?;

    Ok(Comment {
        language,
        desciption: Some(desc),
        value,
    })
}

fn read_time(data: &[u8], trap: &impl Trap) -> Result<Duration> {
    let s = read_string(data, trap)?;
    parsers::time_only(&s)
}

fn read_date(data: &[u8], trap: &impl Trap) -> Result<DateTime> {
    let s = read_string(data, trap)?;
    parsers::date(&s, trap)
}

fn read_position(data: &[u8], trap: &impl Trap) -> Result<(u32, Option<u32>)> {
    let s = read_string(data, trap)?;
    parsers::num_of(&s, trap)
}

fn read_string_list(data: &[u8], trap: &impl Trap) -> Result<Vec<String>> {
    let s = read_string(data, trap)?;
    Ok(s.split('/').map(|a| a.to_string()).collect())
}

fn read_length(data: &[u8], trap: &impl Trap) -> Result<Duration> {
    Ok(Duration::from_millis(read_number(data, trap)?))
}

fn read_number<T: FromStr<Err = ParseIntError>>(
    data: &[u8],
    trap: &impl Trap,
) -> Result<T> {
    parsers::num(&read_string(data, trap)?)
}

fn read_year(data: &[u8], trap: &impl Trap) -> Result<DateTime> {
    let year = read_string(data, trap)?;
    parsers::year(&year, trap)
}

fn read_genres(data: &[u8], trap: &impl Trap) -> Result<Vec<String>> {
    let s = read_string(data, trap)?;
    let mut s = s.as_str();
    let mut res = vec![];

    loop {
        if s.starts_with('(') {
            s = &s[1..];
        } else {
            if !s.is_empty() {
                res.push(s.to_string());
            }
            break;
        }
        if s.starts_with('(') {
            res.push(s.to_string());
            break;
        }
        let Some((pos, _)) = s.char_indices().find(|(_, c)| *c == ')') else {
            trap.error(Error::InvalidGenreRef)?;
            res.push(s.to_string());
            break;
        };
        let gref = &s[..pos];
        s = &s[pos + 1..];
        let genre = match gref {
            "RX" => Some("Remix"),
            "CR" => Some("Cover"),
            g => trap
                .res(g.parse().map_err(|_| Error::InvalidGenreRef))?
                .and_then(get_genre),
        };
        res.push(genre.unwrap_or(gref).to_string());
    }

    Ok(res)
}

fn read_string(data: &[u8], trap: &impl Trap) -> Result<String> {
    if data.is_empty() {
        return Err(Error::InvalidLength);
    }

    read_string_enc(data[0], &data[1..], trap).map(|(_, s)| s)
}

fn read_string_enc(
    enc: u8,
    d: &[u8],
    trap: &impl Trap,
) -> Result<(usize, String)> {
    match enc {
        0 => parsers::iso_8859_1_nt(d, trap),
        1 => parsers::utf_16_bom_nt(d, trap),
        _ => Err(Error::InvalidEncoding),
    }
}

fn read_string_enc_nonull(
    enc: u8,
    d: &[u8],
    trap: &impl Trap,
) -> Result<String> {
    match enc {
        0 => parsers::iso_8859_1(d, trap),
        1 => parsers::utf_16_bom(d, trap),
        _ => Err(Error::InvalidEncoding),
    }
}

fn encoding_null(enc: u8) -> Result<&'static [u8]> {
    let res: Result<&[_]> = match enc {
        0 => Ok(&[0]),    // ISO-8859-1
        1 => Ok(&[0, 0]), // UTF-16
        _ => Err(Error::Unsupported("Unknown encoding in ID3v2.")),
    };
    res
}
