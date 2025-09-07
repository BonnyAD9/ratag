use crate::PictureKind;

/// Picture from tag.
#[derive(Debug)]
pub struct Picture {
    /// Mime type of the picture.
    pub mime: Option<String>,
    /// Description of the picture.
    pub description: Option<String>,
    /// Picture data. May be either picture or uri.
    pub data: Vec<u8>,
    /// Kind of picture.
    pub kind: PictureKind,
    /// If `true`, `data` is uri, otherwise `data` is directly image.
    pub is_uri: bool,
    /// Size of the image.
    pub size: Option<(usize, usize)>,
    /// color depth of the image.
    pub color_depth: Option<u32>,
    /// Size of palette of the image.
    pub palette_size: Option<usize>,
}

impl Picture {
    /// Create image from id3 data.
    pub fn from_id3(
        mime: String,
        description: String,
        kind: PictureKind,
        data: Vec<u8>,
        is_uri: bool,
    ) -> Self {
        Self {
            mime: Some(mime),
            description: Some(description),
            data,
            kind,
            is_uri,
            size: None,
            color_depth: None,
            palette_size: None,
        }
    }

    /// Create image only from data and kind.
    pub fn from_data(data: Vec<u8>, kind: PictureKind) -> Self {
        Self {
            mime: None,
            description: None,
            data,
            kind,
            is_uri: false,
            size: None,
            color_depth: None,
            palette_size: None,
        }
    }
}
