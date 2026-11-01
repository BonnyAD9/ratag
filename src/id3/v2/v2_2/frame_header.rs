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

    pub fn to_bytes(&self) -> [u8; 6] {
        let mut res = [0; 6];
        res[0..3].copy_from_slice(&self.id.to_be_bytes()[1..]);
        res[3..].copy_from_slice(&self.size.to_be_bytes()[1..]);
        res
    }
}

impl<R: BufRead> Breadable<R> for FrameHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
