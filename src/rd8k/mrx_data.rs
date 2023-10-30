use deku::prelude::*;
use std::fmt;

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

impl fmt::Display for MRXMarkerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MRXMarkerType::Unused => "Unused",
                MRXMarkerType::Power => "Power",
                MRXMarkerType::Water => "Water",
                MRXMarkerType::SEBAPower => "SEBAPower",
                MRXMarkerType::Sanitary => "Sanitary",
                MRXMarkerType::Telephone => "Telephone",
                MRXMarkerType::Gas => "Gas",
                MRXMarkerType::CATV => "CATV",
                MRXMarkerType::NonPotable => "Non-Potable",
                MRXMarkerType::EDF => "EDF",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct MRXData {
    marker_type: MRXMarkerType,
    marker_depth: f32,
    marker_signal: f32,
    marker_gain: f32,
}

impl fmt::Display for MRXData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "typ:{} depth:{}m sig:{} gain:{}dB",
            self.marker_type, self.marker_depth, self.marker_signal, self.marker_gain
        )
    }
}
