use super::{Context, Pinout, Ctrl};
use super::ppu_registers::*;
use super::background::Background;
use super::sprites::Sprites;
use crate::mappers::Mapper;

const PATTERN0_INDEX: usize = 0;
const PATTERN1_INDEX: usize = 1;

fn read(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.ctrl.set(Ctrl::RD, false);
    match pinout.address {
        0x0000..=0x03ff => { pinout = mapper.read_ppu_0000_03ff(pinout); }
        0x0400..=0x07ff => { pinout = mapper.read_ppu_0400_07ff(pinout); }
        0x0800..=0x0bff => { pinout = mapper.read_ppu_0800_0bff(pinout); }
        0x0c00..=0x0fff => { pinout = mapper.read_ppu_0c00_0fff(pinout); }
        0x1000..=0x13ff => { pinout = mapper.read_ppu_1000_13ff(pinout); }
        0x1400..=0x17ff => { pinout = mapper.read_ppu_1400_17ff(pinout); }
        0x1800..=0x1bff => { pinout = mapper.read_ppu_1800_1bff(pinout); }
        0x1c00..=0x1fff => { pinout = mapper.read_ppu_1c00_1fff(pinout); }
        0x2000..=0x23ff => { pinout = mapper.read_ppu_2000_23ff(pinout); }
        0x2400..=0x27ff => { pinout = mapper.read_ppu_2400_27ff(pinout); }
        0x2800..=0x2bff => { pinout = mapper.read_ppu_2800_2bff(pinout); }
        0x2c00..=0x2fff => { pinout = mapper.read_ppu_2c00_2fff(pinout); }
        0x3000..=0x33ff => { pinout = mapper.read_ppu_2000_23ff(pinout); }
        0x3400..=0x37ff => { pinout = mapper.read_ppu_2400_27ff(pinout); }
        0x3800..=0x3bff => { pinout = mapper.read_ppu_2800_2bff(pinout); }
        0x3c00..=0x3fff => { pinout = mapper.read_ppu_2c00_2fff(pinout); }
        _ => panic!("ppu read {:#X} - should be able to read 0x3fff", pinout.address)
    }
    pinout
}

fn idle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.ctrl.set(Ctrl::RD, true);
    pinout.ctrl.set(Ctrl::WR, true);

    pinout
}

pub fn render_idle_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = ppu.addr_reg.vram_address() & 0x2FFF;
    pinout = read(ppu, mapper, pinout);

    if ppu.ppu_2007_wr_buffer.is_some() {
        ppu.ppu_2007_wr_buffer = None;
        ppu.addr_reg.ppu_2007_during_render_increment();
        return pinout;
    }

    if ppu.ppu_2007_rd_buffer.is_none() {
        ppu.ppu_2007_rd_buffer = Some(pinout.data);
        ppu.addr_reg.ppu_2007_during_render_increment();
        return pinout;
    }

    //Let the other ppu cycles handle reading and writing  ####################################################

    pinout
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