/// Picture
pub const APIC: u32 = u32::from_be_bytes(*b"APIC");
/// Album
pub const TALB: u32 = u32::from_be_bytes(*b"TALB");
/// Genre
pub const TCON: u32 = u32::from_be_bytes(*b"TCON");
/// Date
pub const TDAT: u32 = u32::from_be_bytes(*b"TDAT");
/// Release time.
pub const TDRL: u32 = u32::from_be_bytes(*b"TDRL");
/// Title
pub const TIT2: u32 = u32::from_be_bytes(*b"TIT2");
/// Time
pub const TIME: u32 = u32::from_be_bytes(*b"TIME");
/// Length
pub const TLEN: u32 = u32::from_be_bytes(*b"TLEN");
/// Artists
pub const TPE1: u32 = u32::from_be_bytes(*b"TPE1");
/// Album artist
pub const TPE2: u32 = u32::from_be_bytes(*b"TPE2");
/// Disc
pub const TPOS: u32 = u32::from_be_bytes(*b"TPOS");
/// Track
pub const TRCK: u32 = u32::from_be_bytes(*b"TRCK");
/// Year
pub const TYER: u32 = u32::from_be_bytes(*b"TYER");
/// Comment
pub const COMM: u32 = u32::from_be_bytes(*b"COMM");
/// Copyright
pub const TCOP: u32 = u32::from_be_bytes(*b"TCOP");
/// Popularimeter (rating)
pub const POPM: u32 = u32::from_be_bytes(*b"POPM");
