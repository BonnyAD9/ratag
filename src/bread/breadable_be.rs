use std::io::BufRead;

use crate::{Bread, Result};

pub trait BreadableBe<R>: Sized {
    fn from_bread_be(bread: &mut Bread<R>) -> Result<Self>;
}

macro_rules! impl_breadable_be {
    ($($t:ident),* $(,)?) => {
        $(impl<R: BufRead> BreadableBe<R> for $t {
            fn from_bread_be(bread: &mut super::Bread<R>) -> crate::Result<Self> {
                Ok($t::from_be_bytes(bread.get()?))
            }
        })*
    };
}

impl_breadable_be!(u16, i16, u32, i32, u64, i64);
