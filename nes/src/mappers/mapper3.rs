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
        mapper3.context.chr_rom = rom.chr_data.clone();

        if mapper3.context.chr_rom.len() == 0 {
            // mapper3 only support chr rom
            panic!("mapper3 - chr  rom size is invalid ");
        }

        match rom.prg_rom_size as usize {
            SIZE_16K => {
                set_prg16k_8000_bfff(&mut mapper3.context.prg_bank_lookup, 0);
                set_prg16k_c000_ffff(&mut mapper3.context.prg_bank_lookup, 0);
             }
            SIZE_32K => {
                set_prg16k_8000_bfff(&mut mapper3.context.prg_bank_lookup, 0);
                set_prg16k_c000_ffff(&mut mapper3.context.prg_bank_lookup, 1);
             }
            _ => panic!("prg rom size is invalid - {:#X}", rom.prg_rom_size)
        };

        set_chr8k_0000_1fff(&mut mapper3.context.chr_bank_lookup, 0);
        set_wram8k_6000_7fff(&mut mapper3.context.wram_bank_lookup, 0);
        set_nametable_from_mirroring_type(&mut mapper3.context.nametable_bank_lookup, rom.nametable_mirroring);

        mapper3
    }

    pub fn write_handler(&mut self, pinout: mos::Pinout) {
        // CNROM only implements the lowest 2 bits, capping it at 32 KiB CHR. Other boards may implement 4 or more bits for larger CHR
        let bank_index = pinout.data as usize;
        set_chr8k_0000_1fff(&mut self.context.chr_bank_lookup, bank_index);
    }
}

impl Mapper for Mapper3 {
    // cpu 
    fn read_cpu_0000_1fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.context.sys_ram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn read_cpu_4020_5fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_6000_7fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        //no wram open bus    
        pinout
    }

    fn read_cpu_8000_8fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[0];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_9000_9fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[1];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }
    
    fn read_cpu_a000_afff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[2];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_b000_bfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[3];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_c000_cfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[4];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_d000_dfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[5];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_e000_efff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[6];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_f000_ffff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[7];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn write_cpu_0000_1fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.context.sys_ram[(pinout.address & 0x7FF) as usize] = pinout.data;
        pinout
    }

    fn write_cpu_4020_5fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn write_cpu_6000_7fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        //no wram open bus
        pinout
    }

    fn write_cpu_8000_8fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

    fn write_cpu_9000_9fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }
    
    fn write_cpu_a000_afff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

    fn write_cpu_b000_bfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

    fn write_cpu_c000_cfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

    fn write_cpu_d000_dfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

    fn write_cpu_e000_efff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

    fn write_cpu_f000_ffff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.write_handler(pinout);
        pinout
    }

     // ppu
     fn read_ppu_0000_03ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[0];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_0400_07ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[1];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_0800_0bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[2];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_0c00_0fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[3];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1000_13ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[4];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1400_17ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[5];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1800_1bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[6];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1c00_1fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[7];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_2000_23ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[0];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn  read_ppu_2400_27ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[1];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn  read_ppu_2800_2bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[2];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn  read_ppu_2c00_2fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[3];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn  write_ppu_0000_03ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_0400_07ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_0800_0bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_0c00_0fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_1000_13ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_1400_17ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_1800_1bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_1c00_1fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }

    fn  write_ppu_2000_23ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[0];
        self.context.vram[get_mem_address(bank, pinout.address)] =pinout.data;
        pinout
    }

    fn  write_ppu_2400_27ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[1];
        self.context.vram[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn  write_ppu_2800_2bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[2];
        self.context.vram[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn  write_ppu_2c00_2fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[3];
        self.context.vram[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn cpu_tick(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn ppu_tick(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }
}