use std::{path::Path, time::Duration};

use crate::{DataType, Result, TagStore, read_tag_from_file, trap};

/// Tag storage with basic information.
#[derive(Debug, Default)]
pub struct BasicTag {
    /// Title of the song.
    pub title: Option<String>,
    /// Albumb of the song.
    pub album: Option<String>,
    /// Artists in this song.
    pub artists: Vec<String>,
    /// Genres of this song.
    pub genres: Vec<String>,
    /// Track number within the album.
    pub track: Option<u32>,
    /// Year of release of the song.
    pub year: Option<u32>,
    /// Disc number.
    pub disc: Option<u32>,
    /// Length of the song.
    pub length: Option<Duration>,
}

impl BasicTag {
    /// Reads the basic tag from file. Supports all tags supported by this
    /// crate. Recoverable errors are skipped.
    pub fn from_file(f: impl AsRef<Path>) -> Result<Box<Self>> {
        let mut res = Box::new(Self::default());
        read_tag_from_file(f, &mut *res, &trap::Skip)?;
        Ok(res)
    }
}

impl TagStore for BasicTag {
    fn stores_data(&self, typ: DataType) -> bool {
        use DataType::*;
        matches!(
            typ,
            Title | Album | Artists | Genres | Track | Year | Disc | Length
        )
    }

    fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    fn set_album(&mut self, album: Option<String>) {
        self.album = album;
    }

    fn set_artists(&mut self, artists: Vec<String>) {
        self.artists = artists;
    }

    fn set_genres(&mut self, genres: Vec<String>) {
        self.genres = genres;
    }

    fn set_track(&mut self, track: Option<u32>) {
        self.track = track;
    }

    fn set_year(&mut self, year: Option<u32>) {
        self.year = year;
    }

    fn set_disc(&mut self, disc: Option<u32>) {
        self.disc = disc;
    }

    fn set_length(&mut self, length: Option<Duration>) {
        self.length = length;
    }
}
