mod mapper_null;
mod mapper_nrom;

use super::ppu;
use mapper_nrom::MapperNrom;
use mapper_null::MapperNull;
use ::nes_rom::ines;

const SIZE_1K: usize = 1024;
const SIZE_2K: usize = 2048;
const SIZE_4K: usize = 4096;
const SIZE_8K: usize = 8192;
const SIZE_16K: usize = 16384;
const SIZE_32K: usize = 32768;
const SIZE_64K: usize = 65536;
const SIZE_128K: usize = 131072;
const SIZE_256K: usize = 262144;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Bank {
    pub offset_mask: usize,
    pub window: usize,
}

impl Bank {
    pub fn new(offset_mask: usize, window: usize) -> Bank {
        Bank {
            offset_mask,
            window,
        }
    }
}

// nametable mirroring
pub fn set_vram_2000_23ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_vram_2400_27ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[1] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_vram_2800_2bff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[2] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_vram_2c00_2fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[3] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_nametable_horizontal(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 0);
    set_vram_2800_2bff(bank_lookup, 1);
    set_vram_2c00_2fff(bank_lookup, 1);
}

pub fn set_nametable_vertical(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 1);
    set_vram_2800_2bff(bank_lookup, 0);
    set_vram_2c00_2fff(bank_lookup, 1);
}

pub fn set_nametable_single_screen(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 0);
    set_vram_2800_2bff(bank_lookup, 0);
    set_vram_2c00_2fff(bank_lookup, 0);
}

pub fn set_nametable_four_screen(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 1);
    set_vram_2800_2bff(bank_lookup, 2);
    set_vram_2c00_2fff(bank_lookup, 3);
}

pub fn set_nametable_diagonal(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 1);
    set_vram_2800_2bff(bank_lookup, 1);
    set_vram_2c00_2fff(bank_lookup, 0);
}

pub fn set_nametable_lshaped(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 1);
    set_vram_2800_2bff(bank_lookup, 1);
    set_vram_2c00_2fff(bank_lookup, 1);
}

pub fn set_nametable_three_screen_vertical(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 2);
    set_vram_2800_2bff(bank_lookup, 1);
    set_vram_2c00_2fff(bank_lookup, 2);
}

pub fn set_nametable_three_screen_horizontal(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 1);
    set_vram_2800_2bff(bank_lookup, 2);
    set_vram_2c00_2fff(bank_lookup, 2);
}

pub fn set_nametable_three_screen_diagonal(bank_lookup: &mut[Bank]) {
    set_vram_2000_23ff(bank_lookup, 0);
    set_vram_2400_27ff(bank_lookup, 1);
    set_vram_2800_2bff(bank_lookup, 1);
    set_vram_2c00_2fff(bank_lookup, 2);
}

// cpu 8k bank switching
pub fn set_wram8k_6000_7fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x1FFF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 13);
}

// cpu 4k bank switching
pub fn set_prg4k_8000_8fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_9000_9fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[1] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_a000_afff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[2] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_b000_bfff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[3] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_c000_cfff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[4] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_d000_dfff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[5] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_e000_efff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[6] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_prg4k_f000_ffff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x0FFF;
    bank_lookup[7] = Bank::new(bank_mask, bank_index << 12);
}

 // cpu 8k bank switching
 pub fn set_prg8k_8000_9fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x1FFF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 13);
    bank_lookup[1] = Bank::new(bank_mask, bank_index << 13);
}

pub fn set_prg8k_a000_bfff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x1FFF;
    bank_lookup[2] = Bank::new(bank_mask, bank_index << 13);
    bank_lookup[3] = Bank::new(bank_mask, bank_index << 13);
}

pub fn set_prg8k_c000_dfff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x1FFF;
    bank_lookup[4] = Bank::new(bank_mask, bank_index << 13);
    bank_lookup[5] = Bank::new(bank_mask, bank_index << 13);
}

pub fn set_prg8k_e000_ffff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x1FFF;
    bank_lookup[6] = Bank::new(bank_mask, bank_index << 13);
    bank_lookup[7] = Bank::new(bank_mask, bank_index << 13);
}

// cpu 16k bank switching
pub fn set_prg16k_8000_bfff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FFF;
    for i in 0..4 {
        bank_lookup[i] = Bank::new(bank_mask, bank_index << 14);
    }
}

pub fn set_prg16k_c000_ffff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FFF;
    for i in 4..8 {
        bank_lookup[i] = Bank::new(bank_mask, bank_index << 14);
    }
}

// cpu 32k bank switching
pub fn set_prg32k_8000_ffff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x7FFF;
    for i in 0..8 {
        bank_lookup[i] = Bank::new(bank_mask, bank_index << 15);
    }
}

// ppu 1k bank switching
pub fn set_chr1k_0000_03ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_0400_07ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[1] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_0800_0bff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[2] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_0c00_0fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[3] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_1000_03ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[4] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_1400_07ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[5] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_1800_0bff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[6] = Bank::new(bank_mask, bank_index << 10);
}

pub fn set_chr1k_1c00_0fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x3FF;
    bank_lookup[7] = Bank::new(bank_mask, bank_index << 10);
}

// ppu 2k bank switching
pub fn set_chr2k_0000_07ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x7FF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 11);
    bank_lookup[1] = Bank::new(bank_mask, bank_index << 11);
}

pub fn set_chr2k_0800_0fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x7FF;
    bank_lookup[2] = Bank::new(bank_mask, bank_index << 11);
    bank_lookup[3] = Bank::new(bank_mask, bank_index << 11);
}

pub fn set_chr2k_1000_17ff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x7FF;
    bank_lookup[4] = Bank::new(bank_mask, bank_index << 11);
    bank_lookup[5] = Bank::new(bank_mask, bank_index << 11);
}

pub fn set_chr2k_1800_1fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x7FF;
    bank_lookup[6] = Bank::new(bank_mask, bank_index << 11);
    bank_lookup[7] = Bank::new(bank_mask, bank_index << 11);
}

// ppu 4k bank switching
pub fn set_chr4k_0000_0fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0xFFF;
    bank_lookup[0] = Bank::new(bank_mask, bank_index << 12);
    bank_lookup[1] = Bank::new(bank_mask, bank_index << 12);
    bank_lookup[2] = Bank::new(bank_mask, bank_index << 12);
    bank_lookup[3] = Bank::new(bank_mask, bank_index << 12);
}

pub fn set_chr4k_1000_1fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0xFFF;
    bank_lookup[4] = Bank::new(bank_mask, bank_index << 12);
    bank_lookup[5] = Bank::new(bank_mask, bank_index << 12);
    bank_lookup[6] = Bank::new(bank_mask, bank_index << 12);
    bank_lookup[7] = Bank::new(bank_mask, bank_index << 12);
}

// ppu 8k bank switching
pub fn set_chr8k_0000_1fff(bank_lookup: &mut[Bank], bank_index: usize) {
    let bank_mask = 0x1FFF;
    for i in 0..8 {
        bank_lookup[i] = Bank::new(bank_mask, bank_index << 13);
    }
}

#[inline]
pub fn get_mem_address(bank: &Bank, address: u16) -> usize {
    bank.window | (address as usize & bank.offset_mask)
}

#[inline]
pub fn get_last_bank_index(bank_size: usize, data_size: usize) -> usize {
    (data_size / bank_size) - 1
}

pub fn set_nametable_from_mirroring_type(bank_lookup: &mut[Bank], mirror_type: ines::NametableMirroring) {
    // TODO update nes_rom crate to support other mirroring types
    match mirror_type {
        ines::NametableMirroring::Horizontal =>  set_nametable_horizontal(bank_lookup),
        ines::NametableMirroring::Vertical => set_nametable_vertical(bank_lookup),
        //ines::NametableMirroring::SingleScreen => set_nametable_single_screen(bank_lookup),
        ines::NametableMirroring::FourScreens => set_nametable_four_screen(bank_lookup),
        //ines::NametableMirroring::Diagonal => set_nametable_diagonal(bank_lookup),
        //ines::NametableMirroring::LShaped => set_nametable_lshaped(bank_lookup),
        //ines::NametableMirroring::ThreeScreenVertical => set_nametable_three_screen_vertical(bank_lookup),
        //ines::NametableMirroring::ThreeScreenHorizontal => set_nametable_three_screen_horizontal(bank_lookup),
        //ines::NametableMirroring::ThreeScreenDiagonal => set_nametable_three_screen_diagonal(bank_lookup),
        ines::NametableMirroring::Other => panic!("Invalid NROM nametable mirroring: {:?}", mirror_type),
    };
}

pub struct Context {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub sys_ram: Vec<u8>,
    pub vram: Vec<u8>,
    pub work_ram: Option<Vec<u8>>,           
    pub prg_bank_lookup: [Bank; 8],          // 4k smallest banks
    pub wram_bank_lookup: [Bank; 1],         // 8k smallest banks
    pub chr_bank_lookup: [Bank; 8],          // 1k smallest banks
    pub nametable_bank_lookup: [Bank; 4],    // 1k smallest banks
}

impl Context {
    pub fn new() -> Context {
        Context {
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            sys_ram: vec![0; SIZE_2K],
            vram: vec![0; SIZE_4K],
            work_ram: None,
            prg_bank_lookup: [Bank::new(0, 0); 8],
            wram_bank_lookup: [Bank::new(0, 0); 1],
            chr_bank_lookup: [Bank::new(0, 0); 8],
            nametable_bank_lookup: [Bank::new(0, 0); 4],
        }
    }
}

pub trait Mapper {
    // mainly to run emulator test roms that have a diffrent entry point
    fn change_rst_vector(&mut self, addr: u16);
    // CPU
    fn read_cpu_0000_1fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   // sys ram
    fn read_cpu_4020_5fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   //expansion space
    fn read_cpu_6000_7fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   // wram
    fn read_cpu_8000_8fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   // prg
    fn read_cpu_9000_9fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_cpu_a000_afff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_cpu_b000_bfff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_cpu_c000_cfff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_cpu_d000_dfff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_cpu_e000_efff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn read_cpu_f000_ffff(&mut self, pinout: mos::Pinout) -> mos::Pinout;

    fn write_cpu_0000_1fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_4020_5fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   
    fn write_cpu_6000_7fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   
    fn write_cpu_8000_8fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;   
    fn write_cpu_9000_9fff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_a000_afff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_b000_bfff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_c000_cfff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_d000_dfff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_e000_efff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn write_cpu_f000_ffff(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    // PPU   
    fn read_ppu_0000_03ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;    // chr
    fn read_ppu_0400_07ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout; 
    fn read_ppu_0800_0bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_0c00_0fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_1000_13ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_1400_17ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_1800_1bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_1c00_1fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_2000_23ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;    // vram
    fn read_ppu_2400_27ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_2800_2bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn read_ppu_2c00_2fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;

    fn write_ppu_0000_03ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_0400_07ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_0800_0bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_0c00_0fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_1000_13ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_1400_17ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_1800_1bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_1c00_1fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_2000_23ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_2400_27ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_2800_2bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_2c00_2fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    // used to monitor cpu and ppu buses for complex behaivor e.g. mmc5
    fn cpu_tick(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn ppu_tick(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    // no side effects from reading (e.g. mappers with memory mapped regs) used for debugging
    //fn peek_ppu(&mut self, addr: u16) -> u8;
}

pub fn create_mapper_null() -> Box<dyn Mapper> {
    Box::new(MapperNull {})
}

pub fn create_mapper(rom: &ines::Ines) -> Box<dyn Mapper> {
    match rom.mapper {
        0 => {
            Box::new(MapperNrom::from_ines(rom))
        }
        // TODO: add error handling instead of panicking like a monster
        _ => { panic!("mapper {} implementation not found", rom.mapper); }
    }
}