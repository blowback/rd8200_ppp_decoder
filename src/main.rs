use anyhow::Result;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crc::{Algorithm, Crc, CRC_16_IBM_SDLC};
use deku::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: std::path::PathBuf,
}

// The basic frame suitable for checksumming, i.e.
// we're not parsing any of the deeper structure
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PPPBytes {
    #[deku(assert_eq = "0x7e")]
    flag_sequence: u8,
    #[deku(assert_eq = "0xff")]
    address: u8,
    #[deku(assert_eq = "0x03")]
    control: u8,
    #[deku(endian = "big")]
    #[deku(assert_eq = "0x0021")]
    protocol: u16,
    #[deku(assert_eq = "0x01")]
    command: u8,
    response: u8,
    length: u16,
    #[deku(count = "length / 4")]
    data: Vec<u32>,
    fcs: u16,
    // RD8200 brackets every packet with FS, ie FS starts and
    // ends the packet, unlike the more conventional fence-post
    // arrangement where this is only one FS between subsequent
    // packets.
    #[deku(assert_eq = "0x7e")]
    flag_sequence2: u8,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PPPFrame {
    #[deku(assert_eq = "0x7e")]
    flag_sequence: u8,
    #[deku(assert_eq = "0xff")]
    address: u8,
    #[deku(assert_eq = "0x03")]
    control: u8,
    #[deku(endian = "big")]
    #[deku(assert_eq = "0x0021")]
    protocol: u16,
    #[deku(assert_eq = "0x01")]
    command: u8,
    response: u8,
    length: u16,
    //#[deku(count = "length")]
    //data: Vec<u8>,
    data: RDData,
    fcs: u16,
    // RD8200 brackets every packet with FS, ie FS starts and
    // ends the packet, unlike the more conventional fence-post
    // arrangement where this is only one FS between subsequent
    // packets.
    #[deku(assert_eq = "0x7e")]
    flag_sequence2: u8,
}

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

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum RD8KMode {
    NotDefined = 0,
    Active,
    Radio,
    Power,
    FaultFind8K,
    FaultFindCD,
    CD,
    ACD,
    PassiveAvoidance,
    CPS,
    CATV,
    ELF,
    Sonde,
    CDInverted,
    Marker,
    ActiveCustom,
    ActivePower,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RD8KData {
    index: u32,
    mode: RD8KMode,
    frequency: f32,
    log_id: u32,
    additional_data: u32,
    depth: f32,
    ff_signal: f32,
    current: f32,
    cd_phase: f32,
    signal_strength: f32,
    gain: f32,
}

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

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum MRXMarkerType {
    Unused = 0,
    Power,
    Water,
    SEBAPower,
    Sanitary,
    Telephone,
    Gas,
    CATV,
    NonPotable,
    EDF,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct MRXData {
    marker_type: MRXMarkerType,
    marker_depth: f32,
    marker_signal: f32,
    marker_gain: f32,
}

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

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RDChecksum {
    csum: u32,
}

fn fcs(data: &PPPBytes) -> bool {
    if let Ok(v) = data.to_bytes() {
        let fcs: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
        let b = v.as_slice();
        let l = b.len();
        let pcs: u16 = (b[l - 2] as u16) << 8 | b[l - 3] as u16;
        let acs = fcs.checksum(&b[1..(l - 3)]);

        if pcs != acs {
            println!("Primary FCS failure: pcs: {:04x} != acs: {:04x}", pcs, acs);
            return false;
        } else {
            return true;
        }
    } else {
        println!("Error encoding PPP frame");
    }
    return false;
}

fn csum(data: &mut Vec<u32>) -> bool {
    let pcs: u64 = data.pop().unwrap() as u64;
    let acs: u64 = data.iter().fold(0, |acc, x| acc + *x as u64);

    if acs & 0xffffffff != pcs & 0xffffffff {
        println!(
            "Secondary csum failure: pcs: {:04x} != acs: {:04x}",
            pcs, acs
        );
        return false;
    }
    return true;
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut data = std::fs::read(args.path)?;
    println!("Data: {:02x?}", data);

    let mut buf = data.as_slice();
    let mut next_ptr = (buf, 0);

    loop {
        let mut ptr = next_ptr;
        let mut fcs_ok = false;
        let mut csum_ok = false;

        match PPPBytes::from_bytes(ptr) {
            Ok((rest, mut undecoded)) => {
                next_ptr = rest;
                fcs_ok = fcs(&undecoded);
                csum_ok = csum(&mut undecoded.data);
            }
            Err(e) => {
                println!("Error reading raw frame: {}", e);

                let tmp = next_ptr.0;

                if tmp.len() > 0 {
                    next_ptr = (&tmp[1..], 0);
                } else {
                    break;
                }
            }
        }

        if fcs_ok && csum_ok {
            match PPPFrame::from_bytes(ptr) {
                Ok((rest, frame)) => {
                    println!("decoded: {:#02x?}", frame);
                }
                Err(e) => {
                    println!("error decoding frame: {}", e);
                }
            }
        } else {
            println!("dropping frame (FCS/csum failure)");
        }
    }

    Ok(())
}
