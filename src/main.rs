use anyhow::Result;

use crc::{Crc, CRC_16_IBM_SDLC};

use deku::prelude::*;
use rd8200_ppp_decoder::args;
use rd8200_ppp_decoder::rd8k::byte_frame::PPPBytes;
use rd8200_ppp_decoder::rd8k::ppp_frame::PPPFrame;

// Calculate the RFC-1662 16-bit CRC over all bytes of the packet.
fn fcs(data: &PPPBytes) -> bool {
    if let Ok(v) = data.to_bytes() {
        let fcs: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
        let b = v.as_slice();
        let l = b.len();
        let pcs: u16 = (b[l - 2] as u16) << 8 | b[l - 3] as u16;
        let acs = fcs.checksum(&b[1..(l - 3)]);

        if pcs != acs {
            eprintln!("Primary FCS failure: pcs: {:04x} != acs: {:04x}", pcs, acs);
            return false;
        } else {
            return true;
        }
    } else {
        println!("Error encoding PPP frame");
    }
    return false;
}

// Calculate the simple additive checksum over the RD data fields.
fn csum(data: &mut Vec<u32>) -> bool {
    let pcs: u64 = data.pop().unwrap() as u64;
    let acs: u64 = data.iter().fold(0, |acc, x| acc + *x as u64);

    if acs & 0xffffffff != pcs & 0xffffffff {
        eprintln!(
            "Secondary csum failure: pcs: {:04x} != acs: {:04x}",
            pcs, acs
        );
        return false;
    }
    return true;
}

// Check the integrity of RD82000 PPP packets, and display them in decoded form.
fn main() -> Result<()> {
    let args = args::Cli::get();
    for path in args.paths.iter() {
        let data = std::fs::read(path.clone())?;

        if args.debug > 2 {
            println!("Data: {:02x?}", data);
        }

        let buf = data.as_slice();
        let mut next_ptr = (buf, 0);

        let mut pkt_idx = 0;
        loop {
            let ptr = next_ptr;
            #[allow(unused_assignments)]
            let mut fcs_ok = false;
            #[allow(unused_assignments)]
            let mut csum_ok = false;

            match PPPBytes::from_bytes(ptr) {
                Ok((rest, mut undecoded)) => {
                    next_ptr = rest;
                    fcs_ok = fcs(&undecoded);
                    csum_ok = csum(&mut undecoded.data);
                }
                Err(e) => {
                    eprintln!("Error reading raw frame: {}", e);

                    let tmp = next_ptr.0;

                    if tmp.len() > 0 {
                        next_ptr = (&tmp[1..], 0);
                        continue;
                    } else {
                        break;
                    }
                }
            }

            if fcs_ok && csum_ok {
                match PPPFrame::from_bytes(ptr) {
                    Ok((_rest, frame)) => {
                        println!("frame: {pkt_idx}: {}", frame.data);
                    }
                    Err(e) => {
                        println!("frame: {pkt_idx} | error decoding frame: {}", e);
                    }
                }
            } else {
                println!("frame: {pkt_idx} | dropping frame (FCS/csum failure)");
            }
            pkt_idx += 1;
        }
    }

    Ok(())
}
