use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
    parsers,
};

pub struct FrameHeader {
    pub id: u32,
    pub size: u32,
    pub flags: u16,
}

impl FrameHeader {
    pub const GROUPING: u16 = 0x0040;
    pub const COMPRESSION: u16 = 0x0008;
    pub const ENCRYPTION: u16 = 0x0004;
    pub const UNSYNCHRONIZATION: u16 = 0x0002;
    pub const DATA_LENGTH_INDICATOR: u16 = 0x0001;

    pub fn from_bytes(d: &[u8; 10]) -> Self {
        Self {
            id: u32::from_be_bytes(d[..4].try_into().unwrap()),
            size: parsers::syncsafe_be_u32(d[4..8].try_into().unwrap()),
            flags: u16::from_be_bytes(d[8..].try_into().unwrap()),
        }
    }

    pub fn grouping(&self) -> bool {
        self.has_flag(Self::GROUPING)
    }

    pub fn compression(&self) -> bool {
        self.has_flag(Self::COMPRESSION)
    }

    pub fn encryption(&self) -> bool {
        self.has_flag(Self::ENCRYPTION)
    }

    pub fn unsynchronization(&self) -> bool {
        self.has_flag(Self::UNSYNCHRONIZATION)
    }

    pub fn data_length_indicator(&self) -> bool {
        self.has_flag(Self::DATA_LENGTH_INDICATOR)
    }

    fn has_flag(&self, flag: u16) -> bool {
        (self.flags & flag) == flag
    }
}

impl<R: BufRead> Breadable<R> for FrameHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
