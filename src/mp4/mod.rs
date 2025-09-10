mod boxtype;
mod full_box;
mod mp4_box;
mod opt_u64;

use self::{full_box::*, mp4_box::*, opt_u64::*};

use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Seek, SeekFrom},
    path::Path,
    time::Duration,
};

use crate::{
    Comment, DataType, Error, Picture, PictureKind, Result, TagRead, TagStore,
    TagStoreExt,
    bread::Bread,
    id3::genres::get_genre,
    parsers::{self, DateTime},
    trap::{Trap, TrapExt},
};

// Implementation is based on:
// https://web.archive.org/web/20091024221536/http://geocities.com/xhelmboyx/quicktime/formats/mp4-layout.txt

/// TagRead for mp4.
#[derive(Debug)]
pub struct Mp4;

impl<R: BufRead + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Mp4 {
    fn extensions(&self) -> &[&str] {
        &["mp4", "m4a", "m4p", "m4b", "m4r", "m4v"]
    }

    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

/// Read tags from mp4 file given by path.
pub fn from_path(
    p: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    from_read(BufReader::new(File::open(p)?), store, trap)
}

/// Read tags from mp4 file, this will seek to the correct position within the
/// file.
pub fn from_seek(
    mut r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    r.rewind()?;
    from_read(r, store, trap)
}

/// Read tags from mp4 file. This will not seek to correct position, it assumes
/// that it already is at the correct position.
pub fn from_read(
    r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let mut r = Bread::new(r);

    let mut bx: Mp4Box = r.get()?;
    if bx.boxtype != boxtype::FTYP {
        return Err(Error::NoTag);
    }

    while !store.done() {
        match bx.boxtype {
            boxtype::MOOV => {
                read_moov(&mut r, store, trap, bx.size_next)?;
            }
            _ => {
                let Some(s) = *bx.size_next else {
                    break;
                };
                r.useek_by(s)?;
            }
        }

        bx = match r.get() {
            Ok(v) => v,
            Err(Error::Io(e)) if e.kind() == ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn read_moov(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    len: OptU64,
) -> Result<()> {
    let mut pos = OptU64(Some(0));
    while pos < len && !store.done() {
        let bx: Mp4Box = r.get()?;
        pos += bx.size_total;
        match bx.boxtype {
            boxtype::UDTA => {
                read_udta(r, store, trap, bx.size_next)?;
            }
            boxtype::MVHD if store.stores_data(DataType::Length) => {
                read_mvhd(r, store, trap, bx.size_next)?;
            }
            _ => {
                let Some(s) = *bx.size_next else {
                    break;
                };
                r.useek_by(s)?;
            }
        }
    }

    Ok(())
}

fn read_udta(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    len: OptU64,
) -> Result<()> {
    let mut pos = OptU64(Some(0));
    while pos < len && !store.done() {
        let bx: Mp4Box = r.get()?;
        pos += bx.size_total;
        match bx.boxtype {
            boxtype::META => {
                read_meta(r, store, trap, bx.size_next)?;
            }
            boxtype::CPRT if store.stores_data(DataType::Copyright) => {
                let Some(size) = *bx.size_next else {
                    r.seek(SeekFrom::End(0))?;
                    return trap
                        .error(Error::Unsupported("Size to end of file."));
                };
                if let Some(c) =
                    r.witht(size as usize, trap, read_copyright)?
                {
                    store.set_copyright(c);
                }
            }
            _ => {
                let Some(s) = *bx.size_next else {
                    break;
                };
                r.useek_by(s)?;
            }
        }
    }

    Ok(())
}

fn read_mvhd(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    len: OptU64,
) -> Result<()> {
    let Some(mut len) = *len else {
        r.seek(SeekFrom::End(0))?;
        return trap.error(Error::InvalidLength);
    };

    if len < 32 {
        r.useek_by(len)?;
        return trap.error(Error::InvalidLength);
    }

    let fb: FullBox = r.get()?;
    len -= 4;
    let (ts, dur) = match fb.version {
        0 => {
            r.seek_by(8)?;
            let time_scale: u32 = r.get_be()?;
            let duration: u32 = r.get_be()?;
            len -= 16;
            (time_scale, duration as u64)
        }
        1 => {
            r.seek_by(16)?;
            let time_scale: u32 = r.get_be()?;
            let duration: u64 = r.get_be()?;
            len -= 28;
            (time_scale, duration)
        }
        _ => {
            r.useek_by(len)?;
            return trap.error(Error::Unsupported(
                "Unsupported movie header version. Max v1 is supported.",
            ));
        }
    };

    r.useek_by(len)?;

    let dur = Duration::from_secs_f64(dur as f64 / ts as f64);
    store.set_length(dur);

    Ok(())
}

fn read_meta(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    len: OptU64,
) -> Result<()> {
    let mut pos = OptU64(Some(4));
    r.seek_by(4)?; // fullbox
    while pos < len && !store.done() {
        let bx: Mp4Box = r.get()?;
        pos += bx.size_total;
        match bx.boxtype {
            boxtype::ILST => {
                read_ilst(r, store, trap, bx.size_next)?;
            }
            _ => {
                let Some(s) = *bx.size_next else {
                    break;
                };
                r.useek_by(s)?;
            }
        }
    }

    Ok(())
}

fn read_copyright(d: &[u8], trap: &impl Trap) -> Result<String> {
    if d.len() < 6 {
        return Err(Error::InvalidLength);
    }

    Ok(parsers::utf_8_nt(&d[6..], trap)?.1)
}

fn read_ilst(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    len: OptU64,
) -> Result<()> {
    let mut pos = OptU64(Some(0));
    while pos < len && !store.done() {
        let bx: Mp4Box = r.get()?;
        pos += bx.size_total;
        let len = bx.size_next;
        match bx.boxtype {
            boxtype::NAM if store.stores_data(DataType::Title) => {
                read_annotation(r, trap, len, read_string, |s| {
                    store.set_title(s);
                    Ok(())
                })?;
            }
            boxtype::CMT if store.stores_data(DataType::Comments) => {
                read_annotation(r, trap, len, read_string, |s| {
                    store.set_comments(vec![Comment::from_value(s)]);
                    Ok(())
                })?;
            }
            boxtype::DAY
                if store.stores_data(DataType::Year)
                    || store.stores_data(DataType::Date)
                    || store.stores_data(DataType::Time) =>
            {
                read_annotation(r, trap, len, read_day, |dt| {
                    store.set_date_time(dt);
                    Ok(())
                })?;
            }
            boxtype::ART if store.stores_data(DataType::Artists) => {
                read_annotation(r, trap, len, read_string, |s| {
                    store.set_artists(vec![s]);
                    Ok(())
                })?;
            }
            boxtype::TRK | boxtype::TRKN
                if store.stores_data(DataType::Track)
                    || store.stores_data(DataType::TrackCount) =>
            {
                read_annotation(r, trap, len, read_num_of, |(t, c)| {
                    store.set_track(t);
                    store.set_track_count(c);
                    Ok(())
                })?;
            }
            boxtype::ALB if store.stores_data(DataType::Album) => {
                read_annotation(r, trap, len, read_string, |s| {
                    store.set_album(s);
                    Ok(())
                })?;
            }
            boxtype::GNRE if store.stores_data(DataType::Genres) => {
                read_annotation(r, trap, len, read_genre, |g| {
                    store.set_genres(vec![g]);
                    Ok(())
                })?;
            }
            boxtype::DISK
                if store.stores_data(DataType::Disc)
                    || store.stores_data(DataType::DiscCount) =>
            {
                read_annotation(r, trap, len, read_num_of, |(d, c)| {
                    store.set_disc(d);
                    store.set_disc_count(c);
                    Ok(())
                })?;
            }
            boxtype::COVR
                if store.stores_data(DataType::Picture(
                    PictureKind::FRONT_COVER,
                )) =>
            {
                read_annotation(r, trap, len, read_image, |d| {
                    store.add_picture(Picture::from_data(
                        d,
                        PictureKind::FRONT_COVER,
                    ));
                    Ok(())
                })?;
            }
            _ => {
                let Some(s) = *bx.size_next else {
                    break;
                };
                r.useek_by(s)?;
            }
        }
    }

    Ok(())
}

fn read_annotation<
    T,
    R: BufRead + Seek,
    Tr: Trap,
    S: FnOnce(T) -> Result<()>,
>(
    r: &mut Bread<R>,
    trap: &Tr,
    len: OptU64,
    parse: impl FnOnce(&mut Bread<R>, &Tr, OptU64, S) -> Result<()>,
    store: S,
) -> Result<()> {
    let mut pos = OptU64(Some(0));
    while pos < len {
        let bx: Mp4Box = r.get()?;
        pos += bx.size_total;
        match bx.boxtype {
            boxtype::DATA => {
                parse(r, trap, bx.size_next, store)?;
                if let Some(pos) = *pos
                    && let Some(len) = *len
                {
                    r.useek_by(len - pos)?;
                } else {
                    r.seek(SeekFrom::End(0))?;
                }
                return Ok(());
            }
            _ => {
                let Some(s) = *bx.size_next else {
                    break;
                };
                r.useek_by(s)?;
            }
        }
    }

    Ok(())
}

fn read_image(
    r: &mut Bread<impl BufRead + Seek>,
    trap: &impl Trap,
    len: OptU64,
    store: impl FnOnce(Vec<u8>) -> Result<()>,
) -> Result<()> {
    let Some(mut len) = *len else {
        r.seek(SeekFrom::End(0))?;
        return trap.error(Error::InvalidLength);
    };

    if len < 8 {
        r.useek_by(len)?;
        return trap.error(Error::InvalidLength);
    }

    let fb: FullBox = r.get()?;
    r.seek_by(4)?;
    len -= 8;

    if fb.flags != FullBox::IMAGE {
        r.useek_by(len)?;
        return trap.error(Error::Unsupported("Box type flags for image."));
    }

    let data = r.read_exact_owned(len as usize)?;
    trap.prop(store(data))
}

fn read_genre(
    r: &mut Bread<impl BufRead + Seek>,
    trap: &impl Trap,
    len: OptU64,
    store: impl FnOnce(String) -> Result<()>,
) -> Result<()> {
    let Some(mut len) = *len else {
        r.seek(SeekFrom::End(0))?;
        return trap.error(Error::InvalidLength);
    };

    if len < 10 {
        r.useek_by(len)?;
        return trap.error(Error::InvalidLength);
    }

    let fb: FullBox = r.get()?;
    r.seek_by(4)?;
    len -= 8;

    match fb.flags {
        FullBox::BINARY => {
            if len != 2 {
                r.useek_by(len)?;
                return trap.error(Error::InvalidLength);
            }
            let num: u16 = r.get_be()?;
            if num > u8::MAX as u16 {
                return trap.error(Error::InvalidGenreRef);
            }
            let Some(g) = get_genre(num as u8) else {
                return trap.error(Error::InvalidGenreRef);
            };
            trap.prop(store(g.to_string()))
        }
        FullBox::TEXT => {
            if let Some(g) =
                r.witht(len as usize, trap, |d, t| parsers::utf_8(d, t))?
            {
                return trap.prop(store(g));
            }
            Ok(())
        }
        _ => {
            r.useek_by(len)?;
            trap.error(Error::Unsupported("Box type flags for genre."))
        }
    }
}

fn read_num_of(
    r: &mut Bread<impl BufRead + Seek>,
    trap: &impl Trap,
    len: OptU64,
    store: impl FnOnce((u32, u32)) -> Result<()>,
) -> Result<()> {
    let Some(mut len) = *len else {
        r.seek(SeekFrom::End(0))?;
        return trap.error(Error::InvalidLength);
    };

    if len < 14 {
        r.useek_by(len)?;
        return trap.error(Error::InvalidLength);
    }

    let fb: FullBox = r.get()?;
    len -= 4;

    if fb.flags != FullBox::BINARY {
        r.useek_by(len)?;
        return trap.error(Error::Unsupported("Box type flags for num of."));
    }

    r.seek_by(6)?;

    let pos: u16 = r.get_be()?;
    let cnt: u16 = r.get_be()?;
    let res = store((pos as u32, cnt as u32));

    len -= 10;
    r.useek_by(len)?;
    res
}

fn read_day(
    r: &mut Bread<impl BufRead + Seek>,
    trap: &impl Trap,
    len: OptU64,
    store: impl FnOnce(DateTime) -> Result<()>,
) -> Result<()> {
    read_string(r, trap, len, |s| store(parsers::year(&s, trap)?))
}

fn read_string(
    r: &mut Bread<impl BufRead + Seek>,
    trap: &impl Trap,
    len: OptU64,
    store: impl FnOnce(String) -> Result<()>,
) -> Result<()> {
    let Some(mut len) = *len else {
        r.seek(SeekFrom::End(0))?;
        return trap.error(Error::InvalidLength);
    };

    if len < 8 {
        r.useek_by(len)?;
        return trap.error(Error::InvalidLength);
    }

    let fb: FullBox = r.get()?;
    len -= 4;

    if fb.flags != FullBox::TEXT {
        r.useek_by(len)?;
        return trap.error(Error::Unsupported("Box type flags for string."));
    }

    r.seek_by(4)?;
    len -= 4;

    if let Some(s) =
        r.witht(len as usize, trap, |d, t| parsers::utf_8(d, t))?
    {
        store(s)
    } else {
        Ok(())
    }
}
