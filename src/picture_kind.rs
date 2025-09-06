bitflags::bitflags! {
    #[doc = "Kind of picture."]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct PictureKind: u32 {
        #[doc = "Any other kind, or unknown."]
        const OTHER = 0x1;
        #[doc = "32x32 pixels 'file icon'."]
        const ICON32 = 0x2;
        #[doc = "Other file icon."]
        const OTHER_ICON = 0x4;
        #[doc = "Cover (front)."]
        const FRONT_COVER = 0x8;
        #[doc = "Cover (back)."]
        const BACK_COVER = 0x10;
        #[doc = "Leaflet pate."]
        const LEAFLET_PAGE = 0x20;
        #[doc = "media (e.g. lable side of CD)."]
        const MEDIA = 0x40;
        #[doc = "Lead artist/lead performer/soloist."]
        const LEAD_ARTIST = 0x80;
        #[doc = "Artist/performer"]
        const ARTIST = 0x100;
        #[doc = "Conductor."]
        const CONDUCTOR = 0x200;
        #[doc = "Band/Orchestra."]
        const BAND = 0x400;
        #[doc = "Composer."]
        const COMPOSER = 0x800;
        #[doc = "Lyricist/text writer."]
        const LYRICIST = 0x1000;
        #[doc = "Recording Location."]
        const RECORDING_LOCATION = 0x2000;
        #[doc = "During recording."]
        const DURING_RECORDING = 0x4000;
        #[doc = "During performance."]
        const DURING_PERFORMANCE = 0x8000;
        #[doc = "Movie/video screen capture."]
        const MOVIE_CAPTURE = 0x1_0000;
        #[doc = "A bright coloured fish."]
        const BRIGHT_COLOURED_FISH = 0x2_0000;
        #[doc = "Illustration."]
        const ILLUSTRATION = 0x4_0000;
        #[doc = "Band/artist logotype."]
        const ARTIST_LOGOTYPE = 0x8_0000;
        #[doc = "Publisher/studio logotype."]
        const PUBLISHER_LOGOTYPE = 0x10_0000;
    }
}

impl Default for PictureKind {
    fn default() -> Self {
        Self::OTHER
    }
}

impl PictureKind {
    pub(crate) fn from_id3(t: u8) -> Option<PictureKind> {
        match t {
            0x00 => Some(Self::OTHER),
            0x01 => Some(Self::ICON32),
            0x02 => Some(Self::OTHER_ICON),
            0x03 => Some(Self::FRONT_COVER),
            0x04 => Some(Self::BACK_COVER),
            0x05 => Some(Self::LEAFLET_PAGE),
            0x06 => Some(Self::MEDIA),
            0x07 => Some(Self::LEAD_ARTIST),
            0x08 => Some(Self::ARTIST),
            0x09 => Some(Self::CONDUCTOR),
            0x0A => Some(Self::BAND),
            0x0B => Some(Self::COMPOSER),
            0x0C => Some(Self::LYRICIST),
            0x0D => Some(Self::RECORDING_LOCATION),
            0x0E => Some(Self::DURING_RECORDING),
            0x0F => Some(Self::DURING_PERFORMANCE),
            0x10 => Some(Self::MOVIE_CAPTURE),
            0x11 => Some(Self::BRIGHT_COLOURED_FISH),
            0x12 => Some(Self::ILLUSTRATION),
            0x13 => Some(Self::ARTIST_LOGOTYPE),
            0x14 => Some(Self::PUBLISHER_LOGOTYPE),
            _ => None,
        }
    }

    pub(crate) fn all_id3() -> Self {
        Self::from_bits_retain(0x1f_ffff)
    }
}
