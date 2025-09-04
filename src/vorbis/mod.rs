use std::io::{BufRead, Seek};

use crate::{Result, TagStore, bread::Bread, trap::Trap};

pub(crate) fn from_bread(
    r: &mut Bread<impl BufRead + Seek>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    framing_bit: bool,
) -> Result<()> {
    todo!()
}
