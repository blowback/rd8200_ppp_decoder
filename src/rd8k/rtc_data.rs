use deku::prelude::*;
use hifitime::prelude::*;
use std::fmt;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RTCDate {
    year: u16,
    month: u8,
    day: u8,
}

impl RTCDate {
    // RTCDate is just a stored version of an earlier GPS date, thus it suffers
    // from the same problems outlined in gps_data.rs
    pub fn correct_rtc_date(&self) -> Self {
        // we know the year must be 2023 or greater...
        const REF_YEAR: u16 = 2023;

        if self.year < REF_YEAR {
            let e = Epoch::from_gregorian_utc_at_midnight(self.year.into(), self.month, self.day);
            let m = Duration::from_days(7.0 * 1024.0);
            let e2 = e + m;
            let (year, month, day, _, _, _, _) = e2.to_gregorian_utc();
            Self {
                year: year as u16,
                month,
                day,
            }
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for RTCDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let corrected = self.correct_rtc_date();
        write!(
            f,
            "{}-{}-{}",
            corrected.year, corrected.month, corrected.day
        )
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
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
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
