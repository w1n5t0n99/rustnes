use super::error::NesError;
use super::mapper_nrom::MapperNrom;
use super::ppu_pinout;
use ::nes_rom::ines;

/*
The contents of the palette are unspecified at power on and unchanged at reset. 
During the warmup state, the PPU outputs a solid color screen based on the value at $3F00.ppu_viewer
This just gives and initial value for testing.
*/
pub static POWER_ON_PALETTE: [u8; 32] = [0x09, 0x01, 0x00, 0x01, 0x00, 0x02, 0x02, 0x0D, 0x08, 0x10, 0x08, 0x24, 0x00, 0x00, 0x04, 0x2C,
0x09, 0x01, 0x34, 0x03, 0x00, 0x04, 0x00, 0x14, 0x08, 0x3A, 0x00, 0x02, 0x00, 0x20, 0x2C, 0x08];

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
    fn read_pattern_table(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout;
    fn write_pattern_table(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout;
    fn read_nametable(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout;
    fn write_nametable(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout;
    fn read_palette(&mut self, pinout: ppu_pinout::Pinout, forced_vblank: bool) -> ppu_pinout::Pinout;
    fn write_palette(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout;
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

    fn read_pattern_table(&mut self, _pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { unimplemented!(); }
    fn write_pattern_table(&mut self, _pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { unimplemented!(); }
    fn read_nametable(&mut self, _pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { unimplemented!(); }
    fn write_nametable(&mut self, _pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { unimplemented!(); }
    fn read_palette(&mut self, _pinout: ppu_pinout::Pinout, _forced_vblank: bool) -> ppu_pinout::Pinout { unimplemented!(); }
    fn write_palette(&mut self, _pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { unimplemented!(); }

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