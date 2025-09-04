use std::io::{BufRead, ErrorKind, Read, Seek, SeekFrom};

mod breadable;
mod breadable_be;

use crate::{Error, Result, Trap, TrapExt};

pub use self::{breadable::*, breadable_be::*};

pub struct Bread<R> {
    buf: Vec<u8>,
    read: R,
}

impl<R: Read> Bread<R> {
    pub fn new(read: R) -> Self {
        Self { read, buf: vec![] }
    }

    pub fn read_exact(&mut self, len: usize) -> Result<&[u8]> {
        self.make_size(len);
        self.read.read_exact(&mut self.buf[..len])?;
        Ok(&self.buf)
    }

    pub fn get<T: Breadable<R>>(&mut self) -> Result<T> {
        T::from_bread(self)
    }

    pub fn get_be<T: BreadableBe<R>>(&mut self) -> Result<T> {
        T::from_bread_be(self)
    }

    pub fn arr<const LEN: usize>(&mut self) -> Result<&[u8; LEN]> {
        self.read_exact(LEN)?;
        Ok(self.buf[..LEN].try_into().unwrap())
    }

    pub fn _try_read_exact(&mut self, len: usize) -> Result<&[u8]> {
        self.make_size(len);
        let mut pos = 0;
        while pos < len {
            match self.read.read(&mut self.buf[pos..]) {
                Ok(0) => return Ok(&self.buf[..pos]),
                Ok(s) => pos += s,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(Error::Io(e)),
            }
        }

        Ok(&self.buf[pos..])
    }

    fn make_size(&mut self, len: usize) {
        if self.buf.len() < len {
            self.buf.resize(len, 0);
        }
    }
}

impl<R: Seek> Bread<R> {
    pub fn seek(&mut self, from: SeekFrom) -> Result<u64> {
        Ok(self.read.seek(from)?)
    }

    pub fn seek_by(&mut self, amt: i64) -> Result<u64> {
        self.seek(SeekFrom::Current(amt))
    }
}

impl<R: BufRead> Bread<R> {
    pub fn witht<T, Tr: Trap>(
        &mut self,
        len: usize,
        trap: &Tr,
        f: impl FnOnce(&[u8], &Tr) -> Result<T>,
    ) -> Result<Option<T>> {
        let buf = self.read.fill_buf()?;
        if buf.len() < len {
            return trap.res(f(self.read_exact(len)?, trap));
        }
        let res = f(&buf[..len], trap);
        self.read.consume(len);
        trap.res(res)
    }

    pub fn withc<T, const LEN: usize>(
        &mut self,
        f: impl FnOnce(&[u8; LEN]) -> Result<T>,
    ) -> Result<T> {
        let buf = self.read.fill_buf()?;
        if buf.len() < LEN {
            return f(self.arr()?);
        }
        let res = f(buf[..LEN].try_into().unwrap());
        self.read.consume(LEN);
        res
    }

    pub fn expect(&mut self, mut b: &[u8]) -> Result<bool> {
        while !b.is_empty() {
            let buf = self.read.fill_buf()?;
            let len = buf.len().min(b.len());
            if b[..len] != buf[..len] || len == 0 {
                return Ok(false);
            }
            self.read.consume(len);
            b = &b[len..];
        }
        Ok(true)
    }
}
