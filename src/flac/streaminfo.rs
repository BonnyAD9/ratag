use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

pub struct Streaminfo {
    pub sample_cnt: u64,
    pub sample_rate: u32,
}

impl Streaminfo {
    pub fn from_bytes(d: &[u8; 34]) -> Self {
        // 0..2: min block size
        // 2..4: max block size
        // 4..7: min frame size
        // 7..10: max frame size
        // 10..18: sample rate, channel count, bits per sample, sample cnt
        // 18..34: MD5 signature

        let d = u64::from_be_bytes(d[10..18].try_into().unwrap());
        // 0..20: sample rate
        // 20..23: channel count
        // 23..28: bits per sample
        // 28..64: sample count
        let sample_rate = ((d & 0xFFFF_F000_0000_0000) >> 44) as u32;
        let sample_cnt = d & 0xFFF_FFFF;

        Self {
            sample_cnt,
            sample_rate,
        }
    }
}

impl<R: BufRead> Breadable<R> for Streaminfo {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(|a| Ok(Self::from_bytes(a)))
    }
}
