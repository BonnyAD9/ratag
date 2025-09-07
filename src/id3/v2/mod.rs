mod frame;
mod header;
mod id3v2;
mod v2_3;
mod v2_4;

use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    num::ParseIntError,
    path::Path,
    str::FromStr,
    time::Duration,
};

use crate::{
    Comment, DataType, Error, Picture, PictureKind, Result, TagStore, Trap,
    TrapExt,
    bread::Bread,
    parsers::{self, DateTime},
};

use self::header::*;
pub use self::id3v2::*;

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
    if !r.expect(b"ID3")? {
        return Err(Error::NoTag);
    }

    let header: Header = r.get()?;

    match header.major_version {
        Header::MAJOR_VERSION3 => v2_3::from_bread(r, store, trap, header),
        Header::MAJOR_VERSION4 => v2_4::from_bread(r, store, trap, header),
        _ => Err(Error::Unsupported("ID3v2 other version than 3 and 4.")),
    }
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

    let is_uri = mime == "-->";

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

    store.add_picture(Picture::from_id3(
        mime,
        description,
        kind,
        data,
        is_uri,
    ));

    Ok(())
}

fn read_comment(mut data: &[u8], trap: &impl Trap) -> Result<Comment> {
    let enc = data[0];
    let language = trap.res(parsers::ascii(&data[1..4], trap))?;

    data = &data[4..];
    let (len, desc) = read_string_enc_nt(enc, data, trap)?;
    let (_, value) = read_string_enc(enc, &data[len..], trap)?;

    Ok(Comment {
        language,
        desciption: Some(desc),
        value,
    })
}

fn read_year(data: &[u8], trap: &impl Trap) -> Result<DateTime> {
    let year = read_string(data, trap)?;
    parsers::year(&year, trap)
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

fn read_num_of(data: &[u8], trap: &impl Trap) -> Result<(u32, Option<u32>)> {
    let s = read_string(data, trap)?;
    parsers::num_of(&s, trap)
}

fn read_string(data: &[u8], trap: &impl Trap) -> Result<String> {
    if data.is_empty() {
        return Err(Error::InvalidLength);
    }

    read_string_enc(data[0], &data[1..], trap).map(|(_, s)| s)
}

fn read_string_enc_nt(
    enc: u8,
    d: &[u8],
    trap: &impl Trap,
) -> Result<(usize, String)> {
    match enc {
        0 => parsers::iso_8859_1_nt(d, trap),
        1 => parsers::utf_16_bom_nt(d, trap),
        2 => parsers::utf_16_be_nt(d, trap),
        3 => parsers::utf_8_nt(d, trap),
        _ => Err(Error::InvalidEncoding),
    }
}

fn read_string_enc(
    enc: u8,
    d: &[u8],
    trap: &impl Trap,
) -> Result<(usize, String)> {
    match enc {
        0 => parsers::iso_8859_1_mnt(d, trap),
        1 => parsers::utf_16_bom_mnt(d, trap),
        2 => parsers::utf_16_be_mnt(d, trap),
        3 => parsers::utf_8_mnt(d, trap),
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
        2 => parsers::utf_16_be(d, trap),
        3 => parsers::utf_8(d, trap),
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
