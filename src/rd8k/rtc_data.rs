use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RTCDate {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RTCTime {
    hour: u8,
    minute: u8,
    second: u8,
    pad: u8,
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
