use ::nes_rom::ines;

use super::*;
use super::ppu;

pub enum PrgBankMode {
    Switch32K,
    FixFirst,
    FixLast,
}

pub enum ChrBankMode {
    Switch8K,
    Switch4K,
}

pub struct Mapper1 {
    pub context: Context,
    pub prg_bank_mode: PrgBankMode,
    pub chr_bank_mode: ChrBankMode,
    pub shift_register: u8,
    pub shift_count: u8,
    pub cpu_cycle: u64,
    pub last_write_cpu_cycle: u64,
    pub ram_enable: bool,
    pub uses_chr_ram: bool,
}

impl Mapper1 {
    pub fn new() -> Mapper1 {
        Mapper1 {
            context: Context::new(),
            prg_bank_mode: PrgBankMode::FixLast,
            chr_bank_mode: ChrBankMode::Switch8K,
            shift_register: 0,
            shift_count: 0,
            cpu_cycle: 0,
            last_write_cpu_cycle: 0,
            ram_enable: false,
            uses_chr_ram: false,
        }
    }

    pub fn from_ines(rom: &ines::Ines) -> Mapper1 {
        let mut mapper1 = Mapper1::new();

        mapper1.context.prg_rom = rom.prg_data.clone();
        mapper1.context.chr = rom.chr_data.clone();
        if mapper1.context.chr.len() == 0 {
            // set chr ram
            mapper1.context.chr = vec![0; SIZE_8K];
            mapper1.uses_chr_ram = true;
        }

        mapper1.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_16K);
        mapper1.context.prg_addr_mapper.set_banking_region_to_last_bank(0, SIZE_16K, mapper1.context.prg_rom.len());
        mapper1.context.wram_addr_mapper.set_banking_region(0, 0, SIZE_8K);
        mapper1.context.chr_addr_mapper.set_banking_region(0, 0, SIZE_8K);
        set_nametable_from_mirroring_type(&mut mapper1.context, rom.nametable_mirroring);

        mapper1
    }

    pub fn clear_shift(&mut self) {
        self.shift_register = 0;
        self.shift_count = 0;
    }

    pub fn ctrl_handler(&mut self, data: u8) {
        // mirroring
        match data & 0x03 {
            0 => set_nametable_single_screen_lower(&mut self.context),
            1 => set_nametable_single_screen_upper(&mut self.context),
            2 => set_nametable_vertical(&mut self.context),
            3 => set_nametable_horizontal(&mut self.context),
            _ => panic!("mapper 1 mirroring out of bounds")
        }

        // prg bank mode
        match (data & 0x0C) >> 2 {
            0 | 1 => { self.prg_bank_mode = PrgBankMode::Switch32K; }
            2 => { self.prg_bank_mode = PrgBankMode::FixFirst; }
            3 => { self.prg_bank_mode = PrgBankMode::FixLast; }
            _ => panic!("mapper 1 prg bank mode out of bounds")
        }

        match (data & 0x10) >> 4 {
            0 => { self.chr_bank_mode = ChrBankMode::Switch8K; }
            1 => { self.chr_bank_mode = ChrBankMode::Switch4K; }
            _ => panic!("mapper 1 chr bank mode out of bounds")
        }
    }

    pub fn chr_bank0_handler(&mut self, data: u8) {
        match self.chr_bank_mode {
            ChrBankMode::Switch8K => {
                self.context.chr_addr_mapper.set_banking_region(0, (data >> 1) as usize, SIZE_8K);
            }
            ChrBankMode::Switch4K => {
                self.context.chr_addr_mapper.set_banking_region(0, data as usize, SIZE_4K);
            }
        }
    }

    pub fn chr_bank1_handler(&mut self, data: u8) {
        match self.chr_bank_mode {
            ChrBankMode::Switch8K => {
                // ignored in 8kb mode
            }
            ChrBankMode::Switch4K => {
                self.context.chr_addr_mapper.set_banking_region(1, data as usize, SIZE_4K);
            }
        }
    }

    pub fn prg_bank_handler(&mut self, data: u8) {
        let bank = (data & 0x0F) as usize;
        let ram_enable = data & 0x10;

        match self.prg_bank_mode {
            PrgBankMode::Switch32K => {
                self.context.prg_addr_mapper.set_banking_region(0, bank >> 1, SIZE_32K);
            }
            PrgBankMode::FixFirst => {
                self.context.prg_addr_mapper.set_banking_region(0, 0, SIZE_16K);
                self.context.prg_addr_mapper.set_banking_region(1, bank, SIZE_16K);
            }
            PrgBankMode::FixLast => {
                self.context.prg_addr_mapper.set_banking_region(0, bank, SIZE_16K);
                self.context.prg_addr_mapper.set_banking_region_to_last_bank(1, SIZE_16K, self.context.prg_rom.len());

            }
        }

        if ram_enable == 0x10 { self.ram_enable = true; }
    }

    pub fn write_handler(&mut self, pinout: mos::Pinout) {
        if (pinout.data & 0x80) > 0 {
            self.clear_shift();
        }
        else {
            self.shift_register = (self.shift_register << 1) | (pinout.data & 0x01);
            self.shift_count += 1; 
        }

        // every fifth write
        if self.shift_count == 5 {
            let reg_index = ((pinout.address & 0x6000) >> 13) as u8 & 0x0F;

            match reg_index {
                0 => { self.ctrl_handler(self.shift_register); }
                1 => { self.chr_bank0_handler(self.shift_register); }
                2 => { self.chr_bank1_handler(self.shift_register); }
                3 => { self.prg_bank_handler(self.shift_register); }
                _ => panic!("mmc1 register out of bounds")
            }

            self.clear_shift();
        }
    }
}

impl Mapper for Mapper1 {
    // cpu 
    fn read_cpu_internal_ram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.context.sys_ram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn read_cpu_exp(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn read_cpu_wram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.ram_enable {
            let internal_address = self.context.wram_addr_mapper.translate_address(pinout.address);
            pinout.data = self.context.prg_ram[internal_address as usize];
        }
    
        pinout
    }

    fn read_cpu_prg(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let internal_address = self.context.prg_addr_mapper.translate_address(pinout.address);
        pinout.data = self.context.prg_rom[internal_address as usize];
        pinout
    }

    fn write_cpu_internal_ram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.context.sys_ram[(pinout.address & 0x7FF) as usize] = pinout.data;
        pinout
    }

    fn write_cpu_exp(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        // open bus
        pinout
    }

    fn write_cpu_wram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.ram_enable {
            let internal_address = self.context.wram_addr_mapper.translate_address(pinout.address);
            self.context.prg_ram[internal_address as usize] = pinout.data;
        }

        pinout
    }

    fn write_cpu_prg(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

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

    fn  write_ppu_chr(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        if self.uses_chr_ram {
            let internal_address = self.context.chr_addr_mapper.translate_address(pinout.address);
            self.context.chr[internal_address as usize] = pinout.data;
        }

        pinout
    }

    fn  write_ppu_nt(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        let internal_address = self.context.nt_addr_mapper.translate_address(pinout.address);
        self.context.vram[internal_address as usize] =pinout.data;
        pinout
    }

    fn cpu_tick(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.cpu_cycle += 1;
        if !pinout.ctrl.contains(mos::Ctrl::RW) {
            //check if this is a write cycle
            self.last_write_cpu_cycle = self.cpu_cycle;
        }

        pinout
    }

    fn ppu_tick(&mut self, mut pinout: ppu::Pinout) -> ppu::Pinout {
        pinout
    }
}





