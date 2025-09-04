mod metadata_block_header;
mod streaminfo;

use self::{metadata_block_header::*, streaminfo::*};

use std::{
    io::{BufRead, Seek},
    time::Duration,
};

use crate::{
    DataType, Error, Result, TagStore, bread::Bread, trap::Trap, vorbis,
};

pub fn from_read(
    r: impl BufRead + Seek,
    store: &mut impl TagStore,
    trap: &impl Trap,
) -> Result<()> {
    let mut r = Bread::new(r);
    r.expect(b"fLaC").map_err(|_| Error::NoTag)?;

    let mut next = true;
    while next && !store.done() {
        let header: MetadataBlockHeader = r.get()?;
        next = !header.last;
        match header.block_type {
            MetadataBlockHeader::STREAMINFO
                if store.stores_data(DataType::Length) =>
            {
                let si: Streaminfo = r.get()?;
                let secs = si.sample_cnt as f64 / si.sample_rate as f64;
                store.set_length(Some(Duration::from_secs_f64(secs)));
            }
            MetadataBlockHeader::VORBISCOMMENT => {
                vorbis::from_bread(&mut r, store, trap, false)?;
            }
            _ => _ = r.seek_by(header.length as i64)?,
        }
    }

    Ok(())
}
