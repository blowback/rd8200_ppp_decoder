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
    RD8900Early = 0,
    RD8900Late,
    RDMRX,
    RD8100,
    RD8200, // a guess
}

impl TryFrom<u8> for ProtocolID {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == ProtocolID::RD8900Early as u8 => Ok(ProtocolID::RD8900Early),
            x if x == ProtocolID::RD8900Late as u8 => Ok(ProtocolID::RD8900Late),
            x if x == ProtocolID::RDMRX as u8 => Ok(ProtocolID::RDMRX),
            x if x == ProtocolID::RD8100 as u8 => Ok(ProtocolID::RD8100),
            x if x == ProtocolID::RD8200 as u8 => Ok(ProtocolID::RD8200),
            _ => {
                println!("Invalid ProtocolID: {:02x}", v);
                Err(())
            }
        }
    }
}

impl fmt::Display for ProtocolID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProtocolID::RD8900Early => "RD8900Early",
                ProtocolID::RD8900Late => "RD8900Late",
                ProtocolID::RDMRX => "RDMRX",
                ProtocolID::RD8100 => "RD8100",
                ProtocolID::RD8200 => "RD8200",
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
            _ => {
                println!("Invalid LeftRight: {:02x}", v);
                Err(())
            }
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
            _ => {
                println!("Invalid AntennaMode: {:02x}", v);
                Err(())
            }
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
    UnknownAccessory,
    UnknownAccessory1,
    UnknownAccessory2,
    UnknownAccessory3,
    UnknownAccessory4,
    UnknownAccessory5,
    UnknownAccessory6,
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
            x if x == AccessoryType::UnknownAccessory1 as u8 => {
                Ok(AccessoryType::UnknownAccessory1)
            }
            x if x == AccessoryType::UnknownAccessory2 as u8 => {
                Ok(AccessoryType::UnknownAccessory2)
            }
            x if x == AccessoryType::UnknownAccessory3 as u8 => {
                Ok(AccessoryType::UnknownAccessory3)
            }
            x if x == AccessoryType::UnknownAccessory4 as u8 => {
                Ok(AccessoryType::UnknownAccessory4)
            }
            x if x == AccessoryType::UnknownAccessory5 as u8 => {
                Ok(AccessoryType::UnknownAccessory5)
            }
            x if x == AccessoryType::UnknownAccessory6 as u8 => {
                Ok(AccessoryType::UnknownAccessory6)
            }
            _ => {
                println!("Invalid AccessoryType: {:02x}", v);
                Err(())
            }
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
                AccessoryType::UnknownAccessory1 => "UnknownAccessory1",
                AccessoryType::UnknownAccessory2 => "UnknownAccessory2",
                AccessoryType::UnknownAccessory3 => "UnknownAccessory3",
                AccessoryType::UnknownAccessory4 => "UnknownAccessory4",
                AccessoryType::UnknownAccessory5 => "UnknownAccessory5",
                AccessoryType::UnknownAccessory6 => "UnknownAccessory6",
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
}

impl TryFrom<u8> for SondeLine {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == SondeLine::Line as u8 => Ok(SondeLine::Line),
            x if x == SondeLine::Sonde as u8 => Ok(SondeLine::Sonde),
            _ => {
                println!("Invalid Sonde/Line: {:02x}", v);
                Err(())
            }
        }
    }
}

impl TryFrom<bool> for SondeLine {
    type Error = ();

    fn try_from(v: bool) -> Result<Self, Self::Error> {
        match v {
            false => Ok(SondeLine::Line),
            true => Ok(SondeLine::Sonde),
            _ => Err(()),
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
    Critical,  // made up the label, seen in the wild on an RD8200
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
            _ => {
                println!("Invalid BatteryLevel: {:02x}", v);
                Err(())
            }
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
}

impl TryFrom<u8> for VolumeLevel {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == VolumeLevel::Off as u8 => Ok(VolumeLevel::Off),
            x if x == VolumeLevel::Minimum as u8 => Ok(VolumeLevel::Minimum),
            x if x == VolumeLevel::Medium as u8 => Ok(VolumeLevel::Medium),
            x if x == VolumeLevel::Maximum as u8 => Ok(VolumeLevel::Maximum),
            _ => {
                println!("Invalid VolumeLevel: {:02x}", v);
                Err(())
            }
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
}

impl TryFrom<u8> for Overload {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Overload::NoOverload as u8 => Ok(Overload::NoOverload),
            x if x == Overload::Overload as u8 => Ok(Overload::Overload),
            _ => {
                println!("Invalid Overload: {:02x}", v);
                Err(())
            }
        }
    }
}

impl TryFrom<bool> for Overload {
    type Error = ();

    fn try_from(v: bool) -> Result<Self, Self::Error> {
        match v {
            false => Ok(Overload::NoOverload),
            true => Ok(Overload::Overload),
            _ => Err(()),
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
}

impl TryFrom<u32> for AdditionalData {
    type Error = ();

    fn try_from(u: u32) -> Result<Self, Self::Error> {
        let bits = BitSlice::<_, Msb0>::from_element(&u);

        let prot = bits[29..=31].load::<u8>();
        let cmp = bits[20..=28].load::<u16>();
        let lr = bits[16..=19].load::<u8>();
        let am = bits[12..=15].load::<u8>();
        let at = bits[8..=11].load::<u8>();
        let s = bits[7];
        let bat = bits[4..=6].load::<u8>();
        let vol = bits[2..=3].load::<u8>();
        let ovl = bits[1];
        let _ = bits[0];

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
        })
    }
}

impl fmt::Display for AdditionalData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}° {} {} {} {} {} {} {}",
            self.protocol,
            self.compass_angle,
            self.left_right,
            self.antenna_mode,
            self.accessory_type,
            self.sonde_line,
            self.battery_level,
            self.volume_level,
            self.overload
        )
    }
}
