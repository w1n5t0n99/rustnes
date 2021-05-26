use crate::ppu::{Context, Pinout, Ctrl};
use std::io::Write;

// log large enough to cover NTSC and PAL
const LOG_SIZE: usize = 110000;

fn vpos_to_ntsc_scanline(vpos: u16) -> &'static str {
    match vpos {
        261 =>          "Prerender ",
        0..=239 =>      "Render    ",
        240 =>          "Postrender",
        241..=260 =>    "Vblank    ",
        _ => panic!("vpos out of bounds")
    }
}

fn address_to_device(addr: u16) -> &'static str {
    match addr {
        0x0000..=0x1fff => "CHR ",
        0x2000..=0x23bf => "NT 0",
        0x23c0..=0x23ff => "AT 0",
        0x2400..=0x27bf => "NT 1",
        0x27c0..=0x27ff => "AT 1",
        0x2800..=0x2bbf => "NT 2",
        0x2bc0..=0x2bff => "AT 2",
        0x2c00..=0x2fbf => "NT 3",
        0x2fc0..=0x2fff => "AT 3",
        _ => panic!("address out of bounds")
    }
}

pub struct PpuTraceLogger {
    ppu_cache: Vec<(Context, Pinout)>,
    size: usize,
}

impl PpuTraceLogger {
    pub fn new() -> PpuTraceLogger {
        PpuTraceLogger {
            ppu_cache: vec![(Context::new(), Pinout::new()); LOG_SIZE],
            size: 0,
        }
    }

    pub fn clear(&mut self) {
        // the log cache is pod data, no reason to waste time dropping all elements
        self.size = 0;
    }

    pub fn log(&mut self, context: Context, pinout: Pinout) {
        if self.size < LOG_SIZE {
            self.ppu_cache[self.size] = (context, pinout);
            self.size += 1;
        }
    }
    
    pub fn output_log<W: Write>(&self, w: &mut W) {
        
        for (i, (c, p)) in self.ppu_cache.iter().enumerate() {
            if self.size == 0 || i >= self.size {
                break;
            }

            let rd_str = match p.ctrl.contains(Ctrl::RD) {
                true =>"-",
                false => "R",
            };

            let wr_str = match p.ctrl.contains(Ctrl::WR) {
                true => "-",
                false => "W"
            };

            let ale_str = match p.ctrl.contains(Ctrl::ALE) {
                false => "-",
                true => "L"
            };

            write!(w, "CYC: {} {}:{} {:04X} {}{} {:02X} {}",
                c.cycle,
                vpos_to_ntsc_scanline(c.vpos),
                c.vpos,
                p.address,
                rd_str,
                wr_str,
                p.data,
                address_to_device(p.address),
            ).unwrap();

            match i%3 {
                0 => { write!(w, " | ").unwrap(); },
                1 => { write!(w, " | ").unwrap(); },
                2 => { write!(w, "\n").unwrap(); },
                _ => panic!("index out of bounds"),
            }
        }
    }
}


