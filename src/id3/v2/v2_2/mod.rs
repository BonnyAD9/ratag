mod frame;
mod frame_header;
mod picture_header;

use self::{frame_header::*, picture_header::*};

use std::io::{BufRead, Seek};

use crate::{
    DataType, Error, Picture, PictureKind, Result, TagStore, TagStoreExt,
    bread::Bread,
    id3::v2::{
        encoding_null, header::Header, read_comment, read_date23,
        read_genres23, read_length, read_num_of, read_string,
        read_string_enc_nonull, read_string_list23, read_time23, read_year,
    },
    parsers,
    trap::{Trap, TrapExt},
};

pub fn from_bread(
    mut r: Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    header: Header,
) -> Result<()> {
    if header.compression2() {
        return Err(Error::Unsupported("ID3v2.2 compression."));
    }

    let mut pos = 0;

    let mut comments = vec![];

    while !store.done() && pos + 6 < header.size {
        let header: FrameHeader = r.get()?;
        pos += header.size + 6;

        let hsize = header.size as usize;

        match header.id {
            0 => break,
            frame::TT2 if store.stores_data(DataType::Title) => {
                if let Some(t) = r.witht(hsize, trap, read_string)? {
                    store.set_title(Some(t));
                }
            }
            frame::TP1 if store.stores_data(DataType::Artists) => {
                if let Some(a) = r.witht(hsize, trap, read_string_list23)? {
                    store.set_artists(a);
                }
            }
            frame::TCO if store.stores_data(DataType::Genres) => {
                if let Some(a) = r.witht(hsize, trap, read_genres23)? {
                    store.set_genres(a);
                }
            }
            frame::TAL if store.stores_data(DataType::Album) => {
                if let Some(a) = r.witht(hsize, trap, read_string)? {
                    store.set_album(Some(a));
                }
            }
            frame::TPA
                if store.stores_data(DataType::Disc)
                    || store.stores_data(DataType::DiscCount) =>
            {
                if let Some((d, c)) = r.witht(hsize, trap, read_num_of)? {
                    store.set_disc(Some(d));
                    store.set_disc_count(c);
                }
            }
            frame::TRK
                if store.stores_data(DataType::Track)
                    || store.stores_data(DataType::TrackCount) =>
            {
                if let Some((t, c)) = r.witht(hsize, trap, read_num_of)? {
                    store.set_track(Some(t));
                    store.set_track_count(c);
                }
            }
            frame::TYE if store.stores_data(DataType::Year) => {
                if let Some(d) = r.witht(hsize, trap, read_year)? {
                    store.set_date_time(d);
                }
            }
            frame::TDA if store.stores_data(DataType::Date) => {
                if let Some(d) = r.witht(hsize, trap, read_date23)? {
                    store.set_date_time(d);
                }
            }
            frame::TIM if store.stores_data(DataType::Time) => {
                if let Some(t) = r.witht(hsize, trap, read_time23)? {
                    store.set_time(Some(t));
                }
            }
            frame::TLE if store.stores_data(DataType::Length) => {
                if let Some(l) = r.witht(hsize, trap, read_length)? {
                    store.set_length(Some(l));
                }
            }
            frame::COM if store.stores_data(DataType::Comments) => {
                comments.extend(r.witht(hsize, trap, read_comment)?);
            }
            frame::PIC => {
                read_picture(&mut r, store, trap, header.size as i64)?;
            }
            _ => r.seek_by(header.size as i64)?,
        }
    }

    Ok(())
}

fn read_picture(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    mut length: i64,
) -> Result<()> {
    let header: PictureHeader = r.get()?;
    length -= 5;

    let kind = if let Some(t) = PictureKind::from_id3(header.typ) {
        t
    } else {
        trap.error(Error::InvalidPictureKind)?;
        PictureKind::default()
    };

    if !store.stores_data(DataType::Picture(kind)) {
        r.seek_by(length)?;
        return Ok(());
    }

    let null = match encoding_null(header.encoding) {
        Ok(n) => n,
        Err(e) => {
            r.seek_by(length)?;
            return trap.error(e);
        }
    };

    let format = trap.res(parsers::iso_8859_1(&header.format, trap))?;
    let is_uri = format.as_deref() == Some("-->");

    if length < 0 {
        r.seek_by(length)?;
        return trap.error(Error::InvalidLength);
    }

    let (description, len) =
        r.witht_until_chunk(null, length as usize, trap, |s, t| {
            read_string_enc_nonull(
                header.encoding,
                &s[..s.len() - null.len()],
                t,
            )
        })?;
    length -= len as i64;

    if length < 0 {
        r.seek_by(length)?;
        return trap.error(Error::InvalidLength);
    }

    let data = r.read_exact_owned(length as usize)?;

    store.add_picture(Picture::from_id3(
        format,
        description,
        kind,
        data,
        is_uri,
    ));

    Ok(())
}
