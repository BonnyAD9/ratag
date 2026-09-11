use std::{
    fs::File,
    io::{ErrorKind, Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{
    Comment, Error, Result, TagRetrieve, TagStore, TagType, formatters,
    id3::{
        genres::{GENRE_OTHER, get_genre, get_genre_id},
        v1::part_writer::PartWriter,
    },
    parsers,
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
    /// The length of ID3v1 and ID3v1.1 tag.
    pub const LEN0: usize = 128;
    /// The length of ID3v1.2 tag.
    pub const LEN2: usize = 256;

    /// Read ID3v1 tag from file.
    pub fn from_file(p: impl AsRef<Path>, trap: &impl Trap) -> Result<Self> {
        Self::from_seek(File::open(p)?, trap)
    }

    /// Read ID3v1 tag from reader without assuming that it is at the proper
    /// position.
    pub fn from_seek(
        read: impl Seek + Read,
        trap: &impl Trap,
    ) -> Result<Self> {
        Self::from_seek_ver(read, trap).map(|a| a.0)
    }

    /// Read ID3v1 tag from reader without assuming that it is at the proper
    /// position.
    ///
    /// Returns the tag and its version.
    pub fn from_seek_ver(
        mut read: impl Seek + Read,
        trap: &impl Trap,
    ) -> Result<(Self, u8)> {
        let len = read.seek(SeekFrom::End(0))?;
        read.seek(SeekFrom::End(-(len.min(Self::LEN2 as u64) as i64)))?;
        Self::from_read_ver(read, trap)
    }

    /// Read ID3v1 tag from reader, assuming that it is already at the proper
    /// position.
    pub fn from_read(read: impl Read, trap: &impl Trap) -> Result<Self> {
        Self::from_read_ver(read, trap).map(|a| a.0)
    }

    /// Read ID3v1 tag from reader, assuming that it is already at the proper
    /// position.
    ///
    /// Returns the tag and its version.
    pub fn from_read_ver(
        read: impl Read,
        trap: &impl Trap,
    ) -> Result<(Self, u8)> {
        let mut buf = [0; Self::LEN2];
        let len = try_read_exact(read, &mut buf)?;
        match len {
            ..Self::LEN0 => Err(Error::NoTag),
            Self::LEN0..Self::LEN2 => {
                Self::from_bytes_ver(&buf[len - Self::LEN0..len], trap)
            }
            Self::LEN2 => Self::from_bytes_ver(&buf, trap),
            _ => unreachable!(),
        }
    }

    /// Read ID3v1 tag from reader, assuming that it is already at the proper
    /// position.
    ///
    /// Returns the tag and its version.
    pub fn try_from_seek_ver(
        read: impl Read + Seek,
        trap: &impl Trap,
    ) -> Result<Option<(Self, u8)>> {
        match Self::from_seek_ver(read, trap) {
            Ok(r) => Ok(Some(r)),
            Err(Error::NoTag) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Parse ID3v1 tag from read data. Data must be either 128 or 256 bytes
    /// long.
    pub fn from_bytes(data: &[u8], trap: &impl Trap) -> Result<Self> {
        Self::from_bytes_ver(data, trap).map(|a| a.0)
    }

    /// Parse ID3v1 tag from read data. Data must be either 128 or 256 bytes
    /// long.
    ///
    /// Returns the tag and its version.
    pub fn from_bytes_ver(
        data: &[u8],
        trap: &impl Trap,
    ) -> Result<(Self, u8)> {
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
            return Ok((res, if track.is_some() { 1 } else { 0 }));
        };

        // id3v1.2
        parsers::iso_8859_1_mnt_to(&mut res.title, &v12[3..33], trap)?;
        parsers::iso_8859_1_mnt_to(&mut res.artist, &v12[33..63], trap)?;
        parsers::iso_8859_1_mnt_to(&mut res.album, &v12[63..93], trap)?;
        parsers::iso_8859_1_mnt_to(&mut res.comment, &v12[93..108], trap)?;
        res.sub_genre = Some(parsers::iso_8859_1_mnt(&v12[107..128], trap)?.1);

        Ok((res, 2))
    }

    /// Store the ID3v1 data into a tag storage.
    pub fn store(
        self,
        store: &mut impl TagStore,
        trap: &impl Trap,
    ) -> Result<()> {
        if self.sub_genre.is_some() {
            store.set_tag_type(TagType::Id3v1(2));
        } else if self.track.is_some() {
            store.set_tag_type(TagType::Id3v1(1));
        } else {
            store.set_tag_type(TagType::Id3v1(0));
        }

        if !self.title.is_empty() {
            store.set_title(self.title);
        }
        if !self.artist.is_empty() {
            store.set_artists(vec![self.artist]);
        }
        if !self.album.is_empty() {
            store.set_album(self.album);
        }
        if let Some(y) = self.year {
            store.set_year(y as i32);
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
            store.set_track(t as u32);
        }

        Ok(())
    }

    /// Copy all supported fields from `ret` to this tag.
    ///
    /// If `ret` returns none, it will not replace the value in this tag.
    pub fn retrieve(&mut self, ret: &impl TagRetrieve) {
        if let Some(title) = ret.get_title() {
            self.title.clear();
            self.title += title;
        }

        if let Some(artist) = ret.get_artists().first() {
            self.artist.clear();
            self.artist += artist;
        }

        if let Some(album) = ret.get_album() {
            self.album.clear();
            self.album += album;
        }

        if let Some(year) = ret.get_year() {
            self.year = year.try_into().ok();
        }

        if let Some(comment) = ret.get_comments().first() {
            self.comment.clear();
            self.comment += &comment.value;
        }

        let mut sub_genre = None;
        let mut genre = None;

        for g in ret.get_genres() {
            if genre.is_none()
                && let Some(id) = get_genre_id(g)
            {
                genre = Some(id);
            } else if sub_genre.is_none() {
                let mut buf = self.sub_genre.take().unwrap_or_default();
                buf += g;
                sub_genre = Some(buf);
            }
            if sub_genre.is_some() && genre.is_some() {
                break;
            }
        }

        if sub_genre.is_some() || genre.is_some() {
            self.genre = genre.unwrap_or(GENRE_OTHER);
            self.sub_genre = sub_genre;
        }

        if let Some(track) = ret.get_track() {
            self.track = track.try_into().ok();
        }
    }

    /// Retrieve all unset fields from `ret`.
    pub fn retrieve_fill(&mut self, ret: &impl TagRetrieve) {
        if self.title.is_empty()
            && let Some(title) = ret.get_title()
        {
            self.title.clear();
            self.title += title;
        }

        if self.artist.is_empty()
            && let Some(artist) = ret.get_artists().first()
        {
            self.artist.clear();
            self.artist += artist;
        }

        if self.album.is_empty()
            && let Some(album) = ret.get_album()
        {
            self.album.clear();
            self.album += album;
        }

        if self.year.is_none()
            && let Some(year) = ret.get_year()
        {
            self.year = year.try_into().ok();
        }

        if self.comment.is_empty()
            && let Some(comment) = ret.get_comments().first()
        {
            self.comment.clear();
            self.comment += &comment.value;
        }

        let mut sub_genre = None;
        let mut genre = None;

        for g in ret.get_genres() {
            if genre.is_none()
                && let Some(id) = get_genre_id(g)
            {
                genre = Some(id);
            } else if sub_genre.is_none() {
                let mut buf = self.sub_genre.take().unwrap_or_default();
                buf += g;
                sub_genre = Some(buf);
            }
            if sub_genre.is_some() && genre.is_some() {
                break;
            }
        }

        if sub_genre.is_some() || genre.is_some() {
            if self.genre == GENRE_OTHER {
                self.genre = genre.unwrap_or(GENRE_OTHER);
            }
            if self.sub_genre.is_none() {
                self.sub_genre = sub_genre;
            }
        }

        if self.track.is_none()
            && let Some(track) = ret.get_track()
        {
            self.track = track.try_into().ok();
        }
    }

    /// Write this tag to the given stream. This does not seek or check
    /// anything in the stream.
    ///
    /// `version` specifies which tag version should be used. If it is `None`,
    /// it is chosen automatically.
    ///
    /// # Returns
    /// The version that has been written.
    pub fn write_to(
        &self,
        mut write: impl Write,
        version: Option<u8>,
        trap: &impl Trap,
    ) -> Result<u8> {
        let mut buf = [0; Self::LEN2];
        let (v2, v1) = buf.split_at_mut(128);
        v1[..3].copy_from_slice(b"TAG");
        v2[..3].copy_from_slice(b"EXT");

        let mut aext = false;

        let mut pw = PartWriter::new(&mut v1[3..33], &mut v2[3..33]);
        formatters::iso_8859_1(&self.title, &mut pw, trap)?;
        aext |= pw.pos() > 30;

        pw = PartWriter::new(&mut v1[33..63], &mut v2[33..63]);
        formatters::iso_8859_1(&self.artist, &mut pw, trap)?;
        aext |= pw.pos() > 30;

        pw = PartWriter::new(&mut v1[63..93], &mut v2[63..93]);
        formatters::iso_8859_1(&self.album, &mut pw, trap)?;
        aext |= pw.pos() > 30;

        v1[93..97].copy_from_slice(&format_year(self.year, trap)?);

        let at = if version.is_none_or(|v| v > 0)
            && let Some(t) = self.track
        {
            v1[126] = t;
            pw = PartWriter::new(&mut v1[97..125], &mut v2[93..108]);
            formatters::iso_8859_1(&self.comment, &mut pw, trap)?;
            aext |= pw.pos() > 28;
            true
        } else {
            pw = PartWriter::new(&mut v1[97..127], &mut v2[93..108]);
            formatters::iso_8859_1(&self.comment, &mut pw, trap)?;
            aext |= pw.pos() > 30;
            false
        };

        if let Some(sg) = &self.sub_genre {
            pw = PartWriter::new(&mut v2[107..127], &mut []);
            formatters::iso_8859_1(sg, &mut pw, trap)?;
        }

        v1[127] = self.genre;

        let ext = version.map(|v| v >= 2).unwrap_or(aext);
        if ext {
            write.write_all(&buf)?;
            Ok(2)
        } else {
            write.write_all(v1)?;
            Ok(if at { 1 } else { 0 })
        }
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

fn format_year(y: Option<u16>, trap: &impl Trap) -> Result<[u8; 4]> {
    fn digit(y: &mut u16) -> u8 {
        let res = *y % 10;
        *y /= 10;
        res as u8 + b'0'
    }

    let Some(mut y) = y else {
        return Ok([0; 4]);
    };

    if y > 9999 {
        trap.error(Error::OutOfRange("year"))?;
        return Ok([0; 4]);
    }

    let mut res = [digit(&mut y), digit(&mut y), digit(&mut y), digit(&mut y)];
    res.reverse();
    Ok(res)
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

impl Default for Id3v1Tag {
    fn default() -> Self {
        Self {
            title: Default::default(),
            artist: Default::default(),
            album: Default::default(),
            year: Default::default(),
            comment: Default::default(),
            genre: GENRE_OTHER,
            track: Default::default(),
            sub_genre: Default::default(),
        }
    }
}
