use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
    parsers,
};

pub struct Header {
    pub major_version: u8,
    pub _minor_version: u8,
    flags: u8,
    pub size: u32,
}

impl Header {
    const UNSYNCHRONIZATION: u8 = 0x80;
    const COMPRESSION2: u8 = 0x40;
    const EXTENDED_HEADER34: u8 = 0x40;

    pub const MAJOR_VERSION2: u8 = 0x02;
    pub const MAJOR_VERSION3: u8 = 0x03;
    pub const MAJOR_VERSION4: u8 = 0x04;

    pub fn from_bytes(d: &[u8; 7]) -> Self {
        Self {
            major_version: d[0],
            _minor_version: d[1],
            flags: d[3],
            size: parsers::syncsafe_be_u32(d[3..].try_into().unwrap()),
        }
    }

    pub fn unsynchronization(&self) -> bool {
        self.get_flag(Self::UNSYNCHRONIZATION)
    }

    pub fn extended_header34(&self) -> bool {
        self.get_flag(Self::EXTENDED_HEADER34)
    }

    pub fn compression2(&self) -> bool {
        self.get_flag(Self::COMPRESSION2)
    }

    fn get_flag(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }
}

impl<R: BufRead> Breadable<R> for Header {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
