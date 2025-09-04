mod bread;
mod err;
pub mod id3;
use self::bread::Bread;
mod basic_tag;
mod data_type;
mod tag_store;
pub mod trap;

pub use place_macro::place;

pub use self::{basic_tag::*, data_type::*, err::*, tag_store::*};

use self::trap::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        assert!(true);
    }
}
