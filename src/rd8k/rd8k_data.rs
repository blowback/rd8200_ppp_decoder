use crate::rd8k::additional_data::AdditionalData;
use deku::prelude::*;
use std::fmt;

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

impl fmt::Display for RD8KMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RD8KMode::NotDefined => "NotDefined",
                RD8KMode::Active => "Active",
                RD8KMode::Radio => "Radio",
                RD8KMode::Power => "Power",
                RD8KMode::FaultFind8K => "FaultFind8k",
                RD8KMode::FaultFindCD => "FaultFindCD",
                RD8KMode::CD => "CD",
                RD8KMode::ACD => "ACD",
                RD8KMode::PassiveAvoidance => "PassiveAvoidance",
                RD8KMode::CPS => "CPS",
                RD8KMode::CATV => "CATV",
                RD8KMode::ELF => "ELF",
                RD8KMode::Sonde => "Sonde",
                RD8KMode::CDInverted => "CDInverted",
                RD8KMode::Marker => "Marker",
                RD8KMode::ActiveCustom => "ActiveCustom",
                RD8KMode::ActivePower => "ActivePower",
            }
        )
    }
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

impl fmt::Display for RD8KData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(ad) = AdditionalData::try_from(u32::from_le(self.additional_data)) {
            write!(f, "mode: {:?}, freq: {} Hz, depth: {}m, FF: {} dBm, cur: {} A, CD: {}°, SS: {}, gain: {} dB xtra:[{}]", self.mode, self.frequency, self.depth, self.ff_signal, self.current, self.cd_phase, self.signal_strength, self.gain, ad)
        } else {
            write!(f, "mode: {:?}, freq: {} Hz, depth: {}m, FF: {} dBm, cur: {} A, CD: {}°, SS: {}, gain: {} dB", self.mode, self.frequency, self.depth, self.ff_signal, self.current, self.cd_phase, self.signal_strength, self.gain)
        }
    }
}
