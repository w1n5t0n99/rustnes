use mos::{Pinout, Ctrl};
use mos::core::*;
use std::io::Write;

// log large enough to cover NTSC and PAL
const LOG_SIZE: usize = 38000;

fn address_to_device(addr: u16, rw: bool) -> &'static str {
    match addr {
        0x0000..=0x1FFF =>      "RAM       ",
        0x2000..=0x3FFF => {
            match addr & 0x7 {
                0x00 => "PPU CTRL  ",
                0x01 => "PPU MASK  ",
                0x02 => "PPU STATUS",
                0x03 => "OAM ADDR  ",
                0x04 => "OAM DATA  ",
                0x05 => "PPU SCROLL",
                0x06 => "PPU ADDR  ",
                0x07 => "PPU DATA  ",
                _ => panic!("cpu trace logger ppu masking"),
            }
        },     
        0x4000..=0x4013 =>      "APU       ",
        0x4014 =>               "PPU-DMA   ",
        0x4015 =>               "APU       ",
        0x4016 =>               "CTRL1     ",
        0x4017 if rw == true => "CTRL2     ",
        0x4017 =>               "APU       ",
        0x4018..=0x401F =>      "DISABLED  ",
        0x4020..=0xFFFF =>      "CART      ",
    }
}

pub struct CpuTraceLogger {
    cpu_cache: Vec<(Context, Pinout)>,
    size: usize,
}

impl CpuTraceLogger {
    pub fn new() -> CpuTraceLogger {
        CpuTraceLogger {
            cpu_cache: vec![(Context::new(), Pinout::new()); LOG_SIZE],
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        // the log cache is pod data, no reason to waste time dropping all elements
        self.size = 0;
    }

    pub fn log(&mut self, context: Context, pinout: Pinout) {
        if self.size < LOG_SIZE {
            self.cpu_cache[self.size] = (context, pinout);
            self.size += 1;
        }
    }

    pub fn output_log<W: Write>(&self, w: &mut W) {
        
        for (i, (c, p)) in self.cpu_cache.iter().enumerate() {
            if self.size == 0 || i >= self.size {
                break;
            }

            let mut rw = true;
            let rw_str = match p.ctrl.contains(Ctrl::RW) {
                false =>{ rw = false; " <-W- "},
                true => " -R-> "
            };

            let sync_str = match p.ctrl.contains(Ctrl::SYNC) {
                true => "SYNC",
                false => "    "
            };

            let halt_str = match p.ctrl.contains(Ctrl::RDY) {
                false => "RDY",
                true => "   "
            };

            writeln!(w, "{} {} {:04X} {:02X} {} {:04X}{}{:02X} {}\t\tA:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC: {}",
                sync_str,
                halt_str,
                u16::from(c.pc),
                c.ir.opcode,
                opcode_to_mnemonic(c.ir.opcode, c.ir.tm),
                p.address,
                rw_str,
                p.data,
                address_to_device(p.address, rw),
                c.a,
                c.x,
                c.y,
                c.p.bits(),
                c.sp,
                c.cycle
            ).unwrap();
        }
    }
}