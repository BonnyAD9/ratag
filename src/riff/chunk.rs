/// The riff file.
pub const RIFF: u32 = u32::from_be_bytes(*b"RIFF");
/// List chunk.
pub const LIST: u32 = u32::from_be_bytes(*b"LIST");
/// Info list.
pub const INFO: u32 = u32::from_be_bytes(*b"INFO");
/// Artist.
pub const IART: u32 = u32::from_be_bytes(*b"IART");
/// Comments.
pub const ICMT: u32 = u32::from_be_bytes(*b"ICMT");
/// Copyright
pub const ICOP: u32 = u32::from_be_bytes(*b"ICOP");
/// Genre
pub const IGNR: u32 = u32::from_be_bytes(*b"IGNR");
/// Date
pub const ICRD: u32 = u32::from_be_bytes(*b"ICRD");
/// Title
pub const INAM: u32 = u32::from_be_bytes(*b"INAM");
/// Album
pub const IPRD: u32 = u32::from_be_bytes(*b"IPRD");
/// Track
pub const IPRT: u32 = u32::from_be_bytes(*b"IPRT");
/// Disc
pub const PRT1: u32 = u32::from_be_bytes(*b"PRT1");
/// Disc count
pub const PRT2: u32 = u32::from_be_bytes(*b"PRT2");
/// Format information
pub const FMT: u32 = u32::from_be_bytes(*b"fmt ");
/// Wave format file.
pub const WAVE: u32 = u32::from_be_bytes(*b"WAVE");
/// Data chunk.
pub const DATA: u32 = u32::from_be_bytes(*b"data");
