use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "3",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum ProtocolID {
    RD8999Pre1018 = 0,
    RD8999Post1018,
    RDMRX,
    RD8100,
    RD8200, // a guess
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum LeftRight {
    PeakMode = 0, // aka None
    LLLLL,
    LLLL,
    LLL,
    LL,
    L,
    Centre,
    R,
    RR,
    RRR,
    RRRR,
    RRRRR,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum AntennaMode {
    None = 0,
    AFrame,
    MarkerProbe,
    SingleProbe,
    SingleAntenna,
    CDClamp,
    CDStethoscope,
    DoubleAntenna,
    StandardStethoscope,
    UnknownAccessory,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum SondeLine {
    Line = 0,
    Sonde,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "3",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum BatteryLevel {
    High = 0,  // 3 bars
    Medium,    // 2 bars
    MediumLow, // 1 bar
    Low,       // 0 bars
    VeryLow,   // indicator flashing
    Critical,  // made up the label, seen in the wild on an RD8200
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "2",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum VolumeLevel {
    Off = 0, // 3 bars
    Minimum, // 2 bars
    Medium,  // 1 bar
    Maximum, // 0 bars
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "1",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum Overload {
    NoOverload = 0,
    Overload,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct AdditionalData {
    #[deku(bits = 3)]
    protocol: ProtocolID,
    #[deku(bits = 9)]
    compass_angle: u16,
    #[deku(bits = 4)]
    left_right: LeftRight,
    #[deku(bits = 4)]
    antenna_mode: AntennaMode,
    #[deku(bits = 4)]
    accessory_type: u8,
    #[deku(bits = 1)]
    sonde_line: SondeLine,
    #[deku(bits = 3)]
    battery_level: BatteryLevel,
    #[deku(bits = 2)]
    volume_level: VolumeLevel,
    #[deku(bits = 1)]
    overload: Overload,
    #[deku(bits = 1)]
    reserved: u8,
}
