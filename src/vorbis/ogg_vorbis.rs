use crate::{
    TagStore,
    ogg::{OggHeader, OggStream, OggTag},
    trap::Trap,
    vorbis::ogg_vorbis_stream::OggVorbisStream,
};

/// Vorbis codec for Ogg tag.
#[derive(Debug, Clone, Copy)]
pub struct OggVorbis;

impl<S: TagStore, T: Trap> OggTag<S, T> for OggVorbis {
    fn get_stream(
        &self,
        _: &OggHeader,
        page: &[u8],
    ) -> Option<Box<dyn OggStream<S, T>>> {
        (page.len() >= 7 && &page[1..7] == b"vorbis").then(|| {
            Box::new(OggVorbisStream::new()) as Box<dyn OggStream<S, T>>
        })
    }
}
