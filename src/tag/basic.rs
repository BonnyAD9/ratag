use std::{path::Path, time::Duration};

use crate::{DataType, Result, TagStore, read_tag_from_file, trap};

/// Tag storage with basic information.
#[derive(Debug, Default)]
pub struct Basic {
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
    pub year: Option<i32>,
    /// Disc number.
    pub disc: Option<u32>,
    /// Length of the song.
    pub length: Option<Duration>,
}

impl Basic {
    /// Reads the basic tag from file. Supports all tags supported by this
    /// crate. Recoverable errors are skipped.
    pub fn from_file(f: impl AsRef<Path>) -> Result<Box<Self>> {
        let mut res = Box::new(Self::default());
        read_tag_from_file(f, &mut *res, &trap::Skip)?;
        Ok(res)
    }
}

impl TagStore for Basic {
    fn stores_data(&self, typ: DataType) -> bool {
        use DataType::*;
        matches!(
            typ,
            Title | Album | Artists | Genres | Track | Year | Disc | Length
        )
    }

    fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    fn set_album(&mut self, album: String) {
        self.album = Some(album);
    }

    fn set_artists(&mut self, artists: Vec<String>) {
        self.artists = artists;
    }

    fn set_genres(&mut self, genres: Vec<String>) {
        self.genres = genres;
    }

    fn set_track(&mut self, track: u32) {
        self.track = Some(track);
    }

    fn set_year(&mut self, year: i32) {
        self.year = Some(year);
    }

    fn set_disc(&mut self, disc: u32) {
        self.disc = Some(disc);
    }

    fn set_length(&mut self, length: Duration) {
        self.length = Some(length);
    }
}
