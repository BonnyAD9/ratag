use std::io::Cursor;

use crate::{
    Error, TagStore, TagType,
    bread::Bread,
    ogg::{OggHeader, OggPacketStream, OggStream},
    trap::Trap,
    vorbis::identification_header::IdentificationHeader,
    vorbis_comment,
};

#[derive(Debug, Default)]
pub struct OggVorbisStream {
    stream: OggPacketStream,
}

impl OggVorbisStream {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: TagStore, T: Trap> OggStream<S, T> for OggVorbisStream {
    fn read_page(
        &mut self,
        header: &OggHeader,
        page: &[u8],
        store: &mut S,
        trap: &T,
    ) -> crate::Result<bool> {
        let Some(packet) = self.stream.next_page(header, page) else {
            return Ok(true);
        };

        if packet.len() < 7 {
            return Err(Error::OggVorbisPacketTooShort);
        }

        if &packet[1..7] != b"vorbis" {
            return Err(Error::OggPacketNotVorbis);
        }

        let mut r = Bread::new(Cursor::new(&packet[7..]));

        // The identification header.
        if packet[0] == 1 {
            store.set_tag_type(TagType::Vorbis);
            let header: IdentificationHeader = r.get()?;
            if header.version != 0 {
                return Err(Error::Unsupported(
                    "Vorbis version other than 1.",
                ));
            }
            if !header.framing_bit {
                trap.error(Error::VorbisFramingBitUnset)?;
            }
            return Ok(true);
        }

        // The comment header.
        if packet[0] != 3 {
            return Ok(false);
        }

        vorbis_comment::from_bread(&mut r, store, trap, true)?;

        // The tag is red.
        Ok(false)
    }
}
