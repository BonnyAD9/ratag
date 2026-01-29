use std::{collections::HashMap, io::BufRead};

use crate::{
    Comment, DataType, Error, Result, TagStore, TagStoreExt, TagType,
    bread::Bread,
    parsers::{self, DateTime},
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
            .witht(vendor_len as usize, trap, parsers::utf_8)?
            .unwrap_or_default();

        let comment_cnt: u32 = r.get_le()?;
        let mut comments: HashMap<_, Vec<_>> = HashMap::new();
        for _ in 0..comment_cnt {
            let len: u32 = r.get_le()?;
            let Some(comment) = r.witht(len as usize, trap, parsers::utf_8)?
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
        store.set_tag_type(TagType::VorbisComment);

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
                "TITLE" => store.set_title(last(v)),
                "ALBUM" => store.set_album(last(v)),
                "TRACKNUMBER" if store.stores_data(DataType::Track) => {
                    if let Some((t, c)) =
                        trap.res(parsers::num_of(&last(v), trap))?
                    {
                        store.set_track(t);
                        if let Some(c) = c {
                            store.set_track_count(c);
                        }
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
                    if let Some(d) = trap.res(parse_date(&first(v), trap))? {
                        store.set_date_time(d);
                    }
                }
                "DISCNUMBER" if store.stores_data(DataType::Disc) => {
                    if let Some((d, c)) =
                        trap.res(parsers::num_of(&last(v), trap))?
                    {
                        store.set_disc(d);
                        if let Some(c) = c {
                            store.set_disc_count(c);
                        }
                    }
                }
                "TRACKTOTAL" if store.stores_data(DataType::TrackCount) => {
                    if let Some(v) = trap.res(parsers::num(&last(v)))? {
                        store.set_track_count(v);
                    }
                }
                "DISCTOTAL" if store.stores_data(DataType::DiscCount) => {
                    if let Some(v) = trap.res(parsers::num(&last(v)))? {
                        store.set_disc_count(v);
                    }
                }
                "COMMENT" if store.stores_data(DataType::Comments) => {
                    store.set_comments(
                        v.into_iter().map(Comment::from_value).collect(),
                    );
                }
                "COPYRIGHT" if store.stores_data(DataType::Copyright) => {
                    store.set_copyright(last(v));
                }
                "ALBUMARTIST" | "ALBUM ARTIST"
                    if store.stores_data(DataType::AlbumArtist) =>
                {
                    store.set_album_artist(last(v));
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn parse_date(s: &str, trap: &impl Trap) -> Result<DateTime> {
    parsers::year(&s[..s.find(' ').unwrap_or(s.len())], trap)
}
