use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

#[derive(Debug)]
pub struct ChunkHeader {
    pub id: u32,
    pub size: u32,
}

impl ChunkHeader {
    pub fn from_bytes(d: &[u8; 8]) -> Self {
        Self {
            id: u32::from_be_bytes(d[..4].try_into().unwrap()),
            size: u32::from_le_bytes(d[4..].try_into().unwrap()),
        }
    }
}

impl<R: BufRead> Breadable<R> for ChunkHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
