use mos::Pinout;
use mos::core::{Context, ProgramCounter, InstructionRegister, StatusRegister};

pub struct CpuLogger {
    cycle_state: Vec<(Context, Pinout)>,
}