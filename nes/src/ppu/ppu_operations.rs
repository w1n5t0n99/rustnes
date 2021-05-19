use super::{Context, Pinout, Ctrl};
use super::ppu_registers::*;
use super::background::Background;
use super::sprites::Sprites;
use super::bus::Bus;
use crate::mappers::Mapper;

pub fn render_idle_cycle(ppu: &mut Context, bus: &mut Bus,  bg: &mut Background, mapper: &mut dyn Mapper) {
    // PPU address bus during this cycle appears to be the same CHR address
    // that is later used to fetch the next tile
    let b = bus.idle(mapper,  bg.pattern0_address(ppu));
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }
}

pub fn prerender_idle_cycle(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper) {
    let b = bus.idle(mapper,  ppu.addr_reg.vram_address());
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }
}

pub fn nonrender_cycle(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper) {
    let b = bus.idle(mapper,  ppu.addr_reg.vram_address());
    if b {  ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment_amount()); }
}

pub fn open_tile_index(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper) {
    bus.latch(mapper, ppu.addr_reg.tile_address());
}

pub fn read_tile_index(ppu: &mut Context, bus: &mut Bus, bg: &mut Background, mapper: &mut dyn Mapper) {
    let (d, b) = bus.read(mapper, ppu.addr_reg.tile_address());
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }

    bg.next_tile_index = d as u16;   // < ================ TODO change to mimic sprite definition
}

pub fn open_background_attribute(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper) {
    bus.latch(mapper, ppu.addr_reg.attribute_address());
}

pub fn read_background_attribute(ppu: &mut Context, bus: &mut Bus, bg: &mut Background, mapper: &mut dyn Mapper) {
    let (d, b) = bus.read(mapper, ppu.addr_reg.attribute_address());
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }

    bg.next_attribute = ppu.addr_reg.attribute_bits(d);
}

pub fn open_background_pattern0(ppu: &mut Context, bus: &mut Bus,  bg: &mut Background, mapper: &mut dyn Mapper) {
    bus.latch(mapper, bg.pattern0_address(ppu));
}

pub fn read_background_pattern0(ppu: &mut Context, bus: &mut Bus, bg: &mut Background, mapper: &mut dyn Mapper) {
    let (d, b) = bus.read(mapper, bg.pattern0_address(ppu));
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }
    
    bg.set_next_pattern0(d);
}

pub fn open_background_pattern1(ppu: &mut Context, bus: &mut Bus, bg: &mut Background, mapper: &mut dyn Mapper) {
    bus.latch(mapper, bg.pattern1_address(ppu));
}

pub fn read_background_pattern1(ppu: &mut Context, bus: &mut Bus, bg: &mut Background, mapper: &mut dyn Mapper) {
    let (d, b) = bus.read(mapper, bg.pattern1_address(ppu));

    if b {
        ppu.addr_reg.ppu_2007_during_render_increment();
    }
    else {
        ppu.addr_reg.coarse_x_increment();
    }

    bg.set_next_pattern1(d);
}

pub fn open_sprite_pattern0(ppu: &mut Context, bus: &mut Bus, sp: &mut Sprites, mapper: &mut dyn Mapper) {
    bus.latch(mapper, sp.pattern0_address(ppu));
}

pub fn read_sprite_pattern0(ppu: &mut Context, bus: &mut Bus, sp: &mut Sprites, mapper: &mut dyn Mapper) {
    let (d, b) = bus.read(mapper, sp.pattern0_address(ppu));
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }

    sp.set_pattern0(ppu, d);
}

pub fn open_sprite_pattern1(ppu: &mut Context, bus: &mut Bus, sp: &mut Sprites, mapper: &mut dyn Mapper) {
    bus.latch(mapper, sp.pattern1_address(ppu));
}

pub fn read_sprite_pattern1(ppu: &mut Context, bus: &mut Bus, sp: &mut Sprites, mapper: &mut dyn Mapper) {
    let (d, b) = bus.read(mapper, sp.pattern1_address(ppu));
    if b { ppu.addr_reg.ppu_2007_during_render_increment(); }

    sp.set_pattern1( ppu, d);
}

#[inline(always)]
pub fn enter_vblank(ppu: &mut Context, mut pinout: mos::Pinout) -> mos::Pinout {
    let ppu_diff = ppu.cycle - ppu.read_2002_cycle;
    
    match ppu_diff {
        0 => {
            // Reading on same cycle, reads as set but suppresses nmi
            ppu.status_reg.set(StatusRegister::VBLANK_STARTED, true);
            // TODO this also seems to hold true for 1 cycle after
        }
        1 => {
            // Reading one PPU clock before reads it as clear and never sets the flag or generates NMI for that frame.
        }
        _ => {
            ppu.status_reg.set(StatusRegister::VBLANK_STARTED, true);
            if ppu.control_reg.contains(ControlRegister::GENERATE_NMI) {
                ppu.prev_control_reg = ppu.control_reg;
                pinout.ctrl.set(mos::Ctrl::NMI, false);
            }
        }
    }

    pinout
}

#[inline(always)]
pub fn vblank_nmi_update(ppu: &mut Context, mut pinout: mos::Pinout) -> mos::Pinout {
    // during vblank if gen_nmi toggled and status reg not read generate another NMI
    if ppu.control_reg.contains(ControlRegister::GENERATE_NMI) &&
        ppu.status_reg.contains(StatusRegister::VBLANK_STARTED) &&
        !ppu.prev_control_reg.contains(ControlRegister::GENERATE_NMI) {
        pinout.ctrl.set(mos::Ctrl::NMI, false);
    }

    pinout
}

// TODO make sure palette read and writes are put on the bus for mmc5?
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

pub fn is_rendering(ppu: &mut Context) -> bool {
    if (ppu.scanline_index >= 240 && ppu.scanline_index <= 260) || ppu.mask_reg.rendering_enabled() == false {
        false
    }
    else {
        true
    }
}