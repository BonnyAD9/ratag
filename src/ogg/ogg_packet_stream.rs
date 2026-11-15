use std::mem;

use crate::ogg::OggHeader;

/// Helper struct to compose ogg pages into packets.
#[derive(Debug, Default)]
pub struct OggPacketStream {
    data: Vec<u8>,
    buf: Vec<u8>,
}

pub struct OggPacketIterator<'a> {
    first: Option<&'a [u8]>,
    page: &'a [u8],
    segments: &'a [u8],
}

impl OggPacketStream {
    /// Create new [`OggPacketStream`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the next page to produce packets.
    ///
    /// Returns iterator over the packets.
    pub fn next_page<'a>(
        &'a mut self,
        header: &OggHeader,
        segments: &'a [u8],
        mut page: &'a [u8],
    ) -> OggPacketIterator<'a> {
        if segments.is_empty() {
            return OggPacketIterator::empty();
        }

        let (rem_segments, len) = unfinished_packet(segments);

        // The whole page is one unfinished packet.
        if rem_segments.is_empty() && len != 0 {
            if header.header_type.is_fresh() {
                self.data.clear();
            }
            self.data.extend(page);
            return OggPacketIterator::empty();
        }

        self.buf.clear();
        mem::swap(&mut self.buf, &mut self.data);

        // Future page will complete the last packet.
        if len != 0 {
            let idx = page.len() - len;
            self.data.extend(&page[idx..]);
            page = &page[..idx];
        }

        assert!(!rem_segments.is_empty());

        if header.header_type.is_fresh() {
            self.data.clear();
            return OggPacketIterator::new(None, page, rem_segments);
        }

        // Lets first complete the previous packet
        let (rem_segments, len, end) = next_boundary(rem_segments);
        assert!(end);
        self.buf.extend(&page[..len]);
        OggPacketIterator::new(Some(&self.buf), &page[len..], rem_segments)
    }
}

impl<'a> OggPacketIterator<'a> {
    pub fn empty() -> Self {
        Self::new(None, &[], &[])
    }

    fn new(
        first: Option<&'a [u8]>,
        page: &'a [u8],
        segments: &'a [u8],
    ) -> Self {
        Self {
            first,
            page,
            segments,
        }
    }
}

impl<'a> Iterator for OggPacketIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first) = self.first.take() {
            return Some(first);
        }
        if self.segments.is_empty() {
            return None;
        }

        let (rem_segments, len, end) = next_boundary(self.segments);
        self.segments = rem_segments;
        let data = &self.page[..len];
        self.page = &self.page[len..];

        if end { Some(data) } else { None }
    }
}

/// Returns: (remaining segments, boundary length, true if this ends boundary)
fn next_boundary(seg: &[u8]) -> (&[u8], usize, bool) {
    let mut len = 0;
    let mut seg = seg.iter();
    let mut end = false;

    for s in &mut seg {
        len += *s as usize;
        if *s == 255 {
            continue;
        }
        end = true;
        break;
    }

    (seg.as_slice(), len, end)
}

fn unfinished_packet(seg: &[u8]) -> (&[u8], usize) {
    if matches!(seg.last(), None | Some(..=254)) {
        return (seg, 0);
    }

    let mut cnt = 0;
    let mut len = 0;
    for s in seg.iter().rev() {
        if *s != 255 {
            break;
        }
        cnt += 1;
        len += *s as usize;
    }

    (&seg[..seg.len() - cnt], len)
}
