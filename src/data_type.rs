use crate::PictureKind;

/// Type of data that can be within tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// Song title.
    Title,
    /// Album in which the song is.
    Album,
    /// Artists in the song.
    Artists,
    /// Genres of the song.
    Genres,
    /// Track number of the song within album.
    Track,
    /// Total number of tracks in the album.
    TrackCount,
    /// Year of release of the song.
    Year,
    /// Month and day of month of release of the song.
    Date,
    /// Time of release of the song.
    Time,
    /// Disc on which the song is.
    Disc,
    /// Total number of discs.
    DiscCount,
    /// Length of the song.
    Length,
    /// Additional comments.
    Comments,
    /// Picture.
    Picture(PictureKind),
}
