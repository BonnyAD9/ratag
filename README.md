# ratag
Rust Audio TAG library.

Library for reading metadata from audio files.

Supported tag formats:
- `ID3v1` (`ID3v1.0`, `ID3v1.1`, `ID3v1.2`)
- `ID3v2`
    - Only version `ID3v2.3`
    - Unsynchronization, compression and ecryption is not supported.
    - Only frames `TALB`, `TCON`, `TIT2`, `TLEN`, `TPE1`, `TPOS`, `TRCK` and
      `TYER`