use ::nes_rom::ines;

use super::*;
use super::ppu;

pub struct MapperNrom {
    pub context: Context,
    pub uses_chr_ram: bool,
}

impl MapperNrom {
    pub fn new() -> MapperNrom {
        MapperNrom {
            context: Context::new(),
            uses_chr_ram: false,
        }
    }

    pub fn from_ines(rom: &ines::Ines) -> MapperNrom {
        let mut mapper_nrom = MapperNrom::new();

        mapper_nrom.context.prg_rom = rom.prg_data.clone();
        mapper_nrom.context.chr = rom.chr_data.clone();

        if mapper_nrom.context.chr.len() == 0 {
            // set chr ram
            mapper_nrom.context.chr = vec![0; SIZE_8K];
            mapper_nrom.uses_chr_ram = true;
        }

        match rom.prg_rom_size as usize {
            SIZE_16K => {
                mapper_nrom.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_16K);
                mapper_nrom.context.prg_addr_mapper.set_banking_region(1, 0, SIZE_16K);
             }
            SIZE_32K => {
                mapper_nrom.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_32K);
             }
            _ => panic!("prg rom size is invalid - {:#X}", rom.prg_rom_size)
        };

        mapper_nrom.context.chr_addr_mapper.set_banking_region(0, 0, SIZE_8K);
        mapper_nrom.context.wram_addr_mapper.set_banking_region(0, 0, SIZE_8K);

        set_nametable_from_mirroring_type(&mut mapper_nrom.context, rom.nametable_mirroring);

        mapper_nrom
    }
}

impl Mapper for MapperNrom {

    fn read_cpu_internal_ram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.context.sys_ram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn read_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_wram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let internal_address = self.context.wram_addr_mapper.translate_address(pinout.address);
        pinout.data = self.context.prg_ram[internal_address as usize];
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
        let internal_address = self.context.wram_addr_mapper.translate_address(pinout.address);
        self.context.prg_ram[internal_address as usize] = pinout.data;
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
        if self.uses_chr_ram {
            let internal_address = self.context.chr_addr_mapper.translate_address(pinout.address);
            self.context.chr[internal_address as usize] = pinout.data;
        }

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





