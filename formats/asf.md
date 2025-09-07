# Advanced Systems Format

I have not touched the microsoft specification. This has been written based on
taglib implementation and reverse engineering asf files.

All numbers are little endian.

GUID: 16B binary id
STR: UTF-16 LE null terminated string
BOOL: 2B value - ==0 => false, !=0 => true
BYTES: some binary data
ATTR_TYPE: INT16: attribute type:
    0: STR
    1: BYTES
    2: BOOL
    3: INT64
    4: INT32
    5: INT16
    6: guid

## Object
GUID: id
INT64: object size including header (e.g. data size + 24)
data based on object size

### File header

GUID: "\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C"
INT64: size

INT32: number of objects
2B: ??

...: objects
    File properties object
    Stream properties object
    Content description object
    Extended object description object
    Header extension object
    Codec list object
    Content encrypption object
    Extended content encryption object
    Advanced content encryption object
    ...??

### File properties object

GUID: "\xA1\xDC\xAB\x8C\x47\xA9\xCF\x11\x8E\xE4\x00\xC0\x0C\x20\x53\x65"
INT64: size

40B: ??
INT64: Duration in number of 100 ns intervals.
8B: ??
INT64: Preroll - number of ms of silence at the start of the file.

...: ??

### Stream properties object

GUID: "\x91\x07\xDC\xB7\xB7\xA9\xCF\x11\x8E\xE6\x00\xC0\x0C\x20\x53\x65"
INT64: size

54B: ??
UINT16: codec:
    0x0160: WMA1
    0x0161: WMA2
    0x0162: WMA9Pro
    0x0163: WMA9Lossless
UINT16: Channel count
UINT32: Sample rate
UINT32: Bitrate in bytes per second
2B: ??
UINT16: Bits per sample

...: ??

### Content description object

GUID: "\x33\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C"
INT64: size

INT32: title length (in bytes)
INT32: artist length (in bytes)
INT32: copyright length (in bytes)
INT32: comment length (in bytes)
INT32: rating length (in bytes)

STR: title
STR: artist
STR: copyright
STR: comment
STR: rating

...: ??

### Extended content description object

GUID: "\x40\xA4\xD0\xD2\x07\xE3\xD2\x11\x97\xF0\x00\xA0\xC9\x5E\xA8\x50"
INT64: size

INT32: count

...: Attributes

#### Attribute

UINT16: name length
STR: name
ATTR_TYPE: attribute type
UINT16: value size

...: attribute value

### Header extension object

GUID: "\xb5\x03\xbf_.\xa9\xcf\x11\x8e\xe3\x00\xc0\x0c Se"
INT64: size

18B: ??
INT32: data size

...: Extension objects:
    Metadata object
    Metadata library object
    ...??

### Metadata object

GUID: "\xEA\xCB\xF8\xC5\xAF[wH\204g\xAA\214D\xFAL\xCA"
INT64: size

INT32: count

...: Attributes

#### Attribute

INT16: padding
INT16: stream?
ATTR_TYPE: attribute type
UINT32: value size
STR: name

...: attribute value

### Metadata library object

GUID: "\224\034#D\230\224\321I\241A\x1d\x13NEpT"
INT64: size

INT32: count

...: Attributes

#### Attribute

INT16: language?
INT16: stream?
ATTR_TYPE: attribute type
UINT32: value size
STR: name

...: attribute value

### Codec list object

GUID: "\x40\x52\xd1\x86\x1d\x31\xd0\x11\xa3\xa4\x00\xa0\xc9\x03\x48\xf6"
INT64: size

UINT32: count

...: Codecs

#### Codec

INT16: codec type:
    0x0001: Video
    0x0001: Audio
UINT16: name length (in characters)
UINT16: description length (in characters)
UINT16: info length

STR: name (not null terminated)
STR: description (not null terminated)

...: ??

### Content encryption object

GUID: "\xFB\xB3\x11\x22\x23\xBD\xD2\x11\xB4\xB7\x00\xA0\xC9\x55\xFC\x6E"
INT64: size

...??

### Extended content encryption object

GUID: "\x14\xE6\x8A\x29\x22\x26 \x17\x4C\xB9\x35\xDA\xE0\x7E\xE9\x28\x9C"
INT64: size

...??

### Advanced content encryption object

GUID: "\xB6\x9B\x07\x7A\xA4\xDA\x12\x4E\xA5\xCA\x91\xD3\x8D\xC1\x1A\x8D"
INT64: size

...??

## Available attributes

Each attribute may be set multiple times to indicate list of values.

### `WM/AlbumTitle`
STR

Album title.

### `WM/AlbumArtist`
### `WM/AuthorURL`
### `WM/Composer`
### `WM/Writer`
### `WM/Conductor`
### `WM/ModifiedBy`
### `WM/Year`
STR 

Unsigned year as string. Possibly followed by non numbers.

### `WM/OriginalAlbumTitle`
### `WM/OriginalArtist`
### `WM/OriginalFilename`
### `WM/OriginalLyricist`
### `WM/OrigialReleaseYear`
### `WM/Producer`
### `WM/ContentGroupDescription`
### `WM/SubTitle`
### `WM/SetSubTitle`
### `WM/TrackNumber`
UINT16, STR

Track number. If string, possibly followed by non numbers.

### `WM/Track`
UINT16

Track number.

### `WM/PartOfSet`
### `WM/Genre`
STR

Genre.

### `WM/BeatsPerMinute`
### `WM/Mood`
### `WM/InitialKey`
### `WM/ISRC`
### `WM/Lyrics`
### `WM/Media`
### `WM/Publisher`
### `WM/CatalogNo`
### `WM/Barcode`
### `WM/EncodedBy`
### `WM/EncodingSettings`
### `WM/EncodingTime`
### `WM/AudioFileURL`
### `WM/AlbumSortOrder`
### `WM/AlbumArtistSortOrder`
### `WM/ArtistSortOrder`
### `WM/TitleSortOrder`
### `WM/Script`
### `WM/Language`
### `WM/ARTISTS`
### `ASIN`
### `MusicBrainz/Track Id`
### `MusicBrainz/Artist Id`
### `MusicBrainz/Album Id`
### `MusicBrainz/Album Artist Id`
### `MusicBrainz/Album Release Country`
### `MusicBrainz/Album Type`
### `MusicBrainz/Release Group Id`
### `MusicBrainz/Release Track Id`
### `MusicBrainz/Work Id`
### `MusicIP/PUID`
### `Acoustid/Id`
### `Acoustid/Fingerprint`
### `WM/Picture`
BYTES

UCHAR: picture type:
    0x00: Other
    0x01: 32x32 png file icon
    0x02: Other file icon
    0x03: Front cover
    0x04: Back cover
    0x05: Leaflet page
    0x06: Media
    0x07: Lead artist/soloist
    0x08: Artist/performer
    0x09: Conductor
    0x0A: Band/orchestra
    0x0B: Composer
    0x0C: Lyricist/text writer
    0x0D: Recording location/studio
    0x0E: Artists during recording
    0x0F: Artists during performance
    0x10: Movie/video related to the track
    0x11: Picture of a large, coloured fish
    0x12: Illustration
    0x13: Logo of band/performer
    0x14: Logo of piblisher/record company
UINT32: data length
STR: Mime type
STR: Description
BYTES: picture data