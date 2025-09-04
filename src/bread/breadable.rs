use std::io::BufRead;

use crate::{Bread, Result};

pub trait Breadable<R>: Sized {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self>;
}

impl<R: BufRead, const LEN: usize> Breadable<R> for [u8; LEN] {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc(|a| Ok(*a))
    }
}

impl<R: BufRead> Breadable<R> for u8 {
    fn from_bread(bread: &mut Bread<R>) -> Result<Self> {
        bread.withc::<_, 1>(|a| Ok(a[0]))
    }
}
