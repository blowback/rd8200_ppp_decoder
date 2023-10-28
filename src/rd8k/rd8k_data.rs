use crate::rd8k::additional_data::AdditionalData;
use deku::prelude::*;

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
    additional_data: AdditionalData,
    depth: f32,
    ff_signal: f32,
    current: f32,
    cd_phase: f32,
    signal_strength: f32,
    gain: f32,
}
