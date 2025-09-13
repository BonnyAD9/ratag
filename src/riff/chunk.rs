/// The riff file.
pub(super) const RIFF: u32 = u32::from_be_bytes(*b"RIFF");
/// List chunk.
pub(super) const LIST: u32 = u32::from_be_bytes(*b"LIST");
/// Info list.
pub(super) const INFO: u32 = u32::from_be_bytes(*b"INFO");
/// Artist.
pub(super) const IART: u32 = u32::from_be_bytes(*b"IART");
/// Comments.
pub(super) const ICMT: u32 = u32::from_be_bytes(*b"ICMT");
/// Copyright
pub(super) const ICOP: u32 = u32::from_be_bytes(*b"ICOP");
/// Genre
pub(super) const IGNR: u32 = u32::from_be_bytes(*b"IGNR");
/// Date
pub(super) const ICRD: u32 = u32::from_be_bytes(*b"ICRD");
/// Title
pub(super) const INAM: u32 = u32::from_be_bytes(*b"INAM");
/// Album
pub(super) const IPRD: u32 = u32::from_be_bytes(*b"IPRD");
/// Track
pub(super) const IPRT: u32 = u32::from_be_bytes(*b"IPRT");
/// Disc
pub(super) const PRT1: u32 = u32::from_be_bytes(*b"PRT1");
/// Disc count
pub(super) const PRT2: u32 = u32::from_be_bytes(*b"PRT2");
/// Format information
pub(super) const FMT: u32 = u32::from_be_bytes(*b"fmt ");
/// Wave format file.
pub const WAVE: u32 = u32::from_be_bytes(*b"WAVE");
/// Data chunk.
pub(super) const DATA: u32 = u32::from_be_bytes(*b"data");
