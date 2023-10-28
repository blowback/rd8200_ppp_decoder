use crate::rd8k::gps_data::GPSData;
use crate::rd8k::mrx_data::MRXData;
use crate::rd8k::new_locator_data::NewLocatorData;
use crate::rd8k::rd8k_data::RD8KData;
use crate::rd8k::rd_checksum::RDChecksum;
use crate::rd8k::rtc_data::RTCData;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RDData {
    rdk8_data: RD8KData,
    new_locator_data: NewLocatorData,
    mrx_data: MRXData,
    rtc_data: RTCData,
    gps_data: GPSData,
    csum: RDChecksum,
}
