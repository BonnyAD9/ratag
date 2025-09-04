use crate::Result;

pub trait TagRead<R, S, T> {
    fn extensions(&self) -> &[&str];
    fn store(&self, r: &mut R, store: &mut S, trap: &T) -> Result<()>;
}
