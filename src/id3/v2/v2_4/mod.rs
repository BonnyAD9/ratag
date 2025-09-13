mod frame_header;

use self::frame_header::*;

use std::io::{BufRead, Seek};

use crate::{
    DataType, Error, PictureKind, Result, TagStore, TagStoreExt, TagType,
    bread::Bread,
    id3::{
        genres::get_genre,
        v2::{
            frame34, header::Header, read_comment, read_length, read_num_of,
            read_picture34, read_popularimeter, read_string, read_string_enc,
            read_year,
        },
    },
    parsers,
    trap::Trap,
};

// Implementation is based on:
// - https://id3.org/id3v2.4.0-structure
// - https://id3.org/id3v2.4.0-frames

pub fn from_bread(
    mut r: Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    header: Header,
) -> Result<()> {
    store.set_tag_type(TagType::Id3v2(4));

    let mut pos = 0;

    if header.extended_header34() {
        let len = r.withc(parsers::syncsafe_be_u32)? - 4;
        r.seek_by(len as i64)?;
        pos += len + 4;
    }

    let mut comments = vec![];
    let mut ratings = vec![];

    while !store.done() && pos + 10 < header.size {
        let mut header: FrameHeader = r.get()?;
        pos += header.size + 10;

        if header.compression()
            || header.encryption()
            || header.unsynchronization()
        {
            trap.error(Error::Unsupported(
                "ID3v2.4 compression, encryption and unsynchronization.",
            ))?;
            r.seek_by(header.size as i64)?;
            continue;
        }

        let mut ads = 0;
        if header.grouping() {
            ads += 1;
            header.size -= 1;
        }
        if header.data_length_indicator() {
            ads += 4;
            header.size -= 4;
        }
        r.seek_by(ads)?;

        let hsize = header.size as usize;

        match header.id {
            0 => break,
            frame34::TIT2 if store.stores_data(DataType::Title) => {
                if let Some(t) = r.witht(hsize, trap, read_string)? {
                    store.set_title(t);
                }
            }
            frame34::TALB if store.stores_data(DataType::Album) => {
                if let Some(a) = r.witht(hsize, trap, read_string)? {
                    store.set_album(a);
                }
            }
            frame34::TRCK
                if store.stores_data(DataType::Track)
                    || store.stores_data(DataType::TrackCount) =>
            {
                if let Some((t, c)) = r.witht(hsize, trap, read_num_of)? {
                    store.set_track(t);
                    if let Some(c) = c {
                        store.set_track_count(c);
                    }
                }
            }
            frame34::TPOS
                if store.stores_data(DataType::Disc)
                    || store.stores_data(DataType::DiscCount) =>
            {
                if let Some((d, c)) = r.witht(hsize, trap, read_num_of)? {
                    store.set_disc(d);
                    if let Some(c) = c {
                        store.set_disc_count(c);
                    }
                }
            }
            frame34::TPE1 if store.stores_data(DataType::Artists) => {
                if let Some(a) = r.witht(hsize, trap, read_string_list)? {
                    store.set_artists(a);
                }
            }
            frame34::TLEN if store.stores_data(DataType::Length) => {
                if let Some(l) = r.witht(hsize, trap, read_length)? {
                    store.set_length(l);
                }
            }
            frame34::TCON if store.stores_data(DataType::Genres) => {
                if let Some(g) = r.witht(hsize, trap, read_genres)? {
                    store.set_genres(g);
                }
            }
            frame34::TDRL
                if store.stores_data(DataType::Year)
                    || store.stores_data(DataType::Date)
                    || store.stores_data(DataType::Time) =>
            {
                if let Some(dt) = r.witht(hsize, trap, read_year)? {
                    store.set_date_time(dt);
                }
            }
            frame34::COMM if store.stores_data(DataType::Comments) => {
                comments.extend(r.witht(hsize, trap, read_comment)?);
            }
            frame34::APIC
                if store.stores_data(DataType::Picture(
                    PictureKind::all_id3(),
                )) =>
            {
                read_picture34(&mut r, store, trap, header.size as i64)?;
            }
            frame34::TCOP if store.stores_data(DataType::Copyright) => {
                if let Some(c) = r.witht(hsize, trap, read_string)? {
                    store.set_copyright(c);
                }
            }
            frame34::POPM if store.stores_data(DataType::Ratings) => {
                ratings.extend(r.witht(hsize, trap, read_popularimeter)?);
            }
            _ => {
                r.seek_by(header.size as i64)?;
            }
        }
    }

    if !comments.is_empty() {
        store.set_comments(comments);
    }

    if !ratings.is_empty() {
        store.set_ratings(ratings);
    }

    Ok(())
}

fn read_genres(data: &[u8], trap: &impl Trap) -> Result<Vec<String>> {
    let mut res = read_string_list(data, trap)?;
    for g in &mut res {
        if g == "RX" {
            *g = "Remix".to_string();
        } else if g == "CR" {
            *g = "Cover".to_string();
        } else if let Ok(gn) = g.parse()
            && let Some(gs) = get_genre(gn)
        {
            *g = gs.to_string()
        }
    }
    Ok(res)
}

fn read_string_list(mut data: &[u8], trap: &impl Trap) -> Result<Vec<String>> {
    let enc = data[0];
    data = &data[1..];
    let mut res = vec![];

    while !data.is_empty() {
        let (l, s) = read_string_enc(enc, data, trap)?;
        res.push(s);
        data = &data[l..];
    }

    Ok(res)
}
