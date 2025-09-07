mod frame_header;

use std::{
    io::{BufRead, Seek},
    time::Duration,
};

use crate::{
    DataType, Error, PictureKind, Result, TagStore, TagStoreExt,
    bread::Bread,
    id3::{
        get_genre,
        v2::{
            frame, header::Header, read_comment, read_length, read_num_of,
            read_picture, read_string, read_year,
        },
    },
    parsers::{self, DateTime},
    trap::{Trap, TrapExt},
};

pub use self::frame_header::*;

pub fn from_bread(
    mut r: Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    header: Header,
) -> Result<()> {
    if header.unsynchronization() {
        return Err(Error::Unsupported("ID3v2.3 unsynchronization."));
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
            trap.error(Error::Unsupported(
                "ID3v2.3 compression and encryption.",
            ))?;
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
                if let Some(a) = r.witht(hsize, trap, read_num_of)? {
                    store.set_disc(Some(a.0));
                    store.set_disc_count(a.1);
                }
            }
            frame::TRCK
                if store.stores_data(DataType::Track)
                    || store.stores_data(DataType::TrackCount) =>
            {
                if let Some(a) = r.witht(hsize, trap, read_num_of)? {
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

fn read_time(data: &[u8], trap: &impl Trap) -> Result<Duration> {
    let s = read_string(data, trap)?;
    parsers::time_only(&s)
}

fn read_date(data: &[u8], trap: &impl Trap) -> Result<DateTime> {
    let s = read_string(data, trap)?;
    parsers::date(&s, trap)
}

fn read_string_list(data: &[u8], trap: &impl Trap) -> Result<Vec<String>> {
    let s = read_string(data, trap)?;
    Ok(s.split('/').map(|a| a.to_string()).collect())
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
