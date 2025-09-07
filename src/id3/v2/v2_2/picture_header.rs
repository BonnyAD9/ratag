use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

#[derive(Debug)]
pub struct PictureHeader {
    pub encoding: u8,
    pub format: [u8; 3],
    pub typ: u8,
}

impl PictureHeader {
    pub fn from_bytes(d: &[u8; 5]) -> Self {
        Self {
            encoding: d[0],
            format: d[1..4].try_into().unwrap(),
            typ: d[4],
        }
    }
}

impl<R: BufRead> Breadable<R> for PictureHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
