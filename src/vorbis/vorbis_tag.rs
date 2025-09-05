use std::{collections::HashMap, io::BufRead, str::FromStr};

use encoding::{Encoding, all::UTF_8};
use time::{Date, format_description::well_known};

use crate::{
    Comment, DataType, Error, Result, TagStore, TagStoreExt,
    bread::Bread,
    trap::{Trap, TrapExt},
};

/// Tag storing vorbis comments.
#[derive(Debug)]
pub struct VorbisTag {
    /// Vendor string.
    pub vendor: String,
    /// Comments.
    pub comments: HashMap<String, Vec<String>>,
}

impl VorbisTag {
    /// Read vorbis comments from reader. The reader must be correctly
    /// positioned.
    pub fn from_read(r: impl BufRead, trap: &impl Trap) -> Result<Self> {
        let mut r = Bread::new(r);
        Self::from_bread(&mut r, trap, true)
    }

    pub(crate) fn from_bread(
        r: &mut Bread<impl BufRead>,
        trap: &impl Trap,
        framing_bit: bool,
    ) -> Result<Self> {
        let vendor_len: u32 = r.get_le()?;
        let vendor = r
            .witht(vendor_len as usize, trap, read_string)?
            .unwrap_or_default();

        let comment_cnt: u32 = r.get_le()?;
        let mut comments: HashMap<_, Vec<_>> = HashMap::new();
        for _ in 0..comment_cnt {
            let len: u32 = r.get_le()?;
            let Some(comment) = r.witht(len as usize, trap, read_string)?
            else {
                continue;
            };

            let Some((name, value)) = comment.split_once('=') else {
                trap.error(Error::InvalidVorbisComment)?;
                continue;
            };

            comments
                .entry(name.to_ascii_uppercase())
                .or_default()
                .push(value.to_string());
        }

        if framing_bit {
            let b = r.next()?;
            if b == 0 {}
        }

        Ok(Self { vendor, comments })
    }

    /// Store data from the read comments into a tag store.
    pub fn store(
        self,
        store: &mut impl TagStore,
        trap: &impl Trap,
    ) -> Result<()> {
        fn last<T>(v: Vec<T>) -> T {
            v.into_iter().next_back().unwrap()
        }
        fn first<T>(v: Vec<T>) -> T {
            v.into_iter().next().unwrap()
        }

        for (k, v) in self.comments {
            if v.is_empty() {
                continue;
            }
            match k.as_str() {
                "TITLE" => store.set_title(Some(last(v))),
                "ALBUM" => store.set_album(Some(last(v))),
                "TRACKNUMBER" if store.stores_data(DataType::Track) => {
                    if let Some(v) = trap.res(parse_num(&last(v)))? {
                        store.set_track(Some(v));
                    }
                }
                "ARTIST" if store.stores_data(DataType::Artists) => {
                    // Some software uses comma separated values.
                    store.set_artists(
                        v.iter()
                            .flat_map(|a| a.split(", "))
                            .map(|a| a.to_string())
                            .collect(),
                    );
                }
                "GENRE" => store.set_genres(v),
                "DATE"
                    if store.stores_data(DataType::Year)
                        || store.stores_data(DataType::Date) =>
                {
                    if let Some(d) = trap.res(parse_date(&first(v)))? {
                        store.set_date2(d);
                    }
                }
                "DISCNUMBER" if store.stores_data(DataType::Disc) => {
                    if let Some(v) = trap.res(parse_num(&last(v)))? {
                        store.set_disc(Some(v));
                    }
                }
                "TRACKTOTAL" if store.stores_data(DataType::TrackCount) => {
                    if let Some(v) = trap.res(parse_num(&last(v)))? {
                        store.set_track_count(Some(v));
                    }
                }
                "DISCTOTAL" if store.stores_data(DataType::DiscCount) => {
                    if let Some(v) = trap.res(parse_num(&last(v)))? {
                        store.set_disc_count(Some(v));
                    }
                }
                "COMMENT" if store.stores_data(DataType::Comments) => {
                    store.set_comments(
                        v.into_iter().map(Comment::from_value).collect(),
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn parse_num<T: FromStr>(s: &str) -> Result<T> {
    s.parse().map_err(|_| Error::InvalidDigit)
}

fn parse_date(date: &str) -> Result<Date> {
    let date = date.split_once(' ').map(|(d, _)| d).unwrap_or(date);

    Ok(Date::parse(date, &well_known::Iso8601::PARSING)?)
}

fn read_string(data: &[u8], trap: &impl Trap) -> Result<String> {
    UTF_8
        .decode(data, trap.decoder_trap())
        .map_err(|_| Error::InvalidEncoding)
}
