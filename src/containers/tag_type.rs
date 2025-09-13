/// The type of tag.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TagType {
    /// ID3v1.0, ID3v1.1, ID3v1.2 (usually combined with [`TagType::ID3v2`])
    Id3v1(u8),
    /// ID3v2.2, ID3v2.3, ID3v2.4
    Id3v2(u8),
    /// Flac (usually combined with [`TagType::VorbisComment`])
    Flac,
    /// Mp4
    Mp4,
    /// Asf
    Asf,
    /// Riff (WAVE -> [`crate::riff::chunk::WAVE`])
    Riff(u32),
    /// Vorbis comment. (usually combined with [`TagType::Flac`])
    VorbisComment,
    /// Other tag format (by some foregin tag reader)
    Other(&'static str),
}
