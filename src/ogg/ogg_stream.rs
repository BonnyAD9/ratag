use crate::{Result, ogg::OggHeader};

/// Generic Ogg stream that is capable of parsing by pages.
pub trait OggStream<S, T> {
    /// Parse the next page with the given page header.
    ///
    /// # Returns
    /// `true` if more pages are expected. `false` if further pages will not
    /// lead to more tags being red.
    fn read_page(
        &mut self,
        header: &OggHeader,
        page: &[u8],
        store: &mut S,
        trap: &T,
    ) -> Result<bool>;
}
