use deku::prelude::*;

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
