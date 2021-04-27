use ::nes_rom::ines;

use super::*;
use super::ppu;

pub struct MapperDebug {
    pub context: Context,
}

impl MapperDebug {
    pub fn new() -> MapperDebug {
        let mut mapper = MapperDebug {
            context: Context::new(),
        };

        mapper.context.prg_rom = vec![0; SIZE_16K];
        mapper.context.chr = vec![0; SIZE_8K];

        mapper.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_16K);
        mapper.context.prg_addr_mapper.set_banking_region(1, 0, SIZE_16K);
        set_nametable_from_mirroring_type(&mut mapper.context, ines::NametableMirroring::Horizontal);

        mapper
    }

    pub fn poke_internal_ram(&mut self, index: usize, data: u8) {
        self.context.sys_ram[index] = data;
    }

    pub fn peek_internal_ram(&mut self, index: usize) -> u8{
        self.context.sys_ram[index]
    }

    pub fn poke_prg(&mut self, index: usize, data: u8) {
        self.context.prg_rom[index] = data;
    }

    pub fn peek_prg(&mut self, index: usize) -> u8{
        self.context.prg_rom[index]
    }

    pub fn poke_chr(&mut self, index: usize, data: u8) {
        self.context.chr[index] = data;
    }

    pub fn peek_chr(&mut self, index: usize) -> u8{
        self.context.chr[index]
    }

    pub fn poke_nt(&mut self, index: usize, data: u8) {
        self.context.vram[index] = data;
    }

    pub fn peek_nt(&mut self, index: usize) -> u8{
        self.context.vram[index]
    }
}

impl Mapper for MapperDebug {

    fn read_cpu_internal_ram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.context.sys_ram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn read_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_wram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_prg(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let internal_address = self.context.prg_addr_mapper.translate_address(pinout.address);
        pinout.data = self.context.prg_rom[internal_address as usize];
        pinout
    }

    fn write_cpu_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.sys_ram[(pinout.address & 0x7FF) as usize] = pinout.data;
        pinout   
    }

    fn write_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn write_cpu_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout {

        pinout
    }

    fn write_cpu_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    // ppu
    fn read_ppu_chr(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.chr_addr_mapper.translate_address(pinout.address);
        pinout.data = self.context.chr[internal_address as usize];
        pinout
    }

    fn read_ppu_nt(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.nt_addr_mapper.translate_address(pinout.address);
        pinout.data = self.context.vram[internal_address as usize];
        pinout
    }

    fn  write_ppu_chr(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.chr_addr_mapper.translate_address(pinout.address);
        self.context.chr[internal_address as usize] = pinout.data;

        pinout
    }

    fn  write_ppu_nt(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.nt_addr_mapper.translate_address(pinout.address);
        self.context.vram[internal_address as usize] = pinout.data;
        pinout
    }

    fn cpu_tick(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn ppu_tick(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }
}

