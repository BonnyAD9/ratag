use std::io::{BufRead, Read, Seek, SeekFrom};

mod breadable;
mod breadable_be;
mod breadable_le;

use crate::{Result, Trap, TrapExt};

pub use self::{breadable::*, breadable_be::*, breadable_le::*};

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

    pub fn read_exact_owned(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; len];
        self.read.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn get<T: Breadable<R>>(&mut self) -> Result<T> {
        T::from_bread(self)
    }

    pub fn get_be<T: BreadableBe<R>>(&mut self) -> Result<T> {
        T::from_bread_be(self)
    }

    pub fn get_le<T: BreadableLe<R>>(&mut self) -> Result<T> {
        T::from_bread_le(self)
    }

    pub fn next(&mut self) -> Result<u8> {
        self.read_exact(1)?;
        Ok(self.buf[0])
    }

    pub fn arr<const LEN: usize>(&mut self) -> Result<&[u8; LEN]> {
        self.read_exact(LEN)?;
        Ok(self.buf[..LEN].try_into().unwrap())
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

    pub fn seek_by(&mut self, amt: i64) -> Result<()> {
        if amt == 0 {
            return Ok(());
        }
        self.seek(SeekFrom::Current(amt))?;
        Ok(())
    }

    pub fn useek_by(&mut self, mut amt: u64) -> Result<()> {
        const MAXI: u64 = i64::MAX as u64;
        while amt > MAXI {
            amt -= MAXI;
            self.seek_by(i64::MAX)?;
        }
        self.seek_by(amt as i64)
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

    pub fn witht_until<T, Tr: Trap>(
        &mut self,
        pat: &[u8],
        mut max_len: usize,
        trap: &Tr,
        f: impl FnOnce(&[u8], &Tr) -> Result<T>,
    ) -> Result<(Option<T>, usize)> {
        let mut buf = self.read.fill_buf()?;
        buf = &buf[..buf.len().min(max_len)];
        max_len -= buf.len();
        if let Some(p) = buf.windows(pat.len()).position(|a| a == pat) {
            let len = pat.len() + p;
            let res = f(&buf[..len], trap);
            self.read.consume(len);
            return trap.res(res).map(|a| (a, len));
        }

        self.buf.clear();
        self.buf.extend(buf);

        'outer: while max_len != 0 {
            buf = self.read.fill_buf()?;
            buf = &buf[..buf.len().min(max_len)];
            max_len -= buf.len();

            for i in 1..pat.len() - 1 {
                if self.buf.ends_with(&pat[..i]) && buf.starts_with(&pat[i..])
                {
                    let cnt = pat.len() - i;
                    self.buf.extend(&buf[..cnt]);
                    self.read.consume(cnt);
                    break 'outer;
                }
            }

            if let Some(p) = buf.windows(pat.len()).position(|a| a == pat) {
                let len = pat.len() + p;
                self.buf.extend(&buf[..len]);
                self.read.consume(len);
                break;
            }

            self.buf.extend(buf);
            let len = buf.len();
            self.read.consume(len);
        }

        trap.res(f(&self.buf, trap)).map(|a| (a, self.buf.len()))
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
