use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

#[derive(Debug)]
pub struct FrameHeader {
    pub id: u32,
    pub size: u32,
    pub flags: u16,
}

impl FrameHeader {
    const COMPRESSION: u16 = 0x80;
    const ENCRYPTION: u16 = 0x40;
    const GROUPING: u16 = 0x20;

    pub fn compression(&self) -> bool {
        self.has_flag(Self::COMPRESSION)
    }

    pub fn encryption(&self) -> bool {
        self.has_flag(Self::ENCRYPTION)
    }

    pub fn grouping(&self) -> bool {
        self.has_flag(Self::GROUPING)
    }

    fn has_flag(&self, flag: u16) -> bool {
        (self.flags & flag) == flag
    }
}

impl<R: BufRead> Breadable<R> for FrameHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        Ok(Self {
            id: bread.get_be()?,
            size: bread.get_be()?,
            flags: bread.get_be()?,
        })
    }
}
