use crate::Popularimeter;

/// Rating of a song.
#[derive(Debug, Clone)]
pub enum Rating {
    /// Rating given by text.
    Text(String),
    /// Rating given by popularimeter.
    Popularimeter(Popularimeter),
}
