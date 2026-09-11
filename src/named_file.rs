use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::PathBuf,
};

use tempfile::NamedTempFile;

use crate::{atomic_swap::AtomicSwap, set_length::SetLength};

#[derive(Debug)]
pub struct NamedFile {
    path: PathBuf,
    file: File,
}

impl NamedFile {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let path = path.into();
        let file = File::open(&path)?;
        Ok(Self { path, file })
    }

    pub fn modify(path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let mut opt = OpenOptions::new();
        let path = path.into();
        let file = opt.read(true).write(true).truncate(false).open(&path)?;
        Ok(Self { path, file })
    }
}

impl Write for NamedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl Read for NamedFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for NamedFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.file.seek(pos)
    }
}

impl AtomicSwap for NamedFile {
    type TempWrite = NamedTempFile;

    fn temp_write(&self) -> Result<Self::TempWrite, std::io::Error> {
        let parent = self
            .path
            .parent()
            .ok_or(std::io::ErrorKind::InvalidFilename)?;
        NamedTempFile::new_in(parent)
    }

    fn replace(&mut self, f: Self::TempWrite) -> Result<(), std::io::Error> {
        self.file = f.persist(&self.path)?;
        Ok(())
    }
}

impl SetLength for NamedFile {
    fn set_length(&self, len: u64) -> Result<(), std::io::Error> {
        self.file.set_len(len)
    }
}
