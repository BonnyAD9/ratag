use bitflags::bitflags;

bitflags! {
    /// Ogg header flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OggHeaderType: u8 {
        /// This page doesn't start a new packet, but continues the previous
        /// packet.
        const CONTINUATION = 0x01;
        /// This is the first page of the stream.
        const BEGINNING_OF_STREAM = 0x02;
        /// This is the last page of the stream.
        const END_OF_STREAM = 0x04;
    }
}

impl OggHeaderType {
    /// Check if this page is continuation of the previous page.
    pub fn is_continuation(&self) -> bool {
        self.contains(Self::CONTINUATION)
    }

    /// Check if this page starts a new packet.
    pub fn is_fresh(&self) -> bool {
        !self.is_continuation()
    }

    /// Check if this page is the beginning of a new stream.
    pub fn is_bos(&self) -> bool {
        self.contains(Self::BEGINNING_OF_STREAM)
    }

    /// Check if this page is the end of stream.
    pub fn is_eos(&self) -> bool {
        self.contains(Self::END_OF_STREAM)
    }
}
