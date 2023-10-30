use deku::prelude::*;
use std::fmt;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum NewLocatorGPSMode {
    InternalGPS = 0,
    ExternalGPS,
    None,
}

impl fmt::Display for NewLocatorGPSMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NewLocatorGPSMode::InternalGPS => "Internal GPS",
                NewLocatorGPSMode::ExternalGPS => "External GPS",
                NewLocatorGPSMode::None => "No GPS",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct NewLocatorData {
    gps_mode: NewLocatorGPSMode,
}

impl fmt::Display for NewLocatorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.gps_mode)
    }
}
