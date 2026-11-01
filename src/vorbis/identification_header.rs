use std::io::BufRead;

use crate::{
    Result,
    bread::{Bread, Breadable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentificationHeader {
    pub version: u32,
    // pub audio_channels: u8
    // pub audio_sample_rate: u32,
    // pub bitrate_maximum: i32,
    // pub bitrate_nominal: i32,
    // pub bitrate_minimum: i32,
    // pub block_size_01: u8
    pub framing_bit: bool,
}

impl IdentificationHeader {
    pub fn from_bytes(data: &[u8; 23]) -> Self {
        Self {
            version: u32::from_le_bytes(*data.first_chunk().unwrap()),
            // audio_channels: data[4]
            // audio_sample_rate: data[5..9],
            // bitrate_maximum: data[9..13],
            // bitrate_nominal: data[13..17],
            // bitrate_minimum: data[17..21],
            // block_size_01: data[21],
            framing_bit: (data[22] & 0x01) == 1,
        }
    }
}

impl<R: BufRead> Breadable<R> for IdentificationHeader {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(Self::from_bytes)
    }
}
