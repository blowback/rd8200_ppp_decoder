use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct GPSDate {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum GeoidUnits {
    Invalid = 0,
    Metres,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum AltitudeUnits {
    Invalid = 0,
    Metres,
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
