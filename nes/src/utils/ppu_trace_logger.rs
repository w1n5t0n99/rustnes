use crate::ppu::{Context, Pinout};
use std::io::Write;

// log large enough to cover NTSC and PAL
const LOG_SIZE: usize = 38000;

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

    
}


