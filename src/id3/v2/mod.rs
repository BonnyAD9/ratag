mod frame;
mod frame_header;
mod header;
mod id3v2;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    path::Path,
    str::FromStr,
    time::Duration,
};

use encoding::{
    Encoding,
    all::{ISO_8859_1, UTF_16BE, UTF_16LE},
};

use crate::{
    Bread, DataType, Error, Result, TagStore, Trap, TrapExt, id3::get_genre,
};

pub use self::id3v2::*;
use self::{frame_header::*, header::*};

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

    while !store.done() && pos < header.size {
        let mut header: FrameHeader = r.get()?;
        pos += header.size + 10;
        if header.compression() || header.encryption() {
            r.seek_by(header.size as i64)?;
            pos += header.size;
            continue;
        }
        if header.grouping() {
            r.seek_by(1)?;
            header.size -= 1;
        }

        let hsize = header.size as usize;

        match header.id {
            0 => break,
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
            frame::TIT2 if store.stores_data(DataType::Title) => {
                if let Some(t) = r.witht(hsize, trap, read_string)? {
                    store.set_title(Some(t));
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
                if let Some(a) = r.witht(hsize, trap, read_number)? {
                    store.set_year(Some(a));
                }
            }
            _ => {
                r.seek_by(header.size as i64)?;
            }
        }
    }

    Ok(())
}

fn read_position(data: &[u8], trap: &impl Trap) -> Result<(u32, Option<u32>)> {
    let s = read_string(data, trap)?;
    if let Some((t, o)) = s.split_once('/') {
        let t = t.parse().map_err(|_| Error::InvalidDigit)?;
        let o = trap.res(o.parse().map_err(|_| Error::InvalidDigit))?;
        Ok((t, o))
    } else {
        let t = s.parse().map_err(|_| Error::InvalidDigit)?;
        Ok((t, None))
    }
}

fn read_string_list(data: &[u8], trap: &impl Trap) -> Result<Vec<String>> {
    let s = read_string(data, trap)?;
    Ok(s.split('/').map(|a| a.to_string()).collect())
}

fn read_length(data: &[u8], trap: &impl Trap) -> Result<Duration> {
    Ok(Duration::from_millis(read_number(data, trap)?))
}

fn read_number<T: FromStr>(data: &[u8], trap: &impl Trap) -> Result<T> {
    read_string(data, trap)?
        .parse()
        .map_err(|_| Error::InvalidDigit)
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
    let end: &[_] = match data[0] {
        0 => &[0],    // ISO-8859-1
        1 => &[0, 0], // UTF-16
        _ => return Err(Error::Unsupported("Unknown encoding in ID3v2.")),
    };

    let end = data[1..].windows(end.len()).position(|a| a == end);

    let end = if let Some(end) = end {
        end + 1
    } else {
        trap.error(Error::StringNotTerminated)?;
        data.len()
    };

    let trap = trap.decoder_trap();
    let e = |_| Error::InvalidEncoding;
    match data {
        [0, ..] => ISO_8859_1.decode(&data[1..end], trap).map_err(e),
        [1, 0xfe, 0xff, ..] => UTF_16BE.decode(&data[3..end], trap).map_err(e),
        [1, 0xff, 0xfe, ..] => UTF_16LE.decode(&data[3..end], trap).map_err(e),
        _ => Err(Error::InvalidEncoding),
    }
}
