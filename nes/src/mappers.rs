use super::error::NesError;
use super::mapper_nrom::MapperNrom;
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
    fn read_pattern_table(&mut self, vaddr: u16, pinout: mos::Pinout) -> (u8, mos::Pinout);
    fn write_pattern_table(&mut self, vaddr: u16, data: u8, pinout: mos::Pinout) -> mos::Pinout;
    fn read_nametable(&mut self, vaddr: u16, pinout: mos::Pinout) -> (u8, mos::Pinout);
    fn write_nametable(&mut self, vaddr: u16, data: u8, pinout: mos::Pinout) -> mos::Pinout;
    fn read_palette(&mut self, vaddr: u16, forced_vblank: bool) -> u8;
    fn write_palette(&mut self, vaddr: u16, data: u8);
    // no side effects from reading or writing (e.g. mappers with memory mapped regs)
    fn peek_pattern_table(&mut self, addr: u16) -> u8;
    fn peek_nametable(&mut self, addr: u16) -> u8;
    fn peek_palette(&mut self, addr: u16) -> u8;
    fn poke_prg(&mut self, addr: u16, data: u8);
}

pub struct MapperNull;

impl Mapper for MapperNull {
    fn read_internal_ram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_internal_ram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_expansion_rom(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_expansion_rom(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_wram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_wram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_prg(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_prg(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }

    fn read_pattern_table(&mut self, vaddr: u16, pinout: mos::Pinout) -> (u8, mos::Pinout) { unimplemented!(); }
    fn write_pattern_table(&mut self, vaddr: u16, data: u8, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_nametable(&mut self, vaddr: u16, pinout: mos::Pinout) -> (u8, mos::Pinout) { unimplemented!(); }
    fn write_nametable(&mut self, vaddr: u16, data: u8, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_palette(&mut self, vaddr: u16, forced_vblank: bool) -> u8 { unimplemented!(); }
    fn write_palette(&mut self, vaddr: u16, data: u8) { unimplemented!(); }

    fn peek_pattern_table(&mut self, _addr: u16) -> u8 { unimplemented!(); }
    fn peek_nametable(&mut self, _addr: u16) -> u8 { unimplemented!(); }
    fn peek_palette(&mut self, _addr: u16) -> u8 { unimplemented!(); }
    fn poke_prg(&mut self, _addr: u16, _data: u8) { unimplemented!(); }

}

pub fn create_mapper(rom: &ines::Ines) -> Result<Box<dyn Mapper>, NesError> {
    match rom.mapper {
        0 => {
            let boxed: Box<dyn Mapper> = Box::new(MapperNrom::from_ines(rom));
            Ok(boxed)
        }
        _ => Err(NesError::from_mapper(format!("mapper {} implementation not found", rom.mapper)))
    }
}