use mos::Pinout;
use mos::core::{Context, ProgramCounter, InstructionRegister, StatusRegister};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path; 

const CYCLES_PER_FRAME: usize = 38000;
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

    pub fn generate_log_file<P: AsRef<Path>>(file_path: P) {
       // pc opcode mnemonic address-bus data-bus cpu-registers cpu-clock-cycle 
    }
}