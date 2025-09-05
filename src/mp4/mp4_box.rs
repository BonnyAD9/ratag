use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
    mp4::opt_u64::OptU64,
};

#[derive(Debug)]
pub struct Mp4Box {
    pub size_next: OptU64,
    pub size_total: OptU64,
    pub boxtype: u32,
}

impl<R: BufRead> Breadable<R> for Mp4Box {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        let size: u32 = bread.get_be()?;
        let boxtype: u32 = bread.get_be()?;
        let (size_next, size_total) = match size {
            0 => (None, None),
            1 => {
                let size = bread.get_be::<u64>()?;
                (Some(size - 16), Some(size))
            }
            s => (Some(s as u64 - 8), Some(s as u64)),
        };
        Ok(Self {
            size_next: OptU64(size_next),
            size_total: OptU64(size_total),
            boxtype,
        })
    }
}
