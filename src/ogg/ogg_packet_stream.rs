use crate::ogg::OggHeader;

/// Helper struct to compose ogg pages into packets.
#[derive(Debug, Default)]
pub struct OggPacketStream {
    data: Vec<u8>,
}

impl OggPacketStream {
    /// Create new [`OggPacketStream`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the next page.
    ///
    /// If this completes full packet, the packet data is returned.
    pub fn next_page<'a>(
        &'a mut self,
        header: &OggHeader,
        page: &'a [u8],
    ) -> Option<&'a [u8]> {
        if header.header_type.is_fresh() {
            self.data.clear();
        }

        if header.page_segments == 255 {
            self.data.extend(page);
            return None;
        }

        if self.data.is_empty() {
            return Some(page);
        }

        self.data.extend(page);
        Some(&self.data)
    }
}
