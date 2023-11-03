use deku::prelude::*;
use hifitime::prelude::*;
use std::fmt;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct GPSDate {
    year: u16,
    month: u8,
    day: u8,
}

impl GPSDate {
    // The RD8200 takes account of the first GPS rollover in august 1999,
    // but it did not anticipate the second rollver of april 2019, therefore
    // any times reported after that are off by up to 20 years.
    pub fn correct_gps_date(&self) -> Self {
        // we know the year must be 2023 or greater...
        const REF_YEAR: u16 = 2023;

        // no need to apply this repeatedly as the RD8200 already compensates
        // for dates after 1999, and we won't need another iteration until 2038
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

impl fmt::Display for GPSDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let corrected = self.correct_gps_date();
        write!(
            f,
            "{}-{}-{}",
            corrected.year, corrected.month, corrected.day
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum GeoidUnits {
    Invalid = 0,
    Metres,
}

impl fmt::Display for GeoidUnits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeoidUnits::Invalid => "???",
                GeoidUnits::Metres => "m",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum AltitudeUnits {
    Invalid = 0,
    Metres,
}

impl fmt::Display for AltitudeUnits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AltitudeUnits::Invalid => "???",
                AltitudeUnits::Metres => "m",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum GPSFix {
    None = 0,
    GPS,
    DGPS,
    PPS,
    Kinematic,
    RTK,
    Estimated,
    Manual,
    Simulation,
}

impl fmt::Display for GPSFix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GPSFix::None => "None",
                GPSFix::GPS => "GPS",
                GPSFix::DGPS => "DGPS",
                GPSFix::PPS => "PPS",
                GPSFix::Kinematic => "Kinematic",
                GPSFix::RTK => "RTK",
                GPSFix::Estimated => "Estimated",
                GPSFix::Manual => "Manual",
                GPSFix::Simulation => "Simulation",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum TimeIndicator {
    System = 0,
    GPS,
    Unused,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct GPSData {
    gps_date: GPSDate,
    utc: f32,
    hdop: f32,
    altitude: f32,
    geoid_height: f32,
    dgps_time: u32,
    dgps_id: u32,
    geoid_units: GeoidUnits,
    gps_fix: GPSFix,
    num_sats: u32,
    altitude_units: AltitudeUnits,
    latitude: f64,
    longitude: f64,
    time_indicator: TimeIndicator,
}

impl fmt::Display for GPSData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hh, mm, ss) = self.decompose_time();
        write!(
            f,
            "{} {:02}:{:02}:{:02} hdop:{} alt:{}{} height:{}{} fix:{} #sats:{} lat:{}° lon:{}°",
            self.gps_date,
            hh,
            mm,
            ss,
            self.hdop,
            self.altitude,
            self.altitude_units,
            self.geoid_height,
            self.geoid_units,
            self.gps_fix,
            self.num_sats,
            self.latitude,
            self.longitude
        )
    }
}

impl GPSData {
    pub fn decompose_time(&self) -> (u8, u8, u8) {
        fn divrem(u: f32) -> (f32, u8) {
            (u / 100.0, ((u % 100.0).floor()) as u8)
        }

        let (rest, secs) = divrem(self.utc);
        let (rest, mins) = divrem(rest);
        let (_rest, hours) = divrem(rest);
        (hours, mins, secs)
    }
}
