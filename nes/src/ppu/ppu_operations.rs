use super::{Pinout, Context, IO};
use super::ppu_registers::*;
use super::background::Background;
use super::sprites::{SpriteAttribute, SpriteData, Sprites};
use crate::mappers::Mapper;

const PATTERN0_INDEX: usize = 0;
const PATTERN0_OFFSET: u16 = 0;
const PATTERN1_INDEX: usize = 1;
const PATTERN1_OFFSET: u16 = 8;
const SPRITE_8X_VALUE: u8 = 8;
const SPRITE_16X_VALUE: u8 = 16;
const SPRITE_8X_FLIPMASK: u8 = 0b00000111;
const SPRITE_16X_FLIPMASK: u8 = 0b00001111;

#[inline(always)]
fn io_read(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    // assert rd pin, basically only used for debug info
    pinouts.0.rd();
    // read data
    let pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
    // set io rd buffer and io state
    ppu.rd_buffer = pinouts.0.data();
    ppu.io = IO::Idle;

    pinouts
}

#[inline(always)]
fn read(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    // assert rd pin, basically only used for debug info
    pinouts.0.rd();
    // read data
    pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
    pinouts
}

#[inline(always)]
fn io_write(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    // assert wr pin, basically only used for debug info
    pinouts.0.wr();
    // write data, must place on address bus
    pinouts.0.set_data(ppu.wr_buffer);
    pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
    // set io state
    ppu.io = IO::Idle;
    pinouts
}


pub fn render_idle_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.vram_address() & 0x2FFF);

    match ppu.io {
        IO::Idle => { },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => { ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn nonrender_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.vram_address() & 0x2FFF);
    match ppu.io {
        IO::Idle => { },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment()); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment()); },
        IO::WRPALETTE => { ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment()); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn open_tile_index(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.tile_address());

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn read_tile_index(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.tile_address());

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts);  },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    bg.next_tile_index = pinouts.0.data() as u16;
    pinouts
}

pub fn open_background_attribute(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.attribute_address());

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn read_background_attribute(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.attribute_address());

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts);  },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    bg.next_attribute = ppu.addr_reg.attribute_bits(pinouts.0.data());
    pinouts
}

pub fn open_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = bg.pattern0_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn read_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = bg.pattern0_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts);  },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    bg.next_pattern[PATTERN0_INDEX] = pinouts.0.data();
    pinouts
}

pub fn open_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = bg.pattern1_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn read_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = bg.pattern1_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.coarse_x_increment(); },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; ppu.addr_reg.coarse_x_increment(); },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; ppu.addr_reg.coarse_x_increment(); },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    bg.next_pattern[PATTERN1_INDEX] = pinouts.0.data();
    pinouts
}

pub fn open_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = sp.pattern0_address(ppu);
    pinouts.0.set_address(next_addr);


    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn read_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = sp.pattern0_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts); },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    sp.set_pattern0( pinouts.0.data());
    pinouts
}

pub fn open_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = sp.pattern1_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    pinouts
}

pub fn read_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = sp.pattern1_address(ppu);
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts); },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WRPALETTE => {pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); ppu.io = IO::Idle; }
    }

    sp.set_pattern1( pinouts.0.data());
    pinouts
}

#[inline(always)]
pub fn enter_vblank(ppu: &mut Context, mut pinout: mos::Pinout) -> mos::Pinout {
    // Reading one PPU clock before reads it as clear and never sets the flag
    // or generates NMI for that frame.
    if ppu.cycle != ppu.read_2002_cycle {
        ppu.status_reg.set(StatusRegister::VBLANK_STARTED, true);
    }

    if ppu.control_reg.contains(ControlRegister::GENERATE_NMI) {
        pinout.ctrl.set(mos::Ctrl::NMI, false);
    }

    pinout
}

pub fn read_palette_rendering(ppu: &mut Context, address: u16) -> u8 { 
    // only call if rendering enbabled
    let address = address & 0x1F;        
    match address {
        0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C => ppu.palette_ram[0x00],
        _ => ppu.palette_ram[address as usize],
    }
}

pub fn read_palette_nonrender(ppu: &mut Context, address: u16) -> u8 { 
    let address = address & 0x1F;
    match address {
        0x10 => ppu.palette_ram[0x00],
        0x14 => ppu.palette_ram[0x04],
        0x18 => ppu.palette_ram[0x08],
        0x1C => ppu.palette_ram[0x0C],
        _ => ppu.palette_ram[address as usize]
    }
}

#[inline(always)]
pub fn write_palette(ppu: &mut Context, address: u16, data: u8) { 
    let address = address & 0x1F;
    match address {
        0x10 => { ppu.palette_ram[0x00] = data; }
        0x14 => { ppu.palette_ram[0x04] = data; }
        0x18 => { ppu.palette_ram[0x08] = data; }
        0x1C => { ppu.palette_ram[0x0C] = data; }
        _ => { ppu.palette_ram[address as usize] = data; }
    }
}

pub fn is_odd_frame(ppu: &mut Context) -> bool {
    ppu.odd_frame
}

pub fn is_rendering(ppu: &mut Context) -> bool {
    if (ppu.scanline_index >= 240 && ppu.scanline_index <= 260) || ppu.mask_reg.rendering_enabled() == false {
        false
    }
    else {
        true
    }
}