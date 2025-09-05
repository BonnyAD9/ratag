use std::io::BufRead;

use crate::{Bread, Result};

pub trait BreadableLe<R>: Sized {
    fn from_bread_le(bread: &mut Bread<R>) -> Result<Self>;
}

macro_rules! impl_breadable_be {
    ($($t:ident),* $(,)?) => {
        $(impl<R: BufRead> BreadableLe<R> for $t {
            fn from_bread_le(bread: &mut super::Bread<R>) -> crate::Result<Self> {
                Ok($t::from_le_bytes(bread.get()?))
            }
        })*
    };
}

impl_breadable_be!(u16, i16, u32, i32, u64, i64);
