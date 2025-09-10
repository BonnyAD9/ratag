use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

#[derive(Debug)]
pub struct Object {
    pub guid: u128,
    pub size: i64,
}

impl Object {
    pub fn from_bytes(d: &[u8; 24]) -> Self {
        Self {
            guid: u128::from_be_bytes(d[..16].try_into().unwrap()),
            size: i64::from_le_bytes(d[16..].try_into().unwrap()),
        }
    }
}

impl<R: BufRead> Breadable<R> for Object {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
