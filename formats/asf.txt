# Advanced Systems Format

I have not touched the microsoft specification. This has been written based on
taglib and ffmpeg implementation and reverse engineering asf files.

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
OBJ: object
    
## File contents
OBJ: file header object
GUID: ??
UINT64: ??
UCHAR: ??
UCHAR: ??

...: packets

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

GUID: file guid
UINT64: file size in bytes (invalid if broadcasting)
UINT64: time of creation in 100 ns units since 1.1.1601 (invalid if
    broadcasting)
UINT64: number of packets
UINT64: Duration in number of 100 ns intervals.
UINT64: Time to send file in 100 ns units.
UINT64: Preroll - timestamp of first packet in ms.
UINT32: Flags:
    0x01: broadcast
    0x02: seekable
UINT32: Min packet size (not larger than 2^29-1)
UINT32: Max packet size (shall be same as min)
UINT32: Max bitrate - bandwidth of stream in bps. Should be sum of individual
    streams.

...: ??

### Stream properties object

GUID: "\x91\x07\xDC\xB7\xB7\xA9\xCF\x11\x8E\xE6\x00\xC0\x0C\x20\x53\x65"
INT64: size

GUID: stream type
    audio stream: 0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B
    video stream: 0xC0, 0xEF, 0x19, 0xBC, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B
    JPEG video??: 0x00, 0xE1, 0x1B, 0xB6, 0x4E, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B
    command stream (data): 0xC0, 0xCF, 0xDA, 0x59, 0xE6, 0x59, 0xD0, 0x11, 0xA3, 0xAC, 0x00, 0xA0, 0xC9, 0x03, 0x48, 0xF6
    ext stream embed stream header: 0xe2, 0x65, 0xfb, 0x3a, 0xEF, 0x47, 0xF2, 0x40, 0xac, 0x2c, 0x70, 0xa9, 0x0d, 0x71, 0xd3, 0x43
GUID: ??
UINT64: total size
UINT32: type specific size
UINT32: ??
UINT16: stream id. (& 0x7f)
UINT32: ??

if ext stream embed stream header {
    GUID: stream type
        audio: 0x9d, 0x8c, 0x17, 0x31, 0xE1, 0x03, 0x28, 0x45, 0xb5, 0x82, 0x3d, 0xf9, 0xdb, 0x22, 0xf5, 0x03
    if audio {
        GUID: ??
        UINT32: ??
        UINT32: ??
        UINT32: ??
        GUID: ??
        UINT32: ??
    }
}

if audio {
    wav header
} else if video {
    UINT32: ??
    UINT32: ??
    UCHAR: ??
    UINT16: size

    UINT32: size of extra data that follows including this
    UINT32: width
    UINT32: height
    UINT16: panes
    UINT16: bit depth
    UINT32: tag1
    20B: ??
    
    ...: palette (UINT32)
}

#### Wav header

UINT16: codec:
    0x0160: WMA1
    0x0161: WMA2
    0x0162: WMA9Pro
    0x0163: WMA9Lossless
UINT16: Channel count
UINT32: Sample rate
UINT32: Bitrate in bytes per second
UINT16: Block align
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

### Extended content description object

GUID: "\x40\xA4\xD0\xD2\x07\xE3\xD2\x11\x97\xF0\x00\xA0\xC9\x5E\xA8\x50"
INT64: size

INT32: count

...: Attributes

#### Attribute

UINT16: name length (round up to even number to get correct value)
STR: name
ATTR_TYPE: attribute type (BOOL is 32 bit)
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

UINT32: len
DATA: packet of size len

UINT32: len
BYTES: protection type: ascii string of len

UINT32: len
BYTES: protection key: ascii string of len

UINT32: len
BYTES: licence url: ascii string of len

### Extended content encryption object

GUID: "\x14\xE6\x8A\x29\x22\x26 \x17\x4C\xB9\x35\xDA\xE0\x7E\xE9\x28\x9C"
INT64: size

...??

### Advanced content encryption object

GUID: "\xB6\x9B\x07\x7A\xA4\xDA\x12\x4E\xA5\xCA\x91\xD3\x8D\xC1\x1A\x8D"
INT64: size

...??

### Language list object

GUID: 0xa9, 0x46, 0x43, 0x7c, 0xe0, 0xef, 0xfc, 0x4b, 0xb2, 0x29, 0x39, 0x3e, 0xde, 0x41, 0x5c, 0x85
INT64: size

INT16: count

...: languages

#### Language

UCHAR: language length
STR: language

### Extended stream header

GUID: 0xCB, 0xA5, 0xE6, 0x14, 0x72, 0xC6, 0x32, 0x43, 0x83, 0x99, 0xA9, 0x69, 0x52, 0x06, 0x5B, 0x5A
INT64: size

INT64: start time
INT64: end time
INT32: leak datarate
INT32: bucket datarate
INT32: init bucket fullness
INT32: alt leak datarate
INT32: alt bucket datarate
INT32: alt init bucket fullness
INT32: max object size
INT32: flags: reliable, seekable, no cleanpoints, resend live cleanpoints
UINT16: stream number
UINT16: stream language id index
UINT64: average framerate in 100 ns units
UINT16: stream name count
UINT16: payload extension system count

#### Streams

UINT16: ??
UINT16: len
??: of size len

#### Payload extensions

GUID: ?? only byte 0 significant as type ??
UINT16: size
UINT16: ext len
??: of size ext len

### Head 1 object

GUID: 0xb5, 0x03, 0xbf, 0x5f, 0x2E, 0xA9, 0xCF, 0x11, 0x8e, 0xe3, 0x00, 0xc0, 0x0c, 0x20, 0x53, 0x65
UINT64: size

GUID: ??
6B: ??

### Marker header object

GUID: 0x01, 0xCD, 0x87, 0xF4, 0x51, 0xA9, 0xCF, 0x11, 0x8E, 0xE6, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65
UINT64: size

16B: reserved
UINT32: count
2B: reserved
UINT16: name length

BYTES: name (length given by name length)

...: markers

#### Marker

8B: offset
UINT64: presentation time in 100ns intervals (take preroll into account)
UINT16: entry length
UINT32: send time
UINT32: flags
UINT32: name length
STR: name

### Digital signature object

GUID: 0xfc, 0xb3, 0x11, 0x22, 0x23, 0xbd, 0xd2, 0x11, 0xb4, 0xb7, 0x00, 0xa0, 0xc9, 0x55, 0xfc, 0x6e
UINT64: size

...: ??

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

### `AspectRatioX`
BOOL/INT16/INT32/INT64

### `AspectRatioY`
BOOL/INT16/INT32/INT64