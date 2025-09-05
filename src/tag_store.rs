use std::time::Duration;

use crate::{Comment, DataType};

/// Generic storage for data from tag.
#[allow(unused_variables)]
pub trait TagStore {
    /// Check whether the given data type is stored by this storage.
    fn stores_data(&self, typ: DataType) -> bool;

    /// If all data has been read, this can return `true` to stop unnecesary
    /// reading of further data.
    fn done(&self) -> bool {
        false
    }

    /// Set the title of the track.
    fn set_title(&mut self, title: Option<String>) {}

    /// Set the album in which the song is.
    fn set_album(&mut self, album: Option<String>) {}

    /// Artists within the song.
    fn set_artists(&mut self, artists: Vec<String>) {}

    /// Genres of the song.
    fn set_genres(&mut self, genres: Vec<String>) {}

    /// Set track number of the song within album.
    fn set_track(&mut self, track: Option<u32>) {}

    /// Set total number of tracks within album.
    fn set_track_count(&mut self, cnt: Option<u32>) {}

    /// Set year of release of the song. Note that in some cases `set_date` is
    /// called but `set_year` not.
    fn set_year(&mut self, year: Option<u32>) {}

    /// Set the date of release of the song.
    fn set_date(&mut self, month_day: Option<(u32, u32)>) {}

    /// Set disc number.
    fn set_disc(&mut self, disc: Option<u32>) {}

    /// Set the total number of disc.
    fn set_disc_count(&mut self, cnt: Option<u32>) {}

    /// Set the length of the track.
    fn set_length(&mut self, length: Option<Duration>) {}

    /// Set the comments.
    fn set_comments(&mut self, comments: Vec<Comment>) {}
}
