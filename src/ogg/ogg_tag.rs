use crate::ogg::{OggHeader, OggStream};

/// Generic ogg codec for reading tags.
pub trait OggTag<S, T> {
    /// This will be given the first page of the stream. If the packet is
    /// recognised, this will return stream that is capable of handling the
    /// ogg stream.
    fn get_stream(
        &self,
        header: &OggHeader,
        page: &[u8],
    ) -> Option<Box<dyn OggStream<S, T>>>;
}
