#[derive(Debug)]
pub struct WaveFmt {
    pub avg_bytes_per_sec: u32,
}

impl WaveFmt {
    pub fn from_bytes(d: &[u8; 14]) -> Self {
        Self {
            avg_bytes_per_sec: u32::from_le_bytes(
                d[8..12].try_into().unwrap(),
            ),
        }
    }
}
