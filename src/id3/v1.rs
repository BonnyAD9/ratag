use std::{
    fs::File,
    io::{ErrorKind, Read, Seek, SeekFrom},
    path::Path,
};

use encoding::{DecoderTrap, Encoding, all::ISO_8859_1};

use crate::{Error, Result};

#[derive(Debug)]
pub struct Id3v1 {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: Option<u16>,
    pub comment: String,
    pub genre: u8,
    pub track: Option<u8>,
    pub sub_genre: Option<String>,
}

impl Id3v1 {
    const LEN0: usize = 128;
    const LEN2: usize = 256;

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let (v11, v12) = match data.len() {
            Self::LEN0 => (data, None),
            Self::LEN2 => (&data[128..], Some(&data[..128])),
            _ => return Err(Error::InvalidLength),
        };

        dbg!(v11);

        if !v11.starts_with(b"TAG") {
            return Err(Error::NoTag);
        }

        fn decode(data: &[u8]) -> String {
            let end = data.iter().position(|c| *c == 0).unwrap_or(data.len());
            ISO_8859_1
                .decode(&data[..end], DecoderTrap::Replace)
                .unwrap()
        }

        // id3v1.0
        let title = decode(&v11[3..33]);
        let artist = decode(&v11[33..63]);
        let album = decode(&v11[63..93]);
        let year = parse_year(v11[93..97].try_into().unwrap());
        let comment = decode(&v11[97..127]);
        let genre = v11[127];

        // id3v1.1
        let track = (v11[125] == 0 && v11[126] != 0).then_some(v11[126]);

        let mut res = Self {
            title,
            artist,
            album,
            year,
            comment,
            genre,
            track,
            sub_genre: None,
        };

        let v12 = if let Some(v12) = v12
            && v12.starts_with(b"EXT")
        {
            v12
        } else {
            return Ok(res);
        };

        fn decode_to(out: &mut String, data: &[u8]) {
            let end = data.iter().position(|c| *c == 0).unwrap_or(data.len());
            _ = ISO_8859_1.decode_to(&data[..end], DecoderTrap::Replace, out);
        }

        // id3v1.2
        decode_to(&mut res.title, &v12[3..33]);
        decode_to(&mut res.artist, &v12[33..63]);
        decode_to(&mut res.album, &v12[63..93]);
        decode_to(&mut res.comment, &v12[93..108]);
        res.sub_genre = Some(decode(&v12[107..128]));

        Ok(res)
    }

    pub fn from_read(read: impl Read) -> Result<Self> {
        let mut buf = [0; Self::LEN2];
        let len = try_read_exact(read, &mut buf)?;
        match len {
            ..Self::LEN0 => Err(Error::Io(ErrorKind::UnexpectedEof.into())),
            Self::LEN0..Self::LEN2 => {
                Self::from_bytes(&buf[len - Self::LEN0..len])
            }
            Self::LEN2 => Self::from_bytes(&buf),
            _ => unreachable!(),
        }
    }

    pub fn from_seek(mut read: impl Seek + Read) -> Result<Self> {
        read.seek(SeekFrom::End(-(Self::LEN2 as i64)))?;
        Self::from_read(read)
    }

    pub fn from_file(p: impl AsRef<Path>) -> Result<Self> {
        Self::from_seek(File::open(p)?)
    }
}

fn parse_year(s: [u8; 4]) -> Option<u16> {
    fn digit(c: u8) -> Option<u16> {
        c.is_ascii_digit().then(|| (c - b'0') as u16)
    }

    Some(
        digit(s[0])? * 1000
            + digit(s[1])? * 100
            + digit(s[2])? * 10
            + digit(s[3])?,
    )
}

fn try_read_exact(mut r: impl Read, buf: &mut [u8]) -> Result<usize> {
    let mut pos = 0;
    while pos < buf.len() {
        match r.read(&mut buf[pos..]) {
            Ok(0) => return Ok(pos),
            Ok(s) => pos += s,
            Err(e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(Error::Io(e)),
        }
    }

    Ok(pos)
}
