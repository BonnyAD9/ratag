use std::io::BufRead;

use crate::{
    Error, Result,
    bread::{Bread, Breadable},
};

pub struct Header {
    pub version: u16,
    flags: u8,
    pub size: u32,
}

impl Header {
    const UNSYNCHRONIZATION: u8 = 0x80;
    const EXTENDED_HEADER: u8 = 0x40;
    const _EXPERIMENTAL_INDICATOR: u8 = 0x20;
    pub const VERSION3: u16 = 0x0300;

    pub fn unsynchronization(&self) -> bool {
        self.get_flag(Self::UNSYNCHRONIZATION)
    }

    pub fn extended_header(&self) -> bool {
        self.get_flag(Self::EXTENDED_HEADER)
    }

    pub fn _experimental_indicator(&self) -> bool {
        self.get_flag(Self::_EXPERIMENTAL_INDICATOR)
    }

    fn get_flag(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }
}

impl<R: BufRead> Breadable<R> for Header {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        if !bread.expect(b"ID3")? {
            return Err(Error::NoTag);
        }

        let version = bread.get_be()?;
        let flags = bread.get()?;
        let size = bread.withc::<_, 4>(|d| {
            Ok((d[0] as u32) << 21
                | (d[1] as u32) << 14
                | (d[2] as u32) << 7
                | d[3] as u32)
        })?;

        Ok(Self {
            version,
            flags,
            size,
        })
    }
}
