use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
    parsers,
};

pub struct FrameHeader {
    pub id: u32,
    pub size: u32,
}

impl FrameHeader {
    pub fn from_bytes(d: &[u8; 6]) -> Self {
        Self {
            id: parsers::be_u24(d[..3].try_into().unwrap()),
            size: parsers::be_u24(d[3..].try_into().unwrap()),
        }
    }
}

impl<R: BufRead> Breadable<R> for FrameHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
