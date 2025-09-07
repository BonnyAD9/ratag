# ratag
Rust Audio TAG library.

Library for reading metadata from audio files.

Data that can be read from all formats that support it:
- Title, Album, Artists, Track, Track count, Year, Date, Time, Disc, Disc
  count, Comments, Pictures

Recognized file extensions: `mp3`, `mpga`, `bit`, `flac`, `mp4`, `m4a`, `m4p`,
`m4b`, `m4r`, `m4v`. File extensions are used only to speedup the process of
figuring out which tag format should be used. If that fails or the extension is
not recognized, the tag format will be figured out from the file contents.

Supported tag formats: `ID3v1`, `ID3v2`, `flac`, `mp4`. See below for detailed
description.

Supported tag formats (detailed):
- `ID3v1` (`ID3v1.0`, `ID3v1.1`, `ID3v1.2`)
    - Fully supported.
- `ID3v2` (`ID3v2.2`, `ID3v2.3`, `ID3v2.4`)
    - Unsynchronization, compression and ecryption is not supported.
    - ID3v2.2 only frames `TT2`, `TP1`, `TCO`, `TAL`, `TPA`, `TRK`, `TYE`,
      `TDA`, `TIM`, `TLE`, `COM` and `PIC`.
    - ID3v2.3 only frames `APIC`, `COMM`, `TALB`, `TCON`, `TDAT`, `TIT2`,
      `TIME`, `TLEN`, `TPE1`, `TPOS`, `TRCK` and `TYER`.
    - ID3v2.4 only frames `TIT2`, `TALB`, `TRCK`, `TPOS`, `TPE1`, `TLEN`,
      `TCON`, `TDRL`, `COMM` and `APIC`.
    - ID3v2.4 updates are not supported.
    - ID3v2.4 appended frames are not supported.
- `flac`
    - Song length.
    - Picture.
    - Vorbis comment can extract only `TITLE`, `ALBUM`, `TRACKNUMBER`,
      `ARTIST`, `GENRE`, `DATE`, `DISCNUMBER`, `TRACKTOTAL`, `DISCTOTAL` and
      `COMMENT`
- `mp4`
    - Song length at `moov.mvhd`.
    - Metadata at `moov.udta.meta.ilst`: ` nam`, ` cmt`, ` day`, ` ART`,
      ` trk`, `trkn`, ` alb`, `gnre`, `disk`, `covr`

Other parsers:
- `vorbis comment`
    - fully supported when given stream with correct position