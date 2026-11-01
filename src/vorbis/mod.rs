mod identification_header;
mod ogg_vorbis;
mod ogg_vorbis_stream;

pub use self::ogg_vorbis::*;

// Backward compatibility.
pub use crate::vorbis_comment::*;

// Implementation is based on:
// https://xiph.org/vorbis/doc/Vorbis_I_spec.html#x1-640004.2.3
