use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum NewLocatorGPSMode {
    InternalGPS = 0,
    ExternalGPS,
    None,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct NewLocatorData {
    gps_mode: NewLocatorGPSMode,
}
