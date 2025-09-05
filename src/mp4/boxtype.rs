/// Album name box.
pub const ALB: u32 = u32::from_be_bytes(*b"\xa9alb");
/// Artist box.
pub const ART: u32 = u32::from_be_bytes(*b"\xa9ART");
/// Comment box.
pub const CMT: u32 = u32::from_be_bytes(*b"\xa9cmt");
/// Data box.
pub const DATA: u32 = u32::from_be_bytes(*b"data");
/// Date and time.
pub const DAY: u32 = u32::from_be_bytes(*b"\xa9day");
/// Disc number.
pub const DISK: u32 = u32::from_be_bytes(*b"disk");
/// File type information box.
pub const FTYP: u32 = u32::from_be_bytes(*b"ftyp");
/// Genre box.
pub const GNRE: u32 = u32::from_be_bytes(*b"gnre");
/// APPLE item list box.
pub const ILST: u32 = u32::from_be_bytes(*b"ilst");
/// Metadata box.
pub const META: u32 = u32::from_be_bytes(*b"meta");
/// Movie/presentation box.
pub const MOOV: u32 = u32::from_be_bytes(*b"moov");
/// Movie header box.
pub const MVHD: u32 = u32::from_be_bytes(*b"mvhd");
/// Title box.
pub const NAM: u32 = u32::from_be_bytes(*b"\xa9nam");
/// Track number box.
pub const TRK: u32 = u32::from_be_bytes(*b"\xa9trk");
pub const TRKN: u32 = u32::from_be_bytes(*b"trkn");
/// User data box.
pub const UDTA: u32 = u32::from_be_bytes(*b"udta");
