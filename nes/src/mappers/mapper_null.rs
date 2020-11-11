use super::*;

pub struct MapperNull;

impl Mapper for MapperNull {
    fn rst_vector(&mut self, addr: u16)  { unimplemented!(); }
    fn read_internal_ram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_internal_ram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_expansion_rom(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_expansion_rom(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_wram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_wram(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn read_prg(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
    fn write_prg(&mut self, _pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }

    fn read_ppu(&mut self, ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout) { unimplemented!(); }
    fn write_ppu(&mut self, ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout) { unimplemented!(); }

    fn peek_ppu(&mut self, _addr: u16) -> u8 { unimplemented!(); }
    fn poke_ppu(&mut self, _addr: u16, data: u8) { unimplemented!(); }

}