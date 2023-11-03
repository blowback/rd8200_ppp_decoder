use crate::args;
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

#[allow(unused_must_use)]
impl fmt::Display for RDData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let args = args::Cli::get();

        write!(f, "[\n");

        if !args.no_rd {
            write!(f, "\tRD: {{{}}}\n", self.rd8k_data);
        }

        if !args.no_loc {
            write!(f, "\tLOC: {{{}}}\n", self.new_locator_data);
        }

        if !args.no_mrx {
            write!(f, "\tMRX: {{{}}}\n", self.mrx_data);
        }

        if !args.no_rtc {
            write!(f, "\tRTC: {{{}}}\n", self.rtc_data);
        }

        if !args.no_gps {
            write!(f, "\tGPS: {{{}}}\n", self.gps_data);
        }

        write!(f, "]")
    }
}
