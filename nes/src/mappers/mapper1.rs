use ::nes_rom::ines;
use std::ptr;

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
}

impl Mapper1 {
    pub fn new() -> Mapper1 {
        Mapper1 {
            context: Context::new(),
            prg_bank_mode: PrgBankMode::Switch32K,
            chr_bank_mode: ChrBankMode::Switch8K,
            shift_register: 0,
            shift_count: 0,
            cpu_cycle: 0,
            last_write_cpu_cycle: 0,
            ram_enable: false,
        }
    }

    pub fn from_ines(rom: &ines::Ines) -> Mapper1 {
        let mut mapper1 = Mapper1::new();

        mapper1.context.prg_rom = rom.prg_data.clone();
        mapper1.context.chr_rom = rom.chr_data.clone();
        // lets 8k wram
        mapper1.context.work_ram = Some(vec![0; SIZE_8K]);

        set_prg32k_8000_ffff(&mut mapper1.context.prg_bank_lookup, get_last_bank_index(SIZE_32K, mapper1.context.prg_rom.len()));
        set_chr8k_0000_1fff(&mut mapper1.context.chr_bank_lookup, 0);
        set_wram8k_6000_7fff(&mut mapper1.context.wram_bank_lookup, 0);
        set_nametable_from_mirroring_type(&mut mapper1.context.nametable_bank_lookup, rom.nametable_mirroring);

        mapper1
    }

    pub fn clear_shift(&mut self) {
        self.shift_register = 0;
        self.shift_count = 0;
    }

    pub fn handle_ctrl(&mut self, data: u8) {
        // mirroring
        match data & 0x03 {
            0 => set_nametable_single_screen_lower(&mut self.context.nametable_bank_lookup),
            1 => set_nametable_single_screen_upper(&mut self.context.nametable_bank_lookup),
            2 => set_nametable_vertical(&mut self.context.nametable_bank_lookup),
            3 => set_nametable_horizontal(&mut self.context.nametable_bank_lookup),
            _ => panic!("mapper 1 mirroring out of bounds")
        }

        // prg bank mode
        match (data & 0xC0) >> 2 {
            0 | 1 => { self.prg_bank_mode = PrgBankMode::Switch32K; }
            2 => { self.prg_bank_mode = PrgBankMode::FixFirst; }
            3 => { self.prg_bank_mode = PrgBankMode::FixLast; }
            _ => panic!("mapper 1 prg bank mode out of bounds")
        }

        match (data & 0x10) {
            0 => { self.chr_bank_mode = ChrBankMode::Switch8K; }
            0x10 => { self.chr_bank_mode = ChrBankMode::Switch4K; }
            _ => panic!("mapper 1 chr bank mode out of bounds")
        }
    }

    pub fn handle_chr_bank0(&mut self, data: u8) {
        match self.chr_bank_mode {
            ChrBankMode::Switch8K => {
                set_chr8k_0000_1fff(&mut self.context.chr_bank_lookup, (data >> 1) as usize);
            }
            ChrBankMode::Switch4K => {
                set_chr4k_0000_0fff(&mut self.context.chr_bank_lookup, data as usize);
            }
        }
    }

    pub fn handle_chr_bank1(&mut self, data: u8) {
        match self.chr_bank_mode {
            ChrBankMode::Switch8K => {
                // ignored in 8kb mode
            }
            ChrBankMode::Switch4K => {
                set_chr4k_1000_1fff(&mut self.context.chr_bank_lookup, data as usize);
            }
        }
    }

    pub fn handle_prg_bank(&mut self, data: u8) {
        let bank = (data & 0x0F) as usize;
        let ram_enable = data & 0x10;

        match self.prg_bank_mode {
            PrgBankMode::Switch32K => {
                set_prg32k_8000_ffff(&mut self.context.prg_bank_lookup, bank >> 1);
            }
            PrgBankMode::FixFirst => {
                set_prg16k_8000_bfff(&mut self.context.prg_bank_lookup, 0);
                set_prg16k_c000_ffff(&mut self.context.prg_bank_lookup, bank);
            }
            PrgBankMode::FixLast => {
                set_prg16k_8000_bfff(&mut self.context.prg_bank_lookup, bank);
                set_prg16k_c000_ffff(&mut self.context.prg_bank_lookup, get_last_bank_index(SIZE_32K, self.context.prg_rom.len()));
            }
        }

        if ram_enable == 0x10 { self.ram_enable = true; }
    }

    pub fn write_handler(&mut self, pinout: mos::Pinout) {
        if (pinout.data & 0x80) > 0 {
            self.clear_shift();
        }

        match self.shift_count {
            0..=3 => {
                self.shift_register = (self.shift_register << 1) | (pinout.data & 0x01);
                self.shift_count += 1; 
            }
            4 => {
                self.shift_register = (self.shift_register << 1) | (pinout.data & 0x01);
                // index is bit 14 and 13 of address
                let index = ((pinout.address & 0x6000) >> 13) as u8;
                match index {
                    0 => { self.handle_ctrl(self.shift_register); }
                    1 => { self.handle_chr_bank0(self.shift_register); }
                    2 => { self.handle_chr_bank1(self.shift_register); }
                    3 => { self.handle_prg_bank(self.shift_register); }
                    _ => panic!("mmc1 register out of bounds")
                }

                self.clear_shift();
            }
            _ => panic!("mapper1 shift register out of bounds")
        }
    }
}

impl Mapper for Mapper1 {

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
        // open bus
        match self.context.work_ram {
            Some(ref wram) => {
                let bank = &self.context.wram_bank_lookup[0];
                pinout.data = wram[get_mem_address(bank, pinout.address)];
            }
            None => {

            }
        }
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
        match self.context.work_ram {
            Some(ref mut wram) => {
                let bank = &self.context.wram_bank_lookup[0];
                wram[get_mem_address(bank, pinout.address)] = pinout.data;
            }
            None => {

            }
        }
        pinout
    }

    fn write_cpu_8000_8fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }

    fn write_cpu_9000_9fff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }
    
    fn write_cpu_a000_afff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }

    fn write_cpu_b000_bfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }

    fn write_cpu_c000_cfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }

    fn write_cpu_d000_dfff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }

    fn write_cpu_e000_efff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }

        pinout
    }

    fn write_cpu_f000_ffff(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        if self.cpu_cycle > self.last_write_cpu_cycle {
            self.write_handler(pinout);
        }
        
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





