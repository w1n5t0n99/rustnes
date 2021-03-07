use super::{Context, Pinout, Ppu2007State};
use super::ppu_registers::*;
use super::background::Background;
use super::sprites::Sprites;
use crate::mappers::Mapper;

const PATTERN0_INDEX: usize = 0;
const PATTERN1_INDEX: usize = 1;

pub fn render_idle_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = ppu.addr_reg.vram_address() & 0x2FFF;
    match ppu.ppu_2007_state {
        Ppu2007State::Idle => { },
        Ppu2007State::Read => { 
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => { 
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    pinouts
}

pub fn nonrender_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = ppu.addr_reg.vram_address() & 0x2FFF;
    match ppu.ppu_2007_state {
        Ppu2007State::Idle => { },
        Ppu2007State::Read => { 
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment_amount());
        }
        Ppu2007State::Write => { 
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment_amount());
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    pinouts
}

pub fn open_tile_index(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = ppu.addr_reg.tile_address();

    pinouts
}

pub fn read_tile_index(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = ppu.addr_reg.tile_address();

    match ppu.ppu_2007_state { 
        Ppu2007State::Idle => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
        }
        Ppu2007State::Read => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => {
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    bg.next_tile_index = pinouts.0.data as u16;
    pinouts
}

pub fn open_background_attribute(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = ppu.addr_reg.attribute_address();

    pinouts
}

pub fn read_background_attribute(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = ppu.addr_reg.attribute_address();

    match ppu.ppu_2007_state { 
        Ppu2007State::Idle => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
        }
        Ppu2007State::Read => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => {
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    bg.next_attribute = ppu.addr_reg.attribute_bits(pinouts.0.data);
    pinouts
}

pub fn open_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = bg.pattern0_address(ppu);

    pinouts
}

pub fn read_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = bg.pattern0_address(ppu);

    match ppu.ppu_2007_state { 
        Ppu2007State::Idle => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
        }
        Ppu2007State::Read => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => {
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    bg.next_pattern[PATTERN0_INDEX] = pinouts.0.data;
    pinouts
}

pub fn open_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = bg.pattern1_address(ppu);

    pinouts
}

pub fn read_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = bg.pattern1_address(ppu);

    match ppu.ppu_2007_state { 
        Ppu2007State::Idle => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.coarse_x_increment();
        }
        Ppu2007State::Read => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => {
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    bg.next_pattern[PATTERN1_INDEX] = pinouts.0.data;
    pinouts
}

pub fn open_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = sp.pattern0_address(ppu);

    pinouts
}

pub fn read_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = sp.pattern0_address(ppu);

    match ppu.ppu_2007_state { 
        Ppu2007State::Idle => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
        }
        Ppu2007State::Read => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => {
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    sp.set_pattern0(ppu, pinouts.0.data);
    pinouts
}

pub fn open_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = sp.pattern1_address(ppu);

    pinouts
}

pub fn read_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinouts: (Pinout, mos::Pinout)) -> (Pinout, mos::Pinout) {
    pinouts.0.address = sp.pattern1_address(ppu);

    match ppu.ppu_2007_state { 
        Ppu2007State::Idle => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
        }
        Ppu2007State::Read => {
            pinouts = mapper.read_ppu(pinouts.0, pinouts.1);
            ppu.ppu_2007_rd_buffer = pinouts.0.data;
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
        Ppu2007State::Write => {
            pinouts.0.data = ppu.ppu_2007_wr_buffer;
            pinouts = mapper.write_ppu(pinouts.0, pinouts.1);
            ppu.addr_reg.ppu_2007_during_render_increment();
        }
    }

    ppu.ppu_2007_state = Ppu2007State::Idle;
    sp.set_pattern1( ppu, pinouts.0.data);
    pinouts
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
                pinout.ctrl.set(mos::Ctrl::NMI, false);
            }
        }
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