use std::{
    fs::File,
    io::{ErrorKind, Read, Seek, SeekFrom},
    path::Path,
};

use crate::{
    Comment, Error, Result, TagStore, id3::genres::get_genre, parsers,
    trap::Trap,
};

/// Data stored in ID3v1 tag.
#[derive(Debug)]
pub struct Id3v1Tag {
    /// Title of the song.
    pub title: String,
    /// Artist of the song.
    pub artist: String,
    /// Album name.
    pub album: String,
    /// Year of release.
    pub year: Option<u16>,
    /// Comment.
    pub comment: String,
    /// Genre.
    pub genre: u8,
    /// Track number within the album.
    pub track: Option<u8>,
    /// Additional genre specification.
    pub sub_genre: Option<String>,
}

impl Id3v1Tag {
    const LEN0: usize = 128;
    const LEN2: usize = 256;

    /// Read ID3v1 tag from file.
    pub fn from_file(p: impl AsRef<Path>, trap: &impl Trap) -> Result<Self> {
        Self::from_seek(File::open(p)?, trap)
    }

    /// Read ID3v1 tag from reader without assuming that it is at the proper
    /// position.
    pub fn from_seek(
        mut read: impl Seek + Read,
        trap: &impl Trap,
    ) -> Result<Self> {
        read.seek(SeekFrom::End(-(Self::LEN2 as i64)))?;
        Self::from_read(read, trap)
    }

    /// Read ID3v1 tag from reader, assuming that it is already at the proper
    /// position.
    pub fn from_read(read: impl Read, trap: &impl Trap) -> Result<Self> {
        let mut buf = [0; Self::LEN2];
        let len = try_read_exact(read, &mut buf)?;
        match len {
            ..Self::LEN0 => Err(Error::Io(ErrorKind::UnexpectedEof.into())),
            Self::LEN0..Self::LEN2 => {
                Self::from_bytes(&buf[len - Self::LEN0..len], trap)
            }
            Self::LEN2 => Self::from_bytes(&buf, trap),
            _ => unreachable!(),
        }
    }

    /// Parse ID3v1 tag from read data. Data must be either 128 or 256 bytes
    /// long.
    pub fn from_bytes(data: &[u8], trap: &impl Trap) -> Result<Self> {
        let (v11, v12) = match data.len() {
            Self::LEN0 => (data, None),
            Self::LEN2 => (&data[128..], Some(&data[..128])),
            _ => return Err(Error::InvalidLength),
        };

        if !v11.starts_with(b"TAG") {
            return Err(Error::NoTag);
        }

        // id3v1.0
        let title = parsers::iso_8859_1_mnt(&v11[3..33], trap)?.1;
        let artist = parsers::iso_8859_1_mnt(&v11[33..63], trap)?.1;
        let album = parsers::iso_8859_1_mnt(&v11[63..93], trap)?.1;
        let year = parse_year(v11[93..97].try_into().unwrap());
        let comment = parsers::iso_8859_1_mnt(&v11[97..127], trap)?.1;
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

        // id3v1.2
        parsers::iso_8859_1_mnt_to(&mut res.title, &v12[3..33], trap)?;
        parsers::iso_8859_1_mnt_to(&mut res.artist, &v12[33..63], trap)?;
        parsers::iso_8859_1_mnt_to(&mut res.album, &v12[63..93], trap)?;
        parsers::iso_8859_1_mnt_to(&mut res.comment, &v12[93..108], trap)?;
        res.sub_genre = Some(parsers::iso_8859_1_mnt(&v12[107..128], trap)?.1);

        Ok(res)
    }

    /// Store the ID3v1 data into a tag storage.
    pub fn store(
        self,
        store: &mut impl TagStore,
        trap: &impl Trap,
    ) -> Result<()> {
        if !self.title.is_empty() {
            store.set_title(Some(self.title));
        }
        if !self.artist.is_empty() {
            store.set_artists(vec![self.artist]);
        }
        if !self.album.is_empty() {
            store.set_album(Some(self.album));
        }
        if let Some(y) = self.year {
            store.set_year(Some(y as i32));
        }
        if !self.comment.is_empty() {
            store.set_comments(vec![Comment::from_value(self.comment)]);
        }

        let mut genres = vec![];
        if let Some(g) = get_genre(self.genre) {
            genres.push(g.to_string());
        } else if self.genre != 255 {
            trap.error(Error::InvalidGenreRef)?;
        }
        genres.extend(self.sub_genre);
        if !genres.is_empty() {
            store.set_genres(genres);
        }

        if let Some(t) = self.track {
            store.set_track(Some(t as u32));
        }

        Ok(())
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
