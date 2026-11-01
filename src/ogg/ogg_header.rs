use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
    ogg::OggHeaderType,
};

/// Header of ogg page.
#[derive(Debug, Clone, Copy)]
pub struct OggHeader {
    pub version: u8,
    pub header_type: OggHeaderType,
    pub granule_position: u64,
    pub bitstream_serial_number: u32,
    pub page_sequence_number: u32,
    pub crc_checksum: u32,
    pub page_segments: u8,
}

impl OggHeader {
    /// Parse ogg page header from bytes. Note that this doesn't parse the
    /// segment lengths.
    pub fn from_bytes(d: &[u8; 23]) -> Self {
        Self {
            version: d[0],
            header_type: OggHeaderType::from_bits_retain(d[1]),
            granule_position: u64::from_le_bytes(d[2..10].try_into().unwrap()),
            bitstream_serial_number: u32::from_le_bytes(
                d[10..14].try_into().unwrap(),
            ),
            page_sequence_number: u32::from_le_bytes(
                d[14..18].try_into().unwrap(),
            ),
            crc_checksum: u32::from_le_bytes(d[18..22].try_into().unwrap()),
            page_segments: d[22],
        }
    }
}

impl<R: BufRead> Breadable<R> for OggHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
