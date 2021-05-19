use super::Context;
use super::bus::Bus;
use super::background::Background;
use super::sprites::Sprites;
use super::palette_ram::PaletteRam;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

fn select_pixel(fb: &mut[u16], ppu: &mut Context, pram: &mut PaletteRam, bg: &mut Background, sp: &mut Sprites) {
    let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
    // background pixel is default
    let mut pixel = bg.select_background_pixel(ppu);
    pixel = sp.select_sprite_pixel(ppu, pixel);

    fb[index] = ((pram.read_during_render(pixel as u16) & ppu.mask_reg.monochrome_mask()) as u16) | ppu.mask_reg.emphasis_mask();  
}

fn select_blank_pixel(fb: &mut[u16], ppu: &mut Context, pram: &mut PaletteRam) {
    let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
    let v = ppu.addr_reg.vram_address();
    if (v & 0x3F00) == 0x3F00 {
        fb[index] = ((pram.read(v) & ppu.mask_reg.monochrome_mask()) as u16) | ppu.mask_reg.emphasis_mask();
    }
    else {
        fb[index] = ((pram.read(0x00) & ppu.mask_reg.monochrome_mask()) as u16) | ppu.mask_reg.emphasis_mask();
    }    
}

pub fn scanline_render_tick(fb: &mut[u16], ppu: &mut Context, bus: &mut Bus, pram: &mut PaletteRam, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper) {
    match ppu.hpos {
        0 => { render_idle_cycle(ppu, bus, bg, mapper); ppu.hpos += 1; }
        1 => {
            select_pixel(fb, ppu, pram, bg, sp);

            open_tile_index(ppu, bus, mapper);
            sp.clear_secondary_oam();                // <======= TODO change to per cycle
            ppu.hpos += 1;
        }
        2 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        3 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        4 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        5 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        6 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        7 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        8 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        9 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        10 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        11 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        12 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        13 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        14 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        15 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        16 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        17 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        18 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        19 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        20 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        21 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        22 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        23 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        24 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        25 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        26 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        27 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        28 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        29 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        30 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        31 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        32 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        33 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        34 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        35 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        36 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        37 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        38 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        39 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        40 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        41 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        42 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        43 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        44 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        45 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        46 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        47 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        48 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        49 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        50 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        51 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        52 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        53 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        54 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        55 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        56 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        57 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        58 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        59 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        60 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        61 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        62 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        63 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            ppu.hpos += 1;
        }
        64 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
    }
}

pub fn scanline_render_nonvisible_tick(fb: &mut[u16], ppu: &mut Context, bus: &mut Bus, pram: &mut PaletteRam, mapper: &mut dyn Mapper) {
    match ppu.hpos {
        0 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        1..=256 => {
            // render blank pixel
            select_blank_pixel(fb, ppu, pram);
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        257..=339 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        340 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos = 0;
            ppu.vpos += 1;
        }
        _ => {
            panic!("PPU nonrender 0-340 out of bounds");
        }
    }
}
