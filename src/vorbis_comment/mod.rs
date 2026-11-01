use std::io::BufRead;

use crate::{Result, TagStore, bread::Bread, trap::Trap};

mod vorbis_tag;

pub use self::vorbis_tag::*;

// Implementation is based on:
// - https://xiph.org/vorbis/doc/v-comment.html
// - https://wiki.xiph.org/Field_names
// - https://age.hobba.nl/audio/mirroredpages/ogg-tagging.html

/// Read metadata from vorbis comment. The stream must have proper position.
pub fn from_read(
    r: impl BufRead,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    VorbisTag::from_read(r, trap)?.store(store, trap)
}

pub(crate) fn from_bread(
    r: &mut Bread<impl BufRead>,
    store: &mut impl TagStore,
    trap: &impl Trap,
    framing_bit: bool,
) -> Result<()> {
    VorbisTag::from_bread(r, trap, framing_bit)?.store(store, trap)
}
