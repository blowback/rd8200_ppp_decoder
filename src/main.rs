use anyhow::{bail, Result};

use crc::{Crc, CRC_16_IBM_SDLC};

use deku::prelude::*;
use rd8200_ppp_decoder::args;
use rd8200_ppp_decoder::rd8k::byte_frame::PPPBytes;
use rd8200_ppp_decoder::rd8k::ppp_frame::PPPFrame;

// Undo RFC-1662 octet stuffing
fn unescape(input: &mut [u8]) -> Result<usize> {
    let mut escape: bool = false;
    let mut j: usize = 0;
    let mut n: usize = 0;

    for i in 0..input.len() {
        let c: u8 = input[i];

        if c == 0x7d {
            escape = true;
        } else {
            if escape {
                input[j] = match c {
                    0x5e => 0x7e,
                    0x5d => 0x7d,
                    0x7e => bail!("Abort Sequence detected!"),
                    _ => c ^ 0x20u8,
                };
                escape = false;
                n += 1;
            } else {
                input[j] = c;
            }
            j += 1;
        }
    }
    Ok(n)
}

// Calculate the RFC-1662 16-bit CRC over all bytes of the packet.
fn fcs(data: &PPPBytes) -> bool {
    if let Ok(v) = data.to_bytes() {
        let fcs: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
        let b = v.as_slice();
        let l = b.len();
        let pcs: u16 = data.fcs;
        let acs = fcs.checksum(&b[0..(l - 2)]);

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
        println!("File: {}", path.display());

        let mut data = std::fs::read(path.clone())?;

        if args.debug > 2 {
            println!("Data: {:02x?}", data);
        }

        let buf: &mut [u8] = data.as_mut_slice();
        let mut pkt_idx = 0;

        for pkt in buf.split_mut(|&b| b == 0x7e) {
            // RD8200 likes to bracket every frame with an FS (ie one at the start,
            // one at the end), as opposed to the more common fence-post arrangement
            // where a single FS appears between frames. Net result: we get some empty
            // bufs.
            if pkt.len() > 0 {
                if args.debug > 2 {
                    println!("Escaped packet  was: {:02x?}", pkt);
                }

                if let Ok(_) = unescape(pkt) {
                    if args.debug > 2 {
                        println!("Unescaped packet is: {:02x?}", pkt);
                    }

                    // slurp it in as a (mostly) unparsed sequence of bytes,
                    // over which we can compute the 2 different checksums.
                    match PPPBytes::from_bytes((pkt, 0)) {
                        Ok((_, mut frame)) => {
                            if fcs(&frame) && csum(&mut frame.data) {
                                // both checksums passed! re-parse the data fully.
                                match PPPFrame::from_bytes((pkt, 0)) {
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
                        }
                        Err(e) => {
                            eprintln!("Error reading raw frame: {}", e);
                        }
                    }
                } else {
                    eprintln!("HDLC Abort sequence detected");
                }
                pkt_idx += 1;
            }
        }
    }

    Ok(())
}
