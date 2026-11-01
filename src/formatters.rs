use encoding::{
    ByteWriter, Encoding,
    all::{ISO_8859_1, UTF_16BE},
};

use crate::{Error, Result, bread::utf8_char::Utf8Char, trap::Trap};

pub fn syncsafe_be_u32(mut n: u32) -> [u8; 4] {
    n = n & 0x3fff | n & 0x3fffc000 << 2;
    n = n & 0x7f007f | n & 0x3f803f80 << 1;
    n.to_be_bytes()
}

pub fn iso_8859_1(
    d: &str,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    ISO_8859_1
        .encode_to(d, trap.encoder_trap(), res)
        .map_err(|_| Error::FailedToEncode)
}

pub fn iso_8859_1_map(
    d: &str,
    mut r: impl FnMut(char) -> char,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    for c in d.chars() {
        iso_8859_1(Utf8Char::new(r(c)).as_str(), res, trap)?;
    }
    Ok(())
}

pub fn iso_8859_1_no_nl(
    d: &str,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    iso_8859_1_map(d, |c| if c == '\n' { ' ' } else { c }, res, trap)
}

pub fn utf16_le(
    d: &str,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    UTF_16BE
        .encode_to(d, trap.encoder_trap(), res)
        .map_err(|_| Error::FailedToEncode)
}

pub fn utf16_bom_le(
    d: &str,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    res.write_bytes(&[0xff, 0xfe]);
    utf16_le(d, res, trap)
}

pub fn utf16_le_map(
    d: &str,
    mut r: impl FnMut(char) -> char,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    for c in d.chars() {
        utf16_le(Utf8Char::new(r(c)).as_str(), res, trap)?;
    }
    Ok(())
}

pub fn utf16_bom_le_map(
    d: &str,
    r: impl FnMut(char) -> char,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    res.write_bytes(&[0xff, 0xfe]);
    utf16_le_map(d, r, res, trap)
}

pub fn utf16_bom_le_no_nl(
    d: &str,
    res: &mut impl ByteWriter,
    trap: &impl Trap,
) -> Result<()> {
    utf16_bom_le_map(d, |c| if c == '\n' { ' ' } else { c }, res, trap)
}
