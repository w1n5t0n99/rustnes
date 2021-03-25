use mos::{Pinout, Ctrl};
use mos::core::*;
use std::io::Write;


const LOG_SIZE: usize = 38000;

fn address_to_device(addr: u16) -> &'static str {
    match addr {
        0x0000..=0x1FFF => "RAM      ",
        0x2000..=0x3FFF => "PPU      ",
        0x4000..=0x4013 => "APU      ",
        0x4014 => "PPU-DMA  ",
        0x4015 => "APU      ",
        0x4016 => "CTRL1   ",
        0x4017 => "APU-CTRL2",
        0x4018..=0x401F => "DISABLED ",
        0x4020..=0xFFFF => "CART     ",
    }
}

pub struct TraceLogger {
    cpu_cache: Vec<(Context, Pinout)>,
}

impl TraceLogger {
    pub fn new() -> TraceLogger {
        TraceLogger {
            cpu_cache: Vec::with_capacity(LOG_SIZE),
        }
    }

    pub fn clear(&mut self) {
        self.cpu_cache.clear();
    }

    pub fn log(&mut self, context: Context, pinout: Pinout) {
        self.cpu_cache.push((context, pinout));
    }

    pub fn output_log<W: Write>(&self, w: &mut W) {
        
        for (c, p) in self.cpu_cache.iter() {
            let rw_str = match p.ctrl.contains(Ctrl::RW) {
                true => " <-W- ",
                false => " -R-> "
            };

            let sync_str = match p.ctrl.contains(Ctrl::SYNC) {
                true => "SYNC",
                false => "    "
            };

            writeln!(w, "{} {:04X} {:X} {} {:04X}{}{:02X} {} CYC: {}",
                sync_str,
                u16::from(c.pc),
                c.ir.opcode,
                opcode_to_mnemonic(c.ir.opcode, c.ir.tm),
                p.address,
                rw_str,
                p.data,
                address_to_device(p.address),
                c.cycle
            ).unwrap();
        }

    }
}