use super::{Context, Pinout, Ctrl};
use super::ppu_registers::*;
use super::background::Background;
use super::sprites::Sprites;
use crate::mappers::Mapper;

fn read(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.ctrl.set(Ctrl::RD, false);
    pinout.ctrl.set(Ctrl::WR, true);
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

fn write(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.ctrl.set(Ctrl::WR, false);
    pinout.ctrl.set(Ctrl::RD, true);
    match pinout.address {
        0x0000..=0x03ff => { pinout = mapper.write_ppu_0000_03ff(pinout); }
        0x0400..=0x07ff => { pinout = mapper.write_ppu_0400_07ff(pinout); }
        0x0800..=0x0bff => { pinout = mapper.write_ppu_0800_0bff(pinout); }
        0x0c00..=0x0fff => { pinout = mapper.write_ppu_0c00_0fff(pinout); }
        0x1000..=0x13ff => { pinout = mapper.write_ppu_1000_13ff(pinout); }
        0x1400..=0x17ff => { pinout = mapper.write_ppu_1400_17ff(pinout); }
        0x1800..=0x1bff => { pinout = mapper.write_ppu_1800_1bff(pinout); }
        0x1c00..=0x1fff => { pinout = mapper.write_ppu_1c00_1fff(pinout); }
        0x2000..=0x23ff => { pinout = mapper.write_ppu_2000_23ff(pinout); }
        0x2400..=0x27ff => { pinout = mapper.write_ppu_2400_27ff(pinout); }
        0x2800..=0x2bff => { pinout = mapper.write_ppu_2800_2bff(pinout); }
        0x2c00..=0x2fff => { pinout = mapper.write_ppu_2c00_2fff(pinout); }
        0x3000..=0x33ff => { pinout = mapper.write_ppu_2000_23ff(pinout); }
        0x3400..=0x37ff => { pinout = mapper.write_ppu_2400_27ff(pinout); }
        0x3800..=0x3bff => { pinout = mapper.write_ppu_2800_2bff(pinout); }
        0x3c00..=0x3fff => { pinout = mapper.write_ppu_2c00_2fff(pinout); }
        _ => panic!("ppu write {:#X} - should be able to read 0x3fff", pinout.address)
    }
    pinout
}

fn idle(ppu: &mut Context, mut pinout: Pinout) -> Pinout {
    pinout.ctrl.set(Ctrl::RD, true);
    pinout.ctrl.set(Ctrl::WR, true);

    pinout
}

macro_rules! check_io_buffers_during_render {
    ($ppu:ident, $pinout:ident) => {
        if $ppu.ppu_2007_wr_buffer.is_some() {
            $ppu.ppu_2007_wr_buffer = None;
            $ppu.addr_reg.ppu_2007_during_render_increment();
            return $pinout;
        }

        if $ppu.ppu_2007_rd_buffer.is_none() {
            $ppu.ppu_2007_rd_buffer = Some($pinout.data);
            $ppu.addr_reg.ppu_2007_during_render_increment();
            return $pinout;
        }
    }
}

pub fn render_idle_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout = idle(ppu, pinout);
    pinout
}

pub fn nonrender_cycle(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = ppu.addr_reg.vram_address() & 0x2FFF;

    if ppu.ppu_2007_wr_buffer.is_some() {
        let buffer = ppu.ppu_2007_wr_buffer.take();
        pinout.data = buffer.unwrap();
        pinout = write(ppu, mapper, pinout);
        ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment_amount());
        return pinout;
    }

    if ppu.ppu_2007_rd_buffer.is_none() {
        ppu.ppu_2007_rd_buffer = Some(pinout.data);
        pinout = read(ppu, mapper, pinout);
        ppu.ppu_2007_rd_buffer = Some(pinout.data);
        ppu.addr_reg.increment(ppu.control_reg.vram_addr_increment_amount());
        return pinout;
    }

    pinout = idle(ppu, pinout);
    pinout
}

pub fn open_tile_index(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = ppu.addr_reg.tile_address();
    pinout = idle(ppu, pinout);
    pinout
}

pub fn read_tile_index(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = ppu.addr_reg.tile_address();
    pinout = read(ppu, mapper, pinout);
    bg.next_tile_index = pinout.data as u16;

    check_io_buffers_during_render!(ppu, pinout);

    pinout
}

pub fn open_background_attribute(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = ppu.addr_reg.attribute_address();
    pinout = idle(ppu, pinout);
    pinout
}

pub fn read_background_attribute(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = ppu.addr_reg.attribute_address();
    pinout = read(ppu, mapper, pinout);
    bg.next_attribute = ppu.addr_reg.attribute_bits(pinout.data);

    check_io_buffers_during_render!(ppu, pinout);

    pinout
}

pub fn open_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = bg.pattern0_address(ppu);
    pinout = idle(ppu, pinout);
    pinout
}

pub fn read_background_pattern0(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = bg.pattern0_address(ppu);
    pinout = read(ppu, mapper, pinout);
    bg.set_next_pattern0(pinout.data);

    check_io_buffers_during_render!(ppu, pinout);

    pinout
}

pub fn open_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = bg.pattern1_address(ppu);
    pinout = idle(ppu, pinout);
    pinout
}

pub fn read_background_pattern1(ppu: &mut Context, bg: &mut Background, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = bg.pattern1_address(ppu);
    pinout = read(ppu, mapper, pinout);
    bg.set_next_pattern1(pinout.data);

    check_io_buffers_during_render!(ppu, pinout);

    ppu.addr_reg.coarse_x_increment();
    pinout
}

pub fn open_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = sp.pattern0_address(ppu);
    pinout = idle(ppu, pinout);
    pinout
}

pub fn read_sprite_pattern0(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = sp.pattern0_address(ppu);
    pinout = read(ppu, mapper, pinout);
    sp.set_pattern0(ppu, pinout.data);

    check_io_buffers_during_render!(ppu, pinout);
    
    pinout
}

pub fn open_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = sp.pattern1_address(ppu);
    pinout = idle(ppu, pinout);
    pinout
}

pub fn read_sprite_pattern1(ppu: &mut Context, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    pinout.address = sp.pattern1_address(ppu);
    pinout = read(ppu, mapper, pinout);
    sp.set_pattern1( ppu, pinout.data);

    check_io_buffers_during_render!(ppu, pinout);

    pinout
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