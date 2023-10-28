use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct RDChecksum {
    csum: u32,
}
