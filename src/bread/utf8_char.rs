pub struct Utf8Char([u8; 4]);

impl Utf8Char {
    pub fn new(c: char) -> Self {
        let len = c.len_utf8();
        let c = c as u32;
        match len {
            1 => Self([c as u8, 0, 0, 0]),
            2 => Self([0xc0 | (c >> 6) as u8, 0x10 | (c & 0x3f) as u8, 0, 0]),
            3 => Self([
                0xe0 | (c >> 12) as u8,
                0x10 | (c >> 6 & 0x3f) as u8,
                0x10 | (c & 0x3f) as u8,
                0,
            ]),
            4 => Self([
                0xf0 | (c >> 18) as u8,
                0x10 | (c >> 12 & 0x3f) as u8,
                0x10 | (c >> 6 & 0x3f) as u8,
                0x10 | (c & 0x3f) as u8,
            ]),
            _ => unreachable!(),
        }
    }

    pub fn as_str(&self) -> &str {
        let mut len = self.0[0].leading_ones() as usize;
        if len == 0 {
            len = 1;
        }
        // TODO: remove unwrap, use unchecked
        str::from_utf8(&self.0[..len]).unwrap()
    }
}
