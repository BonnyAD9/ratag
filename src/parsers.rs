use std::{num::ParseIntError, str::FromStr, time::Duration};

use encoding::{
    Encoding,
    all::{ASCII, ISO_8859_1, UTF_8, UTF_16BE, UTF_16LE},
};

use crate::{
    Error, Result,
    trap::{Trap, TrapExt},
};

pub struct DateTime {
    pub year: Option<i32>,
    pub date: Option<(u8, u8)>,
    pub time: Option<Duration>,
}

pub fn syncsafe_be_u32(d: &[u8; 4]) -> u32 {
    (d[0] as u32) << 21
        | (d[1] as u32) << 14
        | (d[2] as u32) << 7
        | d[3] as u32
}

pub fn be_u24(d: &[u8; 3]) -> u32 {
    (d[0] as u32) << 16 | (d[1] as u32) << 8 | d[2] as u32
}

pub fn ascii(d: &[u8], trap: &impl Trap) -> Result<String> {
    ASCII
        .decode(d, trap.decoder_trap())
        .map_err(|_| Error::InvalidEncoding)
}

pub fn iso_8859_1(d: &[u8], trap: &impl Trap) -> Result<String> {
    ISO_8859_1
        .decode(d, trap.decoder_trap())
        .map_err(|_| Error::InvalidEncoding)
}

pub fn iso_8859_1_mnt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = d
        .iter()
        .position(|a| *a == 0)
        .map(|e| (e, e + 1))
        .unwrap_or((d.len(), d.len()));
    Ok((len, iso_8859_1(&d[..end], trap)?))
}

pub fn iso_8859_1_nt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = if let Some(end) = d.iter().position(|a| *a == 0) {
        (end, end + 1)
    } else {
        trap.error(Error::StringNotTerminated)?;
        (d.len(), d.len())
    };
    Ok((len, iso_8859_1(&d[..end], trap)?))
}

pub fn iso_8859_1_to(
    dst: &mut String,
    d: &[u8],
    trap: &impl Trap,
) -> Result<()> {
    ISO_8859_1
        .decode_to(d, trap.decoder_trap(), dst)
        .map_err(|_| Error::InvalidEncoding)
}

pub fn iso_8859_1_mnt_to(
    dst: &mut String,
    d: &[u8],
    trap: &impl Trap,
) -> Result<usize> {
    let (end, len) = d
        .iter()
        .position(|a| *a == 0)
        .map(|e| (e, e + 1))
        .unwrap_or((d.len(), d.len()));
    iso_8859_1_to(dst, &d[..end], trap)?;
    Ok(len)
}

pub fn utf_16_be(d: &[u8], trap: &impl Trap) -> Result<String> {
    UTF_16BE
        .decode(d, trap.decoder_trap())
        .map_err(|_| Error::InvalidEncoding)
}

pub fn utf_16_be_mnt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = d
        .chunks(2)
        .position(|d| *d == [0, 0])
        .map(|e| (e * 2, e * 2 + 2))
        .unwrap_or((d.len(), d.len()));
    Ok((len, utf_16_be(&d[..end], trap)?))
}

pub fn utf_16_be_nt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = if let Some(end) = d.chunks(2).position(|d| *d == [0, 0])
    {
        let e = end * 2;
        (e, e + 2)
    } else {
        trap.error(Error::StringNotTerminated)?;
        (d.len(), d.len())
    };
    Ok((len, utf_16_be(&d[..end], trap)?))
}

pub fn utf_16_le(d: &[u8], trap: &impl Trap) -> Result<String> {
    UTF_16LE
        .decode(d, trap.decoder_trap())
        .map_err(|_| Error::InvalidEncoding)
}

pub fn utf_16_le_mnt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = d
        .chunks(2)
        .position(|d| *d == [0, 0])
        .map(|e| (e * 2, e * 2 + 2))
        .unwrap_or((d.len(), d.len()));
    Ok((len, utf_16_le(&d[..end], trap)?))
}

pub fn utf_16_le_nt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = if let Some(end) = d.chunks(2).position(|d| *d == [0, 0])
    {
        let e = end * 2;
        (e, e + 2)
    } else {
        trap.error(Error::StringNotTerminated)?;
        (d.len(), d.len())
    };
    Ok((len, utf_16_le(&d[..end], trap)?))
}

pub fn utf_16_bom(d: &[u8], trap: &impl Trap) -> Result<String> {
    match d {
        [] => Ok(String::new()),
        [0xfe, 0xff, d @ ..] => utf_16_be(d, trap),
        [0xff, 0xfe, d @ ..] => utf_16_le(d, trap),
        _ => Err(Error::InvalidEncoding),
    }
}

pub fn utf_16_bom_mnt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (l, s) = match d {
        [] => return Ok((0, String::new())),
        [0, 0, ..] => return Ok((2, String::new())),
        [0xfe, 0xff, d @ ..] => utf_16_be_mnt(d, trap)?,
        [0xff, 0xfe, d @ ..] => utf_16_le_mnt(d, trap)?,
        _ => return Err(Error::InvalidEncoding),
    };
    Ok((l + 2, s))
}

pub fn utf_16_bom_nt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (l, s) = match d {
        [] => return Ok((0, String::new())),
        [0, 0, ..] => return Ok((2, String::new())),
        [0xfe, 0xff, d @ ..] => utf_16_be_nt(d, trap)?,
        [0xff, 0xfe, d @ ..] => utf_16_le_nt(d, trap)?,
        _ => return Err(Error::InvalidEncoding),
    };
    Ok((l + 2, s))
}

pub fn utf_8(d: &[u8], trap: &impl Trap) -> Result<String> {
    UTF_8
        .decode(d, trap.decoder_trap())
        .map_err(|_| Error::InvalidEncoding)
}

pub fn utf_8_mnt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (end, len) = d
        .iter()
        .position(|a| *a == 0)
        .map(|e| (e, e + 1))
        .unwrap_or((d.len(), d.len()));
    Ok((len, utf_8(&d[..end], trap)?))
}

pub fn utf_8_nt(d: &[u8], trap: &impl Trap) -> Result<(usize, String)> {
    let (len, end) = if let Some(e) = d.iter().position(|a| *a == 0) {
        (e + 1, e)
    } else {
        trap.error(Error::StringNotTerminated)?;
        (d.len(), d.len())
    };
    Ok((len, utf_8(&d[..end], trap)?))
}

pub fn num<T: FromStr<Err = ParseIntError>>(s: &str) -> Result<T> {
    Ok(s.parse()?)
}

pub fn num_of(s: &str, trap: &impl Trap) -> Result<(u32, Option<u32>)> {
    if let Some((n, o)) = s.split_once('/') {
        let n = num(n)?;
        let o = trap.res(num(o))?;
        Ok((n, o))
    } else {
        let n = num(s)?;
        Ok((n, None))
    }
}

pub fn year(s: &str, trap: &impl Trap) -> Result<DateTime> {
    let (d, t) = s
        .split_once('T')
        .map(|(d, t)| (d, Some(t)))
        .unwrap_or((s, None));

    let (y, d) = if let Some((y, d)) = d.split_once('-') {
        (y, Some(d))
    } else if d.len() != 8 {
        (d, None)
    } else {
        (&d[..4], Some(&d[4..]))
    };

    let year = trap.res(num(y))?;

    let date = if let Some(d) = d {
        trap.res(date_only(d))?
    } else {
        None
    };

    let time = if let Some(t) = t {
        trap.res(time_only(t))?
    } else {
        None
    };

    Ok(DateTime { year, date, time })
}

pub fn date(s: &str, trap: &impl Trap) -> Result<DateTime> {
    let (d, t) = s
        .split_once('T')
        .map(|(d, t)| (d, Some(t)))
        .unwrap_or((s, None));

    let (y, d) = if let Some((y, d)) = d.split_once('-')
        && d.contains('-')
    {
        (Some(y), d)
    } else if matches!(d.len(), 4 | 2) {
        (None, d)
    } else if d.len() == 8 {
        (Some(&d[..4]), &d[4..])
    } else {
        return Err(Error::InvalidDate);
    };

    let year = if let Some(y) = y {
        trap.res(num(y))?
    } else {
        None
    };

    let date = trap.res(date_only(d))?;

    let time = if let Some(t) = t {
        trap.res(time_only(t))?
    } else {
        None
    };

    Ok(DateTime { year, date, time })
}

pub fn date_only(s: &str) -> Result<(u8, u8)> {
    let (m, d) = if let Some((m, d)) = s.split_once('-') {
        (m, Some(d))
    } else if s.len() == 4 {
        (&s[..2], Some(&s[2..]))
    } else if s.len() == 2 {
        (s, None)
    } else {
        return Err(Error::InvalidDate);
    };

    let month = num(m)?;

    let day = if let Some(d) = d { num(d)? } else { 0 };

    Ok((month, day))
}

pub fn time_only(mut s: &str) -> Result<Duration> {
    s = s.strip_suffix('Z').unwrap_or(s);
    let (h, ms) = if let Some((h, ms)) = s.split_once(':') {
        (h, Some(ms))
    } else if s.len() == 4 {
        (&s[..2], Some(&s[2..]))
    } else if s.len() == 2 {
        (s, None)
    } else {
        return Err(Error::InvalidTime);
    };

    let mut secs = num::<u64>(h)? * 3600;

    if let Some(ms) = ms {
        let (m, s) = ms
            .split_once(':')
            .map(|(m, s)| (m, Some(s)))
            .unwrap_or((ms, None));

        secs += num::<u64>(m)? * 60;

        if let Some(s) = s {
            secs += num::<u64>(s)?;
        }
    }

    Ok(Duration::from_secs(secs))
}
