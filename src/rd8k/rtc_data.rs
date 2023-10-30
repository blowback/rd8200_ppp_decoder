use deku::prelude::*;
use std::fmt;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RTCDate {
    year: u16,
    month: u8,
    day: u8,
}

impl fmt::Display for RTCDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RTCTime {
    hour: u8,
    minute: u8,
    second: u8,
    pad: u8,
}

impl fmt::Display for RTCTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum RTCUpdated {
    NotUpdated = 0,
    Updated,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RTCData {
    rtc_date: RTCDate,
    rtc_time: RTCTime,
    rtc_updated: RTCUpdated,
}

impl fmt::Display for RTCData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.rtc_date, self.rtc_time,)
    }
}
