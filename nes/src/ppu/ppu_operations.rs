use super::{Pinout, Context, IO};
use super::ppu_registers::*;
use super::ppu_renderer::{Background, Sprites};
use crate::mappers::Mapper;

const PATTERN0_INDEX: usize = 0;
const PATTERN0_OFFSET: u16 = 0;
const PATTERN1_INDEX: usize = 1;
const PATTERN1_OFFSET: u16 = 8;

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
    pinouts.0.set_address(ppu.addr_reg.vram_address());

    match ppu.io {
        IO::Idle => { },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}

pub fn nonrender_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.set_address(ppu.addr_reg.vram_address());

    match ppu.io {
        IO::Idle => { },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment()); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment()); },
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
    }

    bg.next_attribute = pinouts.0.data();
    pinouts
}

pub fn open_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = (ppu.control_reg.background_table_address() | (bg.next_tile_index << 4)  | PATTERN0_OFFSET | ppu.addr_reg.tile_line()) & 0xFFFF;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}

pub fn read_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = (ppu.control_reg.background_table_address() | (bg.next_tile_index << 4)  | PATTERN0_OFFSET | ppu.addr_reg.tile_line()) & 0xFFFF;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts);  },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
    }

    bg.next_pattern[PATTERN0_INDEX] = pinouts.0.data();
    pinouts
}

pub fn open_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = (ppu.control_reg.background_table_address() | (bg.next_tile_index << 4)  | PATTERN1_OFFSET | ppu.addr_reg.tile_line()) & 0xFFFF;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}

pub fn read_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = (ppu.control_reg.background_table_address() | (bg.next_tile_index << 4)  | PATTERN1_OFFSET | ppu.addr_reg.tile_line()) & 0xFFFF;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.coarse_x_increment(); },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; ppu.addr_reg.coarse_x_increment(); },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; ppu.addr_reg.coarse_x_increment(); },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
    }

    bg.next_pattern[PATTERN1_INDEX] = pinouts.0.data();
    pinouts
}

pub fn open_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = 0;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}

pub fn read_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = 0;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.coarse_x_increment(); },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; ppu.addr_reg.coarse_x_increment(); },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; ppu.addr_reg.coarse_x_increment(); },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}

pub fn open_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = 0;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts.0.latch_address(); },
        IO::RDALE => { pinouts.0.latch_address(); ppu.io = IO::RD; },
        IO::WRALE => { pinouts.0.latch_address(); ppu.io = IO::WR; },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}

pub fn read_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    let next_addr = 0;
    pinouts.0.set_address(next_addr);

    match ppu.io {
        IO::Idle => { pinouts = read(ppu, mapper, pinouts); ppu.addr_reg.coarse_x_increment(); },
        IO::RDALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::RD; ppu.addr_reg.coarse_x_increment(); },
        IO::WRALE => { pinouts = read(ppu, mapper, pinouts); pinouts.0.latch_address(); ppu.io = IO::WR; ppu.addr_reg.coarse_x_increment(); },
        IO::RD => { pinouts = io_read(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
        IO::WR => { pinouts = io_write(ppu, mapper, pinouts); ppu.addr_reg.quirky_increment(); },
    }

    pinouts
}