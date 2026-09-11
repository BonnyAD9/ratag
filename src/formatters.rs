use encoding::{ByteWriter, Encoding, all::ISO_8859_1};

use crate::{Error, Result, trap::Trap};

pub fn iso_8859_1(
    d: &str,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    ISO_8859_1
        .encode_to(d, trap.encoder_trap(), res)
        .map_err(|_| Error::FailedToEncode)
}
