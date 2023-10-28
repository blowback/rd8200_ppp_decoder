use crate::rd8k::rd_data::RDData;
use deku::prelude::*;

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
    pub data: RDData,
    fcs: u16,
    // RD8200 brackets every packet with FS, ie FS starts and
    // ends the packet, unlike the more conventional fence-post
    // arrangement where this is only one FS between subsequent
    // packets.
    #[deku(assert_eq = "0x7e")]
    flag_sequence2: u8,
}
