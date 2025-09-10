mod frame_header;

use std::io::{BufRead, Seek};

use crate::{
    DataType, Error, PictureKind, Result, TagStore, TagStoreExt,
    bread::Bread,
    id3::v2::{
        frame34, header::Header, read_comment, read_date23, read_genres23,
        read_length, read_num_of, read_picture34, read_popularimeter,
        read_string, read_string_list23, read_time23, read_year,
    },
    trap::Trap,
};

pub use self::frame_header::*;

// Implementation is based on: https://id3.org/id3v2.3.0

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

    if header.extended_header34() {
        let len = r.get_be::<u32>()? - 4;
        r.seek_by(len as i64)?;
        pos += 4;
    }

    let mut comments = vec![];
    let mut ratings = vec![];

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
            frame34::APIC
                if store.stores_data(DataType::Picture(
                    PictureKind::all_id3(),
                )) =>
            {
                read_picture34(&mut r, store, trap, header.size as i64)?;
            }
            frame34::TALB if store.stores_data(DataType::Album) => {
                if let Some(r) = r.witht(hsize, trap, read_string)? {
                    store.set_album(r);
                }
            }
            frame34::TCON if store.stores_data(DataType::Genres) => {
                if let Some(g) = r.witht(hsize, trap, read_genres23)? {
                    store.set_genres(g);
                }
            }
            frame34::TDAT if store.stores_data(DataType::Date) => {
                if let Some(d) = r.witht(hsize, trap, read_date23)? {
                    store.set_date_time(d);
                }
            }
            frame34::TIT2 if store.stores_data(DataType::Title) => {
                if let Some(t) = r.witht(hsize, trap, read_string)? {
                    store.set_title(t);
                }
            }
            frame34::TIME if store.stores_data(DataType::Time) => {
                if let Some(t) = r.witht(hsize, trap, read_time23)? {
                    store.set_time(t);
                }
            }
            frame34::TLEN if store.stores_data(DataType::Length) => {
                if let Some(l) = r.witht(hsize, trap, read_length)? {
                    store.set_length(l);
                }
            }
            frame34::TPE1 if store.stores_data(DataType::Artists) => {
                if let Some(a) = r.witht(hsize, trap, read_string_list23)? {
                    store.set_artists(a);
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
            frame34::TYER if store.stores_data(DataType::Year) => {
                if let Some(d) = r.witht(hsize, trap, read_year)? {
                    store.set_date_time(d);
                }
            }
            frame34::COMM if store.stores_data(DataType::Comments) => {
                comments.extend(r.witht(hsize, trap, read_comment)?);
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
