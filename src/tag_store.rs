use std::time::Duration;

use crate::DataType;

#[allow(unused_variables)]
pub trait TagStore {
    fn stores_data(&self, typ: DataType) -> bool;

    fn done(&self) -> bool {
        false
    }

    fn set_title(&mut self, title: Option<String>) {}
    fn set_album(&mut self, album: Option<String>) {}
    fn set_artists(&mut self, artists: Vec<String>) {}
    fn set_genres(&mut self, genres: Vec<String>) {}
    fn set_track(&mut self, track: Option<u32>) {}
    fn set_track_count(&mut self, cnt: Option<u32>) {}
    fn set_year(&mut self, year: Option<u32>) {}
    fn set_disc(&mut self, disc: Option<u32>) {}
    fn set_disc_count(&mut self, cnt: Option<u32>) {}
    fn set_length(&mut self, length: Option<Duration>) {}
}
