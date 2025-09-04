mod genres;
mod v1;
pub mod v2;

pub use self::{v1::*, v2::read_id3v2};

use self::genres::*;
