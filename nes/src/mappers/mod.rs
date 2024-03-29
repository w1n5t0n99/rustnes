mod mapper_null;
mod mapper_nrom;
mod mapper1;
mod mapper3;
pub mod mapper_debug;

use super::ppu;
use super::utils::paging::*;
use mapper_nrom::MapperNrom;
use mapper1::Mapper1;
use mapper3::Mapper3;
use mapper_null::MapperNull;
use ::nes_rom::ines;

// nametable mirroring
pub fn set_nametable_horizontal(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 1, SIZE_1K);
}

pub fn set_nametable_vertical(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 1, SIZE_1K);
}

pub fn set_nametable_single_screen_lower(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 0, SIZE_1K);
}

pub fn set_nametable_single_screen_upper(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 1, SIZE_1K);
}

pub fn set_nametable_four_screen(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 2, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 3, SIZE_1K);
}

pub fn set_nametable_diagonal(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 0, SIZE_1K);
}

pub fn set_nametable_lshaped(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 1, SIZE_1K);
}

pub fn set_nametable_three_screen_vertical(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 2, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 2, SIZE_1K);
}

pub fn set_nametable_three_screen_horizontal(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 2, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 2, SIZE_1K);
}

pub fn set_nametable_three_screen_diagonal(context: &mut Context) {
    context.nt_addr_mapper.set_banking_region(0, 0, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(1, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(2, 1, SIZE_1K);
    context.nt_addr_mapper.set_banking_region(3, 2, SIZE_1K);
}

pub fn set_nametable_from_mirroring_type(context: &mut Context, mirror_type: ines::NametableMirroring) {
    // TODO update nes_rom crate to support other mirroring types
    match mirror_type {
        ines::NametableMirroring::Horizontal =>  set_nametable_horizontal(context),
        ines::NametableMirroring::Vertical => set_nametable_vertical(context),
        ines::NametableMirroring::FourScreens => set_nametable_four_screen(context),
        ines::NametableMirroring::Other => panic!("Invalid NROM nametable mirroring: {:?}", mirror_type),
    };
}

pub struct Context {  
    pub prg_addr_mapper: AddressMapper<32, 0x8000>,
    pub wram_addr_mapper: AddressMapper<8, 0x6000>,
    pub chr_addr_mapper: AddressMapper<8, 0>,
    pub nt_addr_mapper: AddressMapper<4, 0x2000>,
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr: Vec<u8>,
    pub sys_ram: Vec<u8>,
    pub vram: Vec<u8>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            prg_addr_mapper: AddressMapper::new(),
            wram_addr_mapper: AddressMapper::new(),
            chr_addr_mapper: AddressMapper::new(),
            nt_addr_mapper: AddressMapper::new(),
            prg_rom: Vec::new(),
            prg_ram: Vec::new(),
            chr: Vec::new(),
            sys_ram: vec![0; SIZE_2K],
            vram: vec![0; SIZE_4K],
        }
    }
}

pub trait Mapper {
    // CPU
    fn read_cpu_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout; 
    fn read_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout;   // 0x4020-0x6000
    fn read_cpu_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout;   
    fn read_cpu_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout;   

    fn write_cpu_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout; 
    fn write_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout;   // 0x4020-0x6000
    fn write_cpu_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout;   
    fn write_cpu_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout;   
    // PPU   
    fn read_ppu_chr(&mut self, pinout: ppu::Pinout) -> ppu::Pinout; 
    fn read_ppu_nt(&mut self, pinout: ppu::Pinout) -> ppu::Pinout; 

    fn write_ppu_chr(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    fn write_ppu_nt(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
    // used to monitor cpu and ppu buses for complex behaivor e.g. mmc5
    fn cpu_tick(&mut self, pinout: mos::Pinout) -> mos::Pinout;
    fn ppu_tick(&mut self, pinout: ppu::Pinout) -> ppu::Pinout;
}

pub fn create_mapper_null() -> Box<dyn Mapper> {
    Box::new(MapperNull {})
}

pub fn create_mapper(rom: &ines::Ines) -> Box<dyn Mapper> {
    match rom.mapper {
        0 => {
            Box::new(MapperNrom::from_ines(rom))
        }
        1 => {
            Box::new(Mapper1::from_ines(rom))
        }
        3 => {
            Box::new(Mapper3::from_ines(rom))
        }
        // TODO: add error handling instead of panicking like a monster
        _ => { panic!("mapper {} implementation not found", rom.mapper); }
    }
}