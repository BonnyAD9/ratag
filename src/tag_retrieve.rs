use std::time::Duration;

use crate::{Comment, Rating, containers::Picture};

/// The opposite of [`crate::TagStore`]. Trait for tag containers that can
/// provide their fields, usually to be written.
pub trait TagRetrieve {
    /// Get the song title.
    fn get_title(&self) -> Option<&str> {
        None
    }

    /// Get the album name.
    fn get_album(&self) -> Option<&str> {
        None
    }

    /// Get all the involved artists.
    fn get_artists(&self) -> &[String] {
        &[]
    }

    /// Get the album artist.
    fn get_album_artist(&self) -> Option<&str> {
        None
    }

    /// Get all the genres.
    fn get_genres(&self) -> &[String] {
        &[]
    }

    /// Get the track number.
    fn get_track(&self) -> Option<u32> {
        None
    }

    /// Get the total number of tracks in the album.
    fn get_track_count(&self) -> Option<u32> {
        None
    }

    /// Get the year of release.
    fn get_year(&self) -> Option<i32> {
        None
    }

    /// Get the date of release.
    fn get_date(&self) -> Option<(u8, u8)> {
        None
    }

    /// Get the time of release.
    fn get_time(&self) -> Option<Duration> {
        None
    }

    /// Get the disc number.
    fn get_disc(&self) -> Option<u32> {
        None
    }

    /// Get the disc count.
    fn get_disc_count(&self) -> Option<u32> {
        None
    }

    /// Get the length of the song.
    fn get_length(&self) -> Option<Duration> {
        None
    }

    /// Get the comments associated with the song.
    fn get_comments(&self) -> &[Comment] {
        &[]
    }

    /// Get the pictures associated to the song.
    fn get_pictures(&self) -> &[Picture] {
        &[]
    }

    /// Get the copyright notice.
    fn get_copyright(&self) -> Option<&str> {
        None
    }

    /// Get the ratings of the song.
    fn get_ratings(&self) -> &[Rating] {
        &[]
    }
}
