use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

pub struct FullBox {
    pub version: u8,
    pub flags: u32,
}

impl FullBox {
    pub const BINARY: u32 = 0;
    pub const TEXT: u32 = 1;
    pub const IMAGE: u32 = 0xD;

    pub fn from_bytes(d: [u8; 4]) -> Self {
        Self {
            version: d[0],
            flags: u32::from_be_bytes(d) & 0xFF_FFFF,
        }
    }
}

impl<R: BufRead> Breadable<R> for FullBox {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(|a| Ok(Self::from_bytes(*a)))
    }
}
