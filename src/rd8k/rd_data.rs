use crate::rd8k::gps_data::GPSData;
use crate::rd8k::mrx_data::MRXData;
use crate::rd8k::new_locator_data::NewLocatorData;
use crate::rd8k::rd8k_data::RD8KData;
use crate::rd8k::rd_checksum::RDChecksum;
use crate::rd8k::rtc_data::RTCData;
use deku::prelude::*;
use std::fmt;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RDData {
    rd8k_data: RD8KData,
    new_locator_data: NewLocatorData,
    mrx_data: MRXData,
    rtc_data: RTCData,
    gps_data: GPSData,
    csum: RDChecksum,
}

impl fmt::Display for RDData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[\n\tRD:{}\n\tNEWLOC:{}\n\tMRX{}\n\tRTC:{}\n\tGPS:{}\n]",
            self.rd8k_data, self.new_locator_data, self.mrx_data, self.rtc_data, self.gps_data
        )
    }
}
