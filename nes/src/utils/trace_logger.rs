use mos::{Pinout, Ctrl};
use mos::core::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path; 
use std::io::Write;

const CYCLES_PER_FRAME: usize = 38000;

fn address_to_device(addr: u16) -> &'static str {
    match addr {
        0x0000..=0x1FFF => "RAM",
        0x2000..=0x3FFF => "PPU",
        0x4000..=0x4013 => "APU",
        0x4014 => "PPU-DMA",
        0x4015 => "APU",
        0x4016 => "CTRL1",
        0x4017 => "APU-CTRL2",
        0x4018..=0x401F => "DISABLED",
        0x4020..=0xFFFF => "CART",
    }
}

pub struct CpuLogger {
    cycle_state: Vec<(Context, Pinout)>,
}

impl CpuLogger {
    pub fn new() -> CpuLogger {
        CpuLogger {
            cycle_state: Vec::with_capacity(CYCLES_PER_FRAME),
        }
    }

    pub fn clear(&mut self) {
        self.cycle_state.clear();
    }

    pub fn log(&mut self, context: Context, pinout: Pinout) {
        self.cycle_state.push((context, pinout));
    }

    pub fn generate_log_file<P: AsRef<Path>>(&self, file_path: P) {
        // pc opcode mnemonic address-bus data-bus cpu-registers cpu-clock-cycle 
        let f = File::open(file_path).unwrap();
        let mut f = BufWriter::new(f);
        
        for (c, p) in self.cycle_state.iter() {
            let rw_str = match p.ctrl.contains(Ctrl::RW) {
                true => "<W-",
                false => "-R>"
            };

            let sync_str = match p.ctrl.contains(Ctrl::SYNC) {
                true => "SYNC",
                false => "    "
            };

            writeln!(f, "{:X} {:X} {} {} {:X}{}{:X} CYC: {}",
                u16::from(c.pc),
                c.ir.opcode,
                opcode_to_mnemonic(c.ir.opcode, c.ir.tm),
                sync_str,
                p.address,
                rw_str,
                p.data,
                c.cycle
            ).unwrap();
        }

    }
}