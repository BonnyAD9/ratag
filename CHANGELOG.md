# CHANGELOG

## future
### Breaking changes
- Move `TagRead::extensions` into separate trait `TagFormat`.
- `Trap` now requires new method `Trap::encoder_trap`.
- `Error` is now marked `#[non_exoaustive]`.
- New error variants `FailedToEncode`, `InvalidVersion` and `OutOfRange`.

### New features
- Support writing ID3v1.
- New ID3v1 api:
    - Constants `ID3v1::LEN0` and `ID3v1::LEN2`.
    - New constructors `ID3v1Tag::from_seek_ver`, `ID3v1Tag::from_read_ver`,
      `ID3v1::try_from_read_ver` and `from_bytes_ver` that return the ID3v1
      version.
    - New methods `ID3v1Tag::retrieve` and `ID3v1::retrieve_fill` for updating
      information in ID3v1 tag.
    - New method `ID3v1::write_to` to write ID3v1 tag.
    - Implement `Default` for `ID3v1Tag`.
    - New functions `raw_write_to`, `write_to`, `write_to_file`,
      `write_version_to`, `write_version_to_file` to write ID3v1 tag.
    - New functions `migrate_version` and `migrate_version_file` for migrating
      between minor versions of ID3v1 tag.
    - New functions `remove` and `remove_from_file` for removing ID3v1 tag from
      file.
- New functions `write_any_tag`, `write_any_tag_to_file`, `write_tag` and
  `write_tag_to_file` for writing tags.
- New trait `SetLength`.

## v0.1.1
### New features
- Add support for album artist.

## v0.1.0
- Support for ID3v1 and flac.
- Partial support for ID3v2, mp4, ASF and RIFF.
- Supported metadata: Title, Album, Artists, Track, Track count, Year, Date,
  Time, Disc, Disc count, Comments, Pictures, Ratings, Tag type
- Tag types: Basic, Picture, Probe