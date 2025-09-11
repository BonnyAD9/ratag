# ratag
Rust Audio TAG library.

Library for reading metadata from audio files.

Unlike other more specialized libraries, this library doesn't prioritize
support for everything the tagging formats have to offer but it prioritizes to
supporting the basic metadata in as formats as possible. In short it won't read
everyting from the file, but it aims to read the essentials from every file.

On the other it is made in a extensible way, so if you would like to create
your own reader for some specific tag format, it is simple to integrate it
alongside with the readers supported in this library.

Data that this library aims to be capable of reading:
- Title, Album, Artists, Track, Track count, Year, Date, Time, Disc, Disc
  count, Comments, Pictures, Copyright, Ratings

Recognized file extensions: `mp3`, `mpga`, `bit`, `flac`, `mp4`, `m4a`, `m4p`,
`m4b`, `m4r`, `m4v`, `asf`, `wma`, `wmv`, `wav`, `wave`, `avi`, `ani`, `pal`,
`rdi`, `dib`, `rmi`, `rmm`, `webp`. As you can see, the list contains many file
extensions that are not asociated with audio. This is because lot of different
file formats use the same format for tagging. File extensions are used only to
speedup the process of figuring out which tag format should be used. If that
fails or the extension is not recognized, the tag format will be figured out
from the file contents.

Supported tag formats: `ID3v1`, `ID3v2`, `flac`, `mp4`, `ASF`, `RIFF`. See
below for detailed description.

Supported tag formats (detailed):
- `ID3v1` (`ID3v1.0`, `ID3v1.1`, `ID3v1.2`)
    - Fully supported.
- `ID3v2` (`ID3v2.2`, `ID3v2.3`, `ID3v2.4`)
    - Unsynchronization, compression and ecryption is not supported.
    - ID3v2.2 only frames `TT2`, `TP1`, `TCO`, `TAL`, `TPA`, `TRK`, `TYE`,
      `TDA`, `TIM`, `TLE`, `COM`, `TCR`, `POP` and `PIC`.
    - ID3v2.3 only frames `APIC`, `COMM`, `TALB`, `TCON`, `TDAT`, `TIT2`,
      `TIME`, `TLEN`, `TPE1`, `TPOS`, `TRCK`, `TCOP`, `POPM` and `TYER`.
    - ID3v2.4 only frames `TIT2`, `TALB`, `TRCK`, `TPOS`, `TPE1`, `TLEN`,
      `TCON`, `TDRL`, `COMM`, `TCOP`, `POPM` and `APIC`.
    - ID3v2.4 updates are not supported.
    - ID3v2.4 appended frames are not supported.
- `flac`
    - Song length.
    - Picture.
    - Vorbis comment can extract only `TITLE`, `ALBUM`, `TRACKNUMBER`,
      `ARTIST`, `GENRE`, `DATE`, `DISCNUMBER`, `TRACKTOTAL`, `DISCTOTAL`,
      `COPYRIGHT` and `COMMENT`
- `mp4`
    - Song length at `moov.mvhd`.
    - Copyright at `moov.udta.cprt`.
    - Metadata at `moov.udta.meta.ilst`: ` nam`, ` cmt`, ` day`, ` ART`,
      ` trk`, `trkn`, ` alb`, `gnre`, `disk`, `covr`
- `ASF`
    - Length in file properties.
    - All fields in content description.
    - Some fields in extended content description: `WM/AlbumTitle`, `WM/Year`,
      `WM/TrackNumber`, `WM/PartOfSet`, `WM/Genre` and `WM/Picture`
- `RIFF`
    - Supported fields in `INFO` chunk: `IART`, `ICMT`, `ICOP`, `IGNR`, `ICRD`,
      `INAM`, `IPRD`, `IPRT`, `PRT1`, `PRT2`.
    - Length of `WAVE` form using `fmt` and length of `data`.

Other parsers:
- `vorbis comment`
    - fully supported when given stream with correct position