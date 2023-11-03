use crate::args;
use deku::bitvec::*;
use deku::prelude::*;
use std::fmt;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "3",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum ProtocolID {
    RD8000Early = 0,
    RD8000Late,
    RDMRX,
    RD8100,
    RD8200, // a guess
    Unknown,
}

impl TryFrom<u8> for ProtocolID {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == ProtocolID::RD8000Early as u8 => Ok(ProtocolID::RD8000Early),
            x if x == ProtocolID::RD8000Late as u8 => Ok(ProtocolID::RD8000Late),
            x if x == ProtocolID::RDMRX as u8 => Ok(ProtocolID::RDMRX),
            x if x == ProtocolID::RD8100 as u8 => Ok(ProtocolID::RD8100),
            x if x == ProtocolID::RD8200 as u8 => Ok(ProtocolID::RD8200),
            _ => Ok(ProtocolID::Unknown),
        }
    }
}

impl fmt::Display for ProtocolID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProtocolID::RD8000Early => "RD8000Early",
                ProtocolID::RD8000Late => "RD8000Late",
                ProtocolID::RDMRX => "RDMRX",
                ProtocolID::RD8100 => "RD8100",
                ProtocolID::RD8200 => "RD8200",
                ProtocolID::Unknown => "Unknown",
            }
        )
    }
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
    Unknown,
}

impl TryFrom<u8> for LeftRight {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == LeftRight::PeakMode as u8 => Ok(LeftRight::PeakMode),
            x if x == LeftRight::LLLLL as u8 => Ok(LeftRight::LLLLL),
            x if x == LeftRight::LLLL as u8 => Ok(LeftRight::LLLL),
            x if x == LeftRight::LLL as u8 => Ok(LeftRight::LLL),
            x if x == LeftRight::LL as u8 => Ok(LeftRight::LL),
            x if x == LeftRight::L as u8 => Ok(LeftRight::L),
            x if x == LeftRight::Centre as u8 => Ok(LeftRight::Centre),
            x if x == LeftRight::R as u8 => Ok(LeftRight::R),
            x if x == LeftRight::RR as u8 => Ok(LeftRight::RR),
            x if x == LeftRight::RRR as u8 => Ok(LeftRight::RRR),
            x if x == LeftRight::RRRR as u8 => Ok(LeftRight::RRRR),
            x if x == LeftRight::RRRRR as u8 => Ok(LeftRight::RRRRR),
            _ => Ok(LeftRight::Unknown),
        }
    }
}

impl fmt::Display for LeftRight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LeftRight::PeakMode => "PeakMode",
                LeftRight::LLLLL => "LLLLL",
                LeftRight::LLLL => "LLLL",
                LeftRight::LLL => "LLL",
                LeftRight::LL => "LL",
                LeftRight::L => "L",
                LeftRight::Centre => "Centre",
                LeftRight::R => "R",
                LeftRight::RR => "RR",
                LeftRight::RRR => "RRR",
                LeftRight::RRRR => "RRRR",
                LeftRight::RRRRR => "RRRRR",
                LeftRight::Unknown => "Unknown",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum AntennaMode {
    Peak = 0,
    VerticalNone,
    Broad,
    All,
    None,
    Accessory,
    AFrame,
    PeakPlus,
    Guidance,
    Unknown,
}
impl TryFrom<u8> for AntennaMode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == AntennaMode::Peak as u8 => Ok(AntennaMode::Peak),
            x if x == AntennaMode::VerticalNone as u8 => Ok(AntennaMode::VerticalNone),
            x if x == AntennaMode::Broad as u8 => Ok(AntennaMode::Broad),
            x if x == AntennaMode::All as u8 => Ok(AntennaMode::All),
            x if x == AntennaMode::None as u8 => Ok(AntennaMode::None),
            x if x == AntennaMode::Accessory as u8 => Ok(AntennaMode::Accessory),
            x if x == AntennaMode::AFrame as u8 => Ok(AntennaMode::AFrame),
            x if x == AntennaMode::PeakPlus as u8 => Ok(AntennaMode::PeakPlus),
            x if x == AntennaMode::Guidance as u8 => Ok(AntennaMode::Guidance),
            _ => Ok(AntennaMode::Unknown),
        }
    }
}

impl fmt::Display for AntennaMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AntennaMode::Peak => "Peak",
                AntennaMode::VerticalNone => "VerticalNone",
                AntennaMode::Broad => "Broad",
                AntennaMode::All => "All",
                AntennaMode::None => "None",
                AntennaMode::Accessory => "Accessory",
                AntennaMode::AFrame => "AFrame",
                AntennaMode::PeakPlus => "PeakPlus",
                AntennaMode::Guidance => "Guidance",
                AntennaMode::Unknown => "Unknown",
            }
        )
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(
    type = "u8",
    bits = "4",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, bitsize: deku::ctx::BitSize"
)]
pub enum AccessoryType {
    None = 0,
    AFrame,
    MarkerProbe,
    SingleProbe,
    SingleAntenna,
    CDClamp,
    CDStethoscope,
    DoubleAntenna,
    StandardStethoscope,
    UnknownAccessory, // this matches a specific bit pattern
    Unknown,          // blanket case for all unmatched bit patterns
}

impl TryFrom<u8> for AccessoryType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == AccessoryType::None as u8 => Ok(AccessoryType::None),
            x if x == AccessoryType::AFrame as u8 => Ok(AccessoryType::AFrame),
            x if x == AccessoryType::MarkerProbe as u8 => Ok(AccessoryType::MarkerProbe),
            x if x == AccessoryType::SingleProbe as u8 => Ok(AccessoryType::SingleProbe),
            x if x == AccessoryType::SingleAntenna as u8 => Ok(AccessoryType::SingleAntenna),
            x if x == AccessoryType::CDClamp as u8 => Ok(AccessoryType::CDClamp),
            x if x == AccessoryType::CDStethoscope as u8 => Ok(AccessoryType::CDStethoscope),
            x if x == AccessoryType::DoubleAntenna as u8 => Ok(AccessoryType::DoubleAntenna),
            x if x == AccessoryType::StandardStethoscope as u8 => {
                Ok(AccessoryType::StandardStethoscope)
            }
            x if x == AccessoryType::UnknownAccessory as u8 => Ok(AccessoryType::UnknownAccessory),
            _ => Ok(AccessoryType::Unknown),
        }
    }
}

impl fmt::Display for AccessoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccessoryType::None => "None",
                AccessoryType::AFrame => "AFrame",
                AccessoryType::MarkerProbe => "MarkerProbe",
                AccessoryType::SingleProbe => "SingleProbe",
                AccessoryType::SingleAntenna => "SingleAntenna",
                AccessoryType::CDClamp => "CDClamp",
                AccessoryType::CDStethoscope => "CDStethoscope",
                AccessoryType::DoubleAntenna => "DoubleAntenna",
                AccessoryType::StandardStethoscope => "StandardStethoscope",
                AccessoryType::UnknownAccessory => "UnknownAccessory",
                AccessoryType::Unknown => "Unknown",
            }
        )
    }
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
    Unknown,
}

impl TryFrom<u8> for SondeLine {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == SondeLine::Line as u8 => Ok(SondeLine::Line),
            x if x == SondeLine::Sonde as u8 => Ok(SondeLine::Sonde),
            _ => Ok(SondeLine::Unknown),
        }
    }
}

impl TryFrom<bool> for SondeLine {
    type Error = ();

    fn try_from(v: bool) -> Result<Self, Self::Error> {
        match v {
            false => Ok(SondeLine::Line),
            true => Ok(SondeLine::Sonde),
        }
    }
}

impl fmt::Display for SondeLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SondeLine::Line => "Line",
                SondeLine::Sonde => "Sonde",
                SondeLine::Unknown => "Unknown",
            }
        )
    }
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
    Unknown,
    Critical, // made up the label, seen in the wild on an RD8200
}

impl TryFrom<u8> for BatteryLevel {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == BatteryLevel::High as u8 => Ok(BatteryLevel::High),
            x if x == BatteryLevel::Medium as u8 => Ok(BatteryLevel::Medium),
            x if x == BatteryLevel::MediumLow as u8 => Ok(BatteryLevel::MediumLow),
            x if x == BatteryLevel::Low as u8 => Ok(BatteryLevel::Low),
            x if x == BatteryLevel::VeryLow as u8 => Ok(BatteryLevel::VeryLow),
            x if x == BatteryLevel::Critical as u8 => Ok(BatteryLevel::Critical),
            _ => Ok(BatteryLevel::Unknown),
        }
    }
}

impl fmt::Display for BatteryLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BatteryLevel::High => "High",
                BatteryLevel::Medium => "Medium",
                BatteryLevel::MediumLow => "MediumLow",
                BatteryLevel::Low => "Low",
                BatteryLevel::VeryLow => "VeryLow",
                BatteryLevel::Critical => "Critical",
                BatteryLevel::Unknown => "Unknown",
            }
        )
    }
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
    Unknown,
}

impl TryFrom<u8> for VolumeLevel {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == VolumeLevel::Off as u8 => Ok(VolumeLevel::Off),
            x if x == VolumeLevel::Minimum as u8 => Ok(VolumeLevel::Minimum),
            x if x == VolumeLevel::Medium as u8 => Ok(VolumeLevel::Medium),
            x if x == VolumeLevel::Maximum as u8 => Ok(VolumeLevel::Maximum),
            _ => Ok(VolumeLevel::Unknown),
        }
    }
}

impl fmt::Display for VolumeLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VolumeLevel::Off => "Off",
                VolumeLevel::Minimum => "Minimum",
                VolumeLevel::Medium => "Medium",
                VolumeLevel::Maximum => "Maximum",
                VolumeLevel::Unknown => "Unknown",
            }
        )
    }
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
    Unknown,
}

impl TryFrom<u8> for Overload {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Overload::NoOverload as u8 => Ok(Overload::NoOverload),
            x if x == Overload::Overload as u8 => Ok(Overload::Overload),
            _ => Ok(Overload::Unknown),
        }
    }
}

impl TryFrom<bool> for Overload {
    type Error = ();

    fn try_from(v: bool) -> Result<Self, Self::Error> {
        match v {
            false => Ok(Overload::NoOverload),
            true => Ok(Overload::Overload),
        }
    }
}

impl fmt::Display for Overload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Overload::NoOverload => "Noverload",
                Overload::Overload => "OVERLOAD",
                Overload::Unknown => "Unknown",
            }
        )
    }
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
    accessory_type: AccessoryType,
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
    #[deku(skip)]
    orig: u32,
}

impl TryFrom<u32> for AdditionalData {
    type Error = ();

    fn try_from(u: u32) -> Result<Self, Self::Error> {
        let args = args::Cli::get();

        // let bits = BitSlice::<_, Lsb0>::from_element(&u);
        let u1: u32 = if args.big_endian {
            u.to_be()
        } else {
            u.to_le()
        };

        if args.debug > 1 {
            println!("u: {:02x}, u1: {:02x}", u, u1);
            println!("raw_ad={:02x}", u1);
        }

        let (prot, cmp, lr, am, at, s, bat, vol, ovl, _) = if args.msb0 {
            let bits = u1.view_bits::<Msb0>();

            (
                bits[29..=31].load::<u8>(),
                bits[20..=28].load::<u16>(),
                bits[16..=19].load::<u8>(),
                bits[12..=15].load::<u8>(),
                bits[8..=11].load::<u8>(),
                bits[7],
                bits[4..=6].load::<u8>(),
                bits[2..=3].load::<u8>(),
                bits[1],
                bits[0],
            )
        } else {
            let bits = u1.view_bits::<Lsb0>();

            (
                bits[29..=31].load::<u8>(),
                bits[20..=28].load::<u16>(),
                bits[16..=19].load::<u8>(),
                bits[12..=15].load::<u8>(),
                bits[8..=11].load::<u8>(),
                bits[7],
                bits[4..=6].load::<u8>(),
                bits[2..=3].load::<u8>(),
                bits[1],
                bits[0],
            )
        };

        if args.debug > 1 {
            println!("prot={:02x} cmp={:02x} lr={:02x} am={:02x} at={:02x} s={} bat={:02x} vol={:02x} ovl={}", prot, cmp, lr, am, at, s, bat, vol, ovl);
        }

        Ok(AdditionalData {
            protocol: ProtocolID::try_from(prot)?,
            compass_angle: cmp,
            left_right: LeftRight::try_from(lr)?,
            antenna_mode: AntennaMode::try_from(am)?,
            accessory_type: AccessoryType::try_from(at)?,
            sonde_line: SondeLine::try_from(s)?,
            battery_level: BatteryLevel::try_from(bat)?,
            volume_level: VolumeLevel::try_from(vol)?,
            overload: Overload::try_from(ovl)?,
            reserved: 0,
            orig: u,
        })
    }
}

impl fmt::Display for AdditionalData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let args = args::Cli::get();

        write!(
            f,
            "{} {}° lr={} ant={} acc={} {} bat={} vol={} {} {}",
            self.protocol,
            self.compass_angle,
            self.left_right,
            self.antenna_mode,
            self.accessory_type,
            self.sonde_line,
            self.battery_level,
            self.volume_level,
            self.overload,
            if args.debug > 1 {
                format!("{:02x}", self.orig)
            } else {
                format!("")
            },
        )
    }
}
