use deku::prelude::*;
use std::fmt;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct GPSDate {
    year: u16,
    month: u8,
    day: u8,
}

impl fmt::Display for GPSDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
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
        write!(
            f,
            "{} {} hdop:{} alt:{}{} height:{}{} fix:{} #sats:{} lat:{}° lon:{}°",
            self.gps_date,
            self.utc,
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
