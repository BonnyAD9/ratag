mod frame;
mod frame_header;
mod picture_header;

use self::{frame_header::*, picture_header::*};

use std::{
    borrow::Cow,
    fmt::Display,
    io::{BufRead, Seek, Write},
    time::Duration,
};

use crate::{
    Comment, DataType, Error, Picture, PictureKind, Result, TagRetrieve,
    TagStore, TagStoreExt, TagType,
    bread::Bread,
    formatters,
    id3::{
        genres::get_genre_id,
        v2::{
            encoding_null, header::Header, read_comment, read_date23,
            read_genres23, read_length, read_num_of, read_popularimeter,
            read_string, read_string_enc_nonull, read_string_list23,
            read_time23, read_year, unparsed_mode::UnparsedMode,
        },
    },
    parsers,
    trap::{Trap, TrapExt},
};

// Implementation is based on: https://id3.org/id3v2-00.

pub fn from_bread(
    mut r: Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    header: Header,
    unparsed: &mut Vec<u8>,
    umode: UnparsedMode,
) -> Result<()> {
    store.set_tag_type(TagType::Id3v2(2));

    if header.compression2() {
        return Err(Error::Unsupported("ID3v2.2 compression."));
    }

    let mut pos = 0;

    let mut comments = vec![];
    let mut ratings = vec![];

    while !store.done() && pos + 6 < header.size {
        let header: FrameHeader = r.get()?;
        pos += header.size + 6;

        let hsize = header.size as usize;

        match header.id {
            0 => break,
            frame::TT2 if store.stores_data(DataType::Title) => {
                if let Some(t) = r.witht(hsize, trap, read_string)? {
                    store.set_title(t);
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
                    store.set_album(a);
                }
            }
            frame::TPA
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
            frame::TRK
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
                    store.set_time(t);
                }
            }
            frame::TLE if store.stores_data(DataType::Length) => {
                if let Some(l) = r.witht(hsize, trap, read_length)? {
                    store.set_length(l);
                }
            }
            frame::COM if store.stores_data(DataType::Comments) => {
                comments.extend(r.witht(hsize, trap, read_comment)?);
            }
            frame::PIC => {
                read_picture(&mut r, store, trap, header.size as i64)?;
            }
            frame::TCR if store.stores_data(DataType::Copyright) => {
                if let Some(c) = r.witht(hsize, trap, read_string)? {
                    store.set_copyright(c);
                }
            }
            frame::POP if store.stores_data(DataType::Ratings) => {
                ratings.extend(r.witht(hsize, trap, read_popularimeter)?);
            }
            frame::TP2 if store.stores_data(DataType::AlbumArtist) => {
                if let Some(a) = r.witht(hsize, trap, read_string)? {
                    store.set_album_artist(a);
                }
            }
            _ if umode == UnparsedMode::None => {
                r.seek_by(header.size as i64)?
            }
            _ => {
                unparsed.extend(header.to_bytes());
                r.read_exact_to(header.size as usize, unparsed)?;
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

pub fn to_vec(
    w: &mut Vec<u8>,
    r: &impl TagRetrieve,
    trap: &impl Trap,
    unparsed: &[u8],
) -> Result<()> {
    let start = w.len();
    w.extend(b"ID3");
    let header = Header {
        major_version: 0x02,
        minor_version: 0x00,
        flags: 0x00,
        size: 0x00,
    };
    w.extend(&header.to_bytes());

    if let Some(t) = r.get_title() {
        write_string(w, frame::TT2, t, trap)?;
    }
    write_artists(w, frame::TP1, r.get_artists(), trap)?;
    write_genres(w, frame::TCO, r.get_genres(), trap)?;
    if let Some(a) = r.get_album() {
        write_string(w, frame::TAL, a, trap)?;
    }
    if let Some(d) = r.get_disc() {
        write_num_of(w, frame::TPA, d, r.get_disc_count(), trap)?;
    }
    if let Some(t) = r.get_track() {
        write_num_of(w, frame::TRK, t, r.get_track_count(), trap)?;
    }
    if let Some(y) = r.get_year() {
        format_ascii(w, frame::TYE, y, trap)?;
    }
    if let Some(d) = r.get_date() {
        write_date(w, frame::TDA, d, trap)?;
    }
    if let Some(t) = r.get_time() {
        write_time(w, frame::TIM, t, trap)?;
    }
    if let Some(l) = r.get_length() {
        format_ascii(w, frame::TLE, l.as_millis(), trap)?;
    }
    write_comments(w, r.get_comments(), trap)?;

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

    let format = trap
        .res(parsers::iso_8859_1(&header.format, trap))?
        .as_deref()
        .map(format_to_mime);
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
        format.map(|f| f.into_owned()),
        description,
        kind,
        data,
        is_uri,
    ));

    Ok(())
}

fn write_frame(
    res: &mut Vec<u8>,
    id: u32,
    f: impl FnOnce(&mut Vec<u8>) -> Result<()>,
    trap: &impl Trap,
) -> Result<()> {
    let start = res.len();
    let mut header = FrameHeader { id, size: 0 };
    res.extend(header.to_bytes());

    match f(res) {
        Ok(_) => {}
        Err(e) => {
            res.shrink_to(start);
            return trap.error(e);
        }
    }

    let len = res.len() - start - 6;
    if len > 0x0fff_ffff {
        res.shrink_to(start);
        return trap.error(Error::DataTooLong);
    }
    header.size = len as u32;
    res[start..start + 6].copy_from_slice(&header.to_bytes());

    Ok(())
}

fn write_string(
    res: &mut Vec<u8>,
    id: u32,
    s: &str,
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        id,
        |res| {
            res.push(0x01);
            formatters::utf16_bom_le_no_nl(s, res, trap)
        },
        trap,
    )
}

fn write_artists(
    res: &mut Vec<u8>,
    id: u32,
    a: &[String],
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        id,
        |res| {
            res.push(0x01);
            res.extend([0xff, 0xfe]);

            for a in a {
                formatters::utf16_le_map(
                    a,
                    |c| match c {
                        '\n' => ' ',
                        '/' => '\\',
                        c => c,
                    },
                    res,
                    trap,
                )?;
                res.push(b'/');
            }
            res.pop();
            Ok(())
        },
        trap,
    )
}

fn write_genres(
    res: &mut Vec<u8>,
    id: u32,
    g: &[String],
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        id,
        |res| {
            res.push(0x00);

            for g in g {
                if let Some(g) = get_genre_id(g) {
                    write!(res, "({g})")?;
                    continue;
                }
                match g.as_str() {
                    "Remix" => res.extend(b"(RX)"),
                    "Cover" => res.extend(b"(CR)"),
                    _ => {
                        if g.starts_with('(') {
                            res.push(b'(');
                        }
                        formatters::iso_8859_1_no_nl(g, res, trap)?;
                        break;
                    }
                }
            }
            Ok(())
        },
        trap,
    )
}

fn write_num_of(
    res: &mut Vec<u8>,
    id: u32,
    n: u32,
    of: Option<u32>,
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        id,
        |res| {
            res.push(0x00);
            if let Some(c) = of {
                write!(res, "{n}/{c}")?;
            } else {
                write!(res, "{n}")?;
            }
            Ok(())
        },
        trap,
    )
}

fn format_ascii(
    res: &mut Vec<u8>,
    id: u32,
    v: impl Display,
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        id,
        |res| {
            res.push(0x00);
            write!(res, "{v}")?;
            Ok(())
        },
        trap,
    )
}

fn write_date(
    res: &mut Vec<u8>,
    id: u32,
    (m, d): (u8, u8),
    trap: &impl Trap,
) -> Result<()> {
    if m > 12 || d > 31 {
        return trap.error(Error::InvalidDate);
    }
    write_frame(
        res,
        id,
        |res| {
            res.push(0x00);
            write!(res, "{d:02}{m:02}")?;
            Ok(())
        },
        trap,
    )
}

fn write_time(
    res: &mut Vec<u8>,
    id: u32,
    time: Duration,
    trap: &impl Trap,
) -> Result<()> {
    let mut m = time.as_secs() / 60;
    let h = m / 60;
    m %= 60;
    if h > 24 {
        return trap.error(Error::InvalidTime);
    }
    write_frame(
        res,
        id,
        |res| {
            res.push(0x00);
            write!(res, "{h:02}{m:02}")?;
            Ok(())
        },
        trap,
    )
}

fn write_comments(
    res: &mut Vec<u8>,
    c: &[Comment],
    trap: &impl Trap,
) -> Result<()> {
    for c in c {
        write_comment(res, c, trap)?;
    }
    Ok(())
}

fn write_comment(
    res: &mut Vec<u8>,
    c: &Comment,
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        frame::COM,
        |res| {
            res.push(0x01);

            let mut lang = c.language.as_deref().unwrap_or("und");
            if lang.len() != 3 || !lang.is_ascii() {
                trap.error(Error::InvalidLanguage)?;
                lang = "und";
            }
            res.extend(lang.as_bytes());

            if let Some(desc) = c.desciption.as_deref() {
                formatters::utf16_bom_le(desc, res, trap)?;
            }
            res.extend([0; 2]);

            formatters::utf16_bom_le(&c.value, res, trap)?;
            Ok(())
        },
        trap,
    )
}

fn write_picture(
    res: &mut Vec<u8>,
    pic: &Picture,
    trap: &impl Trap,
) -> Result<()> {
    write_frame(
        res,
        frame::PIC,
        |res| {
            res.push(0x01);

            let format = pic
                .mime
                .as_deref()
                .map(mime_to_format)
                .unwrap_or("IDK".into());

            Ok(())
        },
        trap,
    )
}

fn format_to_mime(format: &str) -> Cow<'static, str> {
    match format {
        "JPG" => "image/jpeg".into(),
        "PNG" => "image/png".into(),
        "GIF" => "image/gif".into(),
        "JP2" | "J2K" | "JPF" | "J2C" | "JPC" => "image/jp2".into(),
        "JPM" => "image/jpm".into(),
        "JPX" => "image/jpx".into(),
        "HIF" => "image/heif".into(),
        "JXL" => "image/jxl".into(),
        "TIF" => "image/tiff".into(),
        "BMP" | "DIB" => "image/bmp".into(),
        "PBM" => "image/x-portable-bitmap".into(),
        "PGM" => "image/x-portable-graymap".into(),
        "PPM" => "image/x-portable-pixmap".into(),
        "PNM" => "image/x-portable-anymap".into(),
        v => format!("image/{}", v.to_ascii_lowercase()).into(),
    }
}

fn mime_to_format(mime: &str) -> Cow<'static, str> {
    match mime {
        "image/jpeg" => "JPG".into(),
        "image/png" => "PNG".into(),
        "image/gif" => "GIF".into(),
        "image/jp2" => "JP2".into(),
        "image/jpm" => "JPM".into(),
        "image/jpx" => "JPX".into(),
        "image/heif" => "HIF".into(),
        "image/jxl" => "JXL".into(),
        "image/tiff" => "TIF".into(),
        "image/bmp" => "BMP".into(),
        "image/x-portable-bitmap" => "BMP".into(),
        "image/x-portable-graymap" => "PGM".into(),
        "image/x-portable-pixmap" => "PPM".into(),
        "image/x-portable-anymap" => "PNM".into(),
        _ => {
            let Some(v) = mime.split_once('/') else {
                return "IDK".into();
            };
            if v.1.len() == 3 && v.1.is_ascii() {
                v.1.to_ascii_uppercase().into()
            } else {
                "IDK".into()
            }
        }
    }
}
