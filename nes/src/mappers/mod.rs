pub mod mapper_debug;
mod mapper_null;
mod mapper_nrom;

use super::ppu;
use mapper_debug::MapperDebug;
use mapper_nrom::MapperNrom;
use mapper_null::MapperNull;
use ::nes_rom::ines;

/*
The contents of the palette are unspecified at power on and unchanged at reset. 
During the warmup state, the PPU outputs a solid color screen based on the value at $3F00.ppu_viewer
This just gives and initial value for testing.
*/
pub static POWER_ON_PALETTE: [u8; 32] = [0x09, 0x01, 0x00, 0x01, 0x00, 0x02, 0x02, 0x0D, 0x08, 0x10, 0x08, 0x24, 0x00, 0x00, 0x04, 0x2C,
0x09, 0x01, 0x34, 0x03, 0x00, 0x04, 0x00, 0x14, 0x08, 0x3A, 0x00, 0x02, 0x00, 0x20, 0x2C, 0x08];

// Nametable A, B, C, D are masked indiviually depending on rom mirroring type, NES only had 2 but use 4 here for simplicity
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct NametableOffset {
    pub nt_a: u16,
    pub nt_b: u16,
    pub nt_c: u16,
    pub nt_d: u16,
}

impl NametableOffset {
    pub fn new(a: u16, b: u16, c: u16, d: u16) -> NametableOffset {
        NametableOffset {
            nt_a: a,
            nt_b: b,
            nt_c: c,
            nt_d: d,
        }
    }

    pub fn from_nametable(nt_type: NametableType) -> NametableOffset {
        match nt_type {
            NametableType::Horizontal => return NametableOffset::new(0x2000, 0x2400, 0x2000, 0x2400),
            NametableType::Vertical => return NametableOffset::new(0x2000, 0x2000, 0x2800, 0x2800),
            NametableType::SingleScreen => return NametableOffset::new(0x2000, 0x2400, 0x2800, 0x2C00),
            NametableType::FourScreen => return NametableOffset::new(0x2000, 0x2000, 0x2000, 0x2000),
            NametableType::Diagonal => return NametableOffset::new(0x2000, 0x2000, 0x2400, 0x2C00),
            NametableType::LShaped => return NametableOffset::new(0x2000, 0x2000, 0x2400, 0x2800),
            NametableType::ThreeScreenVertical => return NametableOffset::new(0x2000, 0x2000, 0x2000, 0x2800),
            NametableType::ThreeScreenHorizontal => return NametableOffset::new(0x2000, 0x2000, 0x2000, 0x2400),
            NametableType::ThreeScreenDiagonal => return NametableOffset::new(0x2000, 0x2000, 0x2400, 0x2000),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NametableType {
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen,
    Diagonal,
    LShaped,
    ThreeScreenVertical,
    ThreeScreenHorizontal,
    ThreeScreenDiagonal,
}

pub trait Mapper {
    fn rst_vector(&mut self, addr: u16);
    // CPU
    fn read_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_expansion_rom(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_expansion_rom(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    // PPU    
    fn read_ppu(&mut self, ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout);
    fn write_ppu(&mut self, ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout);
    // no side effects from reading or writing (e.g. mappers with memory mapped regs)
    fn peek_ppu(&mut self, addr: u16) -> u8;
}

pub fn create_mapper(rom: &ines::Ines) -> Box<dyn Mapper> {
    match rom.mapper {
        0 => {
            Box::new(MapperNrom::from_ines(rom))
        }
        // TODO: add error handling instead of panicking like a monster
        _ => { panic!("mapper {} implementation not found", rom.mapper); }
    }
}

pub fn create_mapper_null() -> Box<dyn Mapper> {
    Box::new(MapperNull {})
}