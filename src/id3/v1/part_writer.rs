use encoding::ByteWriter;

pub struct PartWriter<'a> {
    part1: &'a mut [u8],
    part2: &'a mut [u8],
    pos: usize,
}

impl<'a> PartWriter<'a> {
    pub fn new(part1: &'a mut [u8], part2: &'a mut [u8]) -> Self {
        Self {
            part1,
            part2,
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl ByteWriter for PartWriter<'_> {
    fn write_byte(&mut self, b: u8) {
        if let Some(l) = self.pos.checked_sub(self.part1.len()) {
            if l >= self.part2.len() {
                return;
            }
            self.part2[l] = b;
        } else {
            self.part1[self.pos] = b;
        }
        self.pos += 1;
    }

    fn write_bytes(&mut self, mut v: &[u8]) {
        if let Some(l) = self.part1.len().checked_sub(self.pos) {
            let l = l.min(v.len());
            self.part1[self.pos..self.pos + l].copy_from_slice(&v[..l]);
            self.pos += l;
            v = &v[l..];
        }

        if v.is_empty() {
            return;
        }

        let l = v.len().min(self.part1.len() + self.part2.len() - self.pos);
        self.part2.copy_from_slice(&v[..l]);
        self.pos += l;
    }
}
