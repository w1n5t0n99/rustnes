use ::nes_rom::ines;
use std::ptr;

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
        mapper_nrom.context.chr_rom = rom.chr_data.clone();

        if mapper_nrom.context.chr_rom.len() == 0 {
            // set chr ram
            mapper_nrom.context.chr_rom = vec![0; SIZE_8K];
            mapper_nrom.uses_chr_ram = true;
        }

        match rom.prg_rom_size as usize {
            SIZE_16K => {
                set_prg16k_8000_bfff(&mut mapper_nrom.context.prg_bank_lookup, 0);
                set_prg16k_c000_ffff(&mut mapper_nrom.context.prg_bank_lookup, 0);
             }
            SIZE_32K => {
                set_prg16k_8000_bfff(&mut mapper_nrom.context.prg_bank_lookup, 0);
                set_prg16k_c000_ffff(&mut mapper_nrom.context.prg_bank_lookup, 1);
             }
            _ => panic!("prg rom size is invalid - {:#X}", rom.prg_rom_size)
        };

        set_chr8k_0000_1fff(&mut mapper_nrom.context.chr_bank_lookup, 0);
        set_wram8k_6000_7fff(&mut mapper_nrom.context.wram_bank_lookup, 0);
        set_nametable_from_mirroring_type(&mut mapper_nrom.context.nametable_bank_lookup, rom.nametable_mirroring);

        mapper_nrom
    }
}

impl Mapper for MapperNrom {

    fn change_rst_vector(&mut self, addr: u16) {
        let hb = (addr >> 8) as u8;
        let lb = addr as u8;

        let bank = &self.context.prg_bank_lookup[7];

        let mut rst_vec = get_mem_address(bank, 0xFFFD);
        self.context.prg_rom[rst_vec as usize] = hb;

        rst_vec = get_mem_address(bank, 0xFFFC);
        self.context.prg_rom[rst_vec as usize] = lb;
    }

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
        let bank = &self.context.wram_bank_lookup[0];
        pinout.data = self.context.work_ram[get_mem_address(bank, pinout.address)];
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
        let bank = &self.context.wram_bank_lookup[0];
        self.context.work_ram[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn write_cpu_8000_8fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn write_cpu_9000_9fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }
    
    fn write_cpu_a000_afff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn write_cpu_b000_bfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn write_cpu_c000_cfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn write_cpu_d000_dfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn write_cpu_e000_efff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout
    }

    fn write_cpu_f000_ffff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
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
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[0];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_0400_07ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[1];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_0800_0bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[2];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_0c00_0fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[3];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_1000_13ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[4];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_1400_17ff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[5];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_1800_1bff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[6];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_1c00_1fff(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let bank = &self.context.chr_bank_lookup[7];
            self.context.chr_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        }

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





