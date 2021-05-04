use ::nes_rom::ines;

use super::*;
use super::ppu;

pub struct Mapper3 {
    pub context: Context,
}

impl Mapper3 {
    pub fn new() -> Mapper3 {
        Mapper3 {
            context: Context::new(),
        }
    }

    pub fn from_ines(rom: &ines::Ines) -> Mapper3 {
        let mut mapper3 = Mapper3::new();

        mapper3.context.prg_rom = rom.prg_data.clone();
        mapper3.context.chr = rom.chr_data.clone();

        if mapper3.context.chr.len() == 0 {
            // mapper3 only support chr rom
            panic!("mapper3 - chr  rom size is invalid ");
        }

        match rom.prg_rom_size as usize {
            SIZE_16K => {
                mapper3.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_16K);
                mapper3.context.prg_addr_mapper.set_banking_region(1, 0, SIZE_16K);
             }
            SIZE_32K => {
                mapper3.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_32K);
             }
            _ => panic!("prg rom size is invalid - {:#X}", rom.prg_rom_size)
        };

        mapper3.context.wram_addr_mapper.set_banking_region(0, 0, SIZE_8K);
        mapper3.context.chr_addr_mapper.set_banking_region(0, 0, SIZE_8K);
        set_nametable_from_mirroring_type(&mut mapper3.context, rom.nametable_mirroring);

        mapper3
    }

    pub fn write_handler(&mut self, pinout: mos::Pinout) {
        // CNROM only implements the lowest 2 bits, capping it at 32 KiB CHR. Other boards may implement 4 or more bits for larger CHR
        let bank_index = pinout.data as usize;
        self.context.chr_addr_mapper.set_banking_region(0, bank_index, SIZE_8K);
    }
}

impl Mapper for Mapper3 {
    // cpu 
    fn read_cpu_internal_ram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.context.sys_ram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn read_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        //no wram open bus    
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
        //no wram open bus
        pinout
    }

    fn write_cpu_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

     // ppu
     fn read_ppu_chr(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.chr_addr_mapper.translate_address(pinout.address);
        pinout.data = self.context.chr[internal_address as usize];
        pinout
    }

    fn read_ppu_nt(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.nt_addr_mapper.translate_address(pinout.address & 0x2fff);
        pinout.data = self.context.vram[internal_address as usize];
        pinout
    }

    fn  write_ppu_chr(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_nt(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.nt_addr_mapper.translate_address(pinout.address & 0x2fff);
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