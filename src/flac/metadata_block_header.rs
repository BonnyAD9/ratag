use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

pub struct MetadataBlockHeader {
    pub last: bool,
    pub block_type: u8,
    pub length: u32,
}

impl MetadataBlockHeader {
    pub const STREAMINFO: u8 = 0;
    pub const VORBISCOMMENT: u8 = 4;
    pub const PICTURE: u8 = 6;

    pub fn from_bytes(d: &[u8; 4]) -> Self {
        let last = (d[0] & 0x80) == 0x80;
        let block_type = d[0] & 0x7f;
        let length = u32::from_be_bytes(*d) & 0xFF_FFFF;
        Self {
            last,
            block_type,
            length,
        }
    }
}

impl<R: BufRead> Breadable<R> for MetadataBlockHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
