mod chunk;
mod chunk_header;
mod wave_fmt;

use self::{chunk_header::*, wave_fmt::*};

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    num::ParseIntError,
    path::Path,
    str::FromStr,
    time::Duration,
};

use crate::{
    Comment, DataType, Error, Result, TagRead, TagStore, TagStoreExt,
    bread::Bread,
    parsers::{self, DateTime},
    trap::Trap,
};

// Implementation based on:
// - https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/Docs/riffmci.pdf
// - https://exiftool.org/TagNames/RIFF.html

/// Riff tag reader.
#[derive(Debug)]
pub struct Riff;

impl<R: BufRead + Seek, S: TagStore, T: Trap> TagRead<R, S, T> for Riff {
    fn extensions(&self) -> &[&str] {
        &[
            "wav", "wave", "avi", "ani", "pal", "rdi", "dib", "rmi", "rmm",
            "webp",
        ]
    }

    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()> {
        from_seek(r, store, trap)
    }
}

/// Read riff tag from stream. Will seek to correct position before reading.
pub fn from_seek(
    mut r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    r.rewind()?;
    from_read(r, store, trap)
}

/// Read riff tags from file.
pub fn from_file(
    f: impl AsRef<Path>,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    from_read(BufReader::new(File::open(f)?), store, trap)
}

/// Read riff tags from stream. Doesn't seek before reading.
pub fn from_read(
    r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let mut r = Bread::new(r);

    let header: ChunkHeader = r.get()?;
    if header.id != chunk::RIFF {
        return Err(Error::NoTag);
    }

    let typ: u32 = r.get_be()?;

    let mut pos = 0;

    let mut avg_bytes_per_sec = None;
    let mut data_size = None;

    while !store.done() && pos + 8 < header.size {
        let header: ChunkHeader = r.get()?;
        pos += header.size + (header.size & 1) + 8;

        let hsize = header.size as usize;

        match header.id {
            chunk::LIST => read_list(&mut r, store, trap, header.size as i64)?,
            chunk::FMT
                if typ == chunk::WAVE
                    && store.stores_data(DataType::Length) =>
            {
                if let Some(fmt) = r.witht(hsize, trap, read_wave_fmt)? {
                    avg_bytes_per_sec = Some(fmt.avg_bytes_per_sec);
                }
            }
            chunk::DATA
                if typ == chunk::WAVE
                    && store.stores_data(DataType::Length) =>
            {
                data_size = Some(header.size);
                r.seek_by(header.size as i64)?;
            }
            _ => r.seek_by(header.size as i64)?,
        }

        r.seek_by(header.size as i64 & 1)?;

        if let Some(abps) = avg_bytes_per_sec
            && let Some(ds) = data_size
        {
            store.set_length(Duration::from_secs_f64(ds as f64 / abps as f64));
        }
    }

    Ok(())
}

fn read_list(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    mut size: i64,
) -> Result<()> {
    let typ: u32 = r.get_be()?;
    size -= 4;

    if typ != chunk::INFO {
        r.seek_by(size)?;
        return Ok(());
    }

    while size > 0 {
        let header: ChunkHeader = r.get()?;
        size -= 8 + header.size as i64 + (header.size as i64 & 1);
        let hsize = header.size as usize;

        match header.id {
            chunk::IART if store.stores_data(DataType::Artists) => {
                if let Some((_, a)) =
                    r.witht(hsize, trap, parsers::ascii_nt)?
                {
                    store.set_artists(vec![a]);
                }
            }
            chunk::ICMT if store.stores_data(DataType::Comments) => {
                if let Some((_, c)) =
                    r.witht(hsize, trap, parsers::ascii_nt)?
                {
                    store.set_comments(vec![Comment::from_value(c)]);
                }
            }
            chunk::ICOP if store.stores_data(DataType::Copyright) => {
                if let Some((_, c)) =
                    r.witht(hsize, trap, parsers::ascii_nt)?
                {
                    store.set_copyright(c);
                }
            }
            chunk::IGNR if store.stores_data(DataType::Genres) => {
                if let Some((_, g)) =
                    r.witht(hsize, trap, parsers::ascii_nt)?
                {
                    store.set_genres(vec![g]);
                }
            }
            chunk::ICRD
                if store.stores_data(DataType::Year)
                    || store.stores_data(DataType::Date)
                    || store.stores_data(DataType::Time) =>
            {
                if let Some(d) = r.witht(hsize, trap, read_date)? {
                    store.set_date_time(d);
                }
            }
            chunk::INAM if store.stores_data(DataType::Title) => {
                if let Some((_, t)) =
                    r.witht(hsize, trap, parsers::ascii_nt)?
                {
                    store.set_title(t);
                }
            }
            chunk::IPRD if store.stores_data(DataType::Album) => {
                if let Some((_, a)) =
                    r.witht(hsize, trap, parsers::ascii_nt)?
                {
                    store.set_album(a);
                }
            }
            chunk::IPRT if store.stores_data(DataType::Track) => {
                if let Some(t) = r.witht(hsize, trap, read_int)? {
                    store.set_track(t);
                }
            }
            chunk::PRT1 if store.stores_data(DataType::Disc) => {
                if let Some(d) = r.witht(hsize, trap, read_int)? {
                    store.set_disc(d);
                }
            }
            chunk::PRT2 if store.stores_data(DataType::DiscCount) => {
                if let Some(c) = r.witht(hsize, trap, read_int)? {
                    store.set_disc_count(c);
                }
            }
            _ => r.seek_by(header.size as i64)?,
        }

        r.seek_by(header.size as i64 & 1)?;
    }

    r.seek_by(size)?;
    Ok(())
}

fn read_wave_fmt(d: &[u8], _: &impl Trap) -> Result<WaveFmt> {
    if d.len() < 14 {
        return Err(Error::InvalidLength);
    }
    Ok(WaveFmt::from_bytes(d[..14].try_into().unwrap()))
}

fn read_date(d: &[u8], trap: &impl Trap) -> Result<DateTime> {
    let (_, s) = parsers::ascii_nt(d, trap)?;
    parsers::year(&s, trap)
}

fn read_int<T: FromStr<Err = ParseIntError>>(
    d: &[u8],
    trap: &impl Trap,
) -> Result<T> {
    let (_, s) = parsers::ascii_nt(d, trap)?;
    parsers::num(&s)
}
