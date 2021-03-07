use ::nes_rom::ines;
use std::ptr;

use super::*;
use super::ppu;

pub struct MapperNrom {
    pub context: Context,
}

impl MapperNrom {
    pub fn new() -> MapperNrom {
        MapperNrom {
            context: Context::new(),
        }
    }

    pub fn from_ines(rom: &ines::Ines) -> MapperNrom {
        let mut mapper_nrom = MapperNrom::new();

        mapper_nrom.context.prg_rom = rom.prg_data.clone();
        mapper_nrom.context.chr_rom = rom.chr_data.clone();

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

        match rom.chr_rom_size as usize {
            SIZE_8K => {
                set_chr8k_0000_1fff(&mut mapper_nrom.context.chr_bank_lookup, 0);
            }
            _ => panic!("chr rom size is invalid - {:#X}", rom.chr_rom_size)
        }

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
    fn read_cpu_0000_1fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.context.sys_ram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn read_cpu_4020_5fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_6000_7fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        // TODO: maybe implement 8k wram
        pinout
    }

    fn read_cpu_8000_8fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[0];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_9000_9fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[1];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }
    
    fn read_cpu_a000_afff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[2];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_b000_bfff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[3];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_c000_cfff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[4];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_d000_dfff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[5];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_e000_efff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[6];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_cpu_f000_ffff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[7];
        pinout.data = self.context.prg_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn write_cpu_8000_8fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[0];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn write_cpu_9000_9fff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[1];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] =pinout.data;
        pinout
    }
    
    fn write_cpu_a000_afff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[2];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn write_cpu_b000_bfff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[3];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn write_cpu_c000_cfff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[4];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] =pinout.data;
        pinout
    }

    fn write_cpu_d000_dfff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[5];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn write_cpu_e000_efff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[6];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    fn write_cpu_f000_ffff(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let bank = &self.context.prg_bank_lookup[7];
        self.context.prg_rom[get_mem_address(bank, pinout.address)] = pinout.data;
        pinout
    }

    // ppu
    fn read_ppu_0000_03ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[0];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_0400_07ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[1];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_0800_0bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[2];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_0c00_0fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[3];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1000_13ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[4];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1400_17ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[5];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1800_1bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[6];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_1c00_1fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.chr_bank_lookup[7];
        pinout.data = self.context.chr_rom[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_2000_23ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[0];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_2400_27ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[1];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_2800_2bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[2];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn read_ppu_2c00_2fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout {
        let bank = &self.context.nametable_bank_lookup[3];
        pinout.data = self.context.vram[get_mem_address(bank, pinout.address)];
        pinout
    }

    fn write_ppu(&mut self, ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout) {
        let addr = ppu_pinout.address;

        match addr {
            // CHR ROM
            0x0000..=0x1FFF => { /* returns whatevers on the bus already */ },
            // NT A
            0x2000..=0x23FF => { self.vram[(addr - self.nt_offset.nt_a) as usize] = ppu_pinout.data; },
            // NT B
            0x2400..=0x27FF => { self.vram[(addr - self.nt_offset.nt_b) as usize] = ppu_pinout.data; },
            // NT C
            0x2800..=0x2BFF => { self.vram[(addr - self.nt_offset.nt_c) as usize] = ppu_pinout.data; },
            // NT D
            0x2C00..=0x2FFF => { self.vram[(addr - self.nt_offset.nt_d) as usize] = ppu_pinout.data; },
            _ => panic!("NROM PPU write out of bounds: {}", addr),
        }

        (ppu_pinout, cpu_pinout)
    }

    fn peek_ppu(&mut self, addr: u16) -> u8 {
        match addr {
            // CHR
            0x0000..=0x1FFF => { self.chr_rom[addr as usize] }
            // A
            0x2000..=0x23FF => { self.vram[(addr - self.nt_offset.nt_a) as usize] },
            // B
            0x2400..=0x27FF => { self.vram[(addr - self.nt_offset.nt_b) as usize] },
            // C
            0x2800..=0x2BFF => { self.vram[(addr - self.nt_offset.nt_c) as usize] },
            // D
            0x2C00..=0x2FFF => { self.vram[(addr - self.nt_offset.nt_d) as usize] },
            _ => panic!("NROM PPU read out of bounds: {}", addr),
        }
    }

    fn poke_ppu(&mut self, addr: u16, data: u8) {
        match addr {
            // CHR ROM
            0x0000..=0x1FFF => { /* returns whatevers on the bus already */ },
            // NT A
            0x2000..=0x23FF => { self.vram[(addr - self.nt_offset.nt_a) as usize] = data; },
            // NT B
            0x2400..=0x27FF => { self.vram[(addr - self.nt_offset.nt_b) as usize] = data; },
            // NT C
            0x2800..=0x2BFF => { self.vram[(addr - self.nt_offset.nt_c) as usize] = data; },
            // NT D
            0x2C00..=0x2FFF => { self.vram[(addr - self.nt_offset.nt_d) as usize] = data; },
            _ => panic!("NROM PPU write out of bounds: {}", addr),
        }
    }
}





