# ratag
Rust Audio TAG library.

Library for reading metadata from audio files.

Recognized file extensions:
- `mp3`, `mpga`, `bit`, `flac`, `mp4`, `m4a`, `m4p`, `m4b`, `m4r`, `m4v`

Data that can be read from all formats that support it:
- Title, Album, Artists, Track, Track count, Year, Date, Time, Disc, Disc
  count, Comments

Supported tag formats:
- `ID3v1` (`ID3v1.0`, `ID3v1.1`, `ID3v1.2`)
    - Fully supported.
- `ID3v2`
    - Only version `ID3v2.3`
    - Unsynchronization, compression and ecryption is not supported.
    - Only frames `COMM`, `TALB`, `TCON`, `TDAT`, `TIT2`, `TIME`, `TLEN`,
      `TPE1`, `TPOS`, `TRCK` and `TYER`
- `flac`
    - Song length.
    - Vorbis comment can extract only `TITLE`, `ALBUM`, `TRACKNUMBER`,
      `ARTIST`, `GENRE`, `DATE`, `DISCNUMBER`, `TRACKTOTAL`, `DISCTOTAL` and
      `COMMENT`
- `mp4`
    - Song length at `moov.mvhd`.
    - Metadata at `moov.udta.meta.ilst`: ` nam`, ` cmt`, ` day`, ` ART`,
      ` trk`, `trkn`, ` alb`, `gnre`, `disk`

Other parsers:
- `vorbis comment`
    - fully supported when given stream with correct position