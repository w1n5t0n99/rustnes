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
    match v {
        0x3F00..=0x3FFF => {
            fb[index] = ((pram.read(v) & ppu.mask_reg.monochrome_mask()) as u16) | ppu.mask_reg.emphasis_mask();
        }
        0x0000..=0x3EFF => {
            fb[index] = ((pram.read(0x00) & ppu.mask_reg.monochrome_mask()) as u16) | ppu.mask_reg.emphasis_mask();
        }
        _ => {
            panic!("blank pixel address out of range");
        }
    }   
}

pub fn scanline_render_tick(fb: &mut[u16], ppu: &mut Context, bus: &mut Bus, pram: &mut PaletteRam, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper) {
    match ppu.hpos {
        0 => { render_idle_cycle(ppu, bus, bg, mapper); ppu.hpos += 1; }
        // tile data fetched, sprites eval, pixels output
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
        
        65 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        66 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        67 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        68 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        69 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        70 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        71 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        72 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        73 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        74 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        75 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        76 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        77 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        78 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        79 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        80 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        81 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        82 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        83 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        84 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        85 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        86 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        87 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        88 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        89 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        90 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        91 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        92 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        93 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        94 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        95 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        96 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        97 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        98 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        99 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        100 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        101 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        102 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        103 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        104 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        105 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        106 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        107 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        108 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        109 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        110 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        111 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        112 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        113 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        114 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        115 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        116 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        117 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        118 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        119 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        120 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        121 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        122 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        123 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        124 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        125 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        126 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        127 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        128 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        129 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        130 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        131 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        132 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        133 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        134 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        135 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        136 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        137 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        138 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        139 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        140 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        141 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        142 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        143 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        144 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        145 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        146 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        147 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        148 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        149 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        150 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        151 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        152 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        153 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        154 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        155 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        156 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        157 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        158 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        159 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        160 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        161 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        162 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        163 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        164 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        165 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        166 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        167 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        168 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        169 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        170 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        171 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        172 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        173 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        174 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        175 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        176 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        177 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        178 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        179 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        180 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        181 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        182 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        183 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        184 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        185 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        186 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        187 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        188 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        189 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        190 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        191 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        192 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        193 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        194 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        195 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        196 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        197 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        198 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        199 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        200 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        201 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        202 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        203 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        204 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        205 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        206 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        207 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        208 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        209 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        210 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        211 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        212 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        213 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        214 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        215 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        216 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        217 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        218 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        219 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        220 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        221 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        222 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        223 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        224 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        225 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        226 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        227 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        228 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        229 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        230 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        231 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        232 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        233 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        234 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        235 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        236 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        237 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        238 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        239 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        240 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        241 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        242 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        243 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        244 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        245 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        246 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        247 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        248 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.hpos += 1;
        }
        249 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_tile_index(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        250 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_tile_index(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        251 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_attribute(ppu, bus, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        252 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_attribute(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        253 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        254 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern0(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        255 => {
            select_pixel(fb, ppu, pram, bg, sp);
            open_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            ppu.hpos += 1;
        }
        256 => {
            select_pixel(fb, ppu, pram, bg, sp);
            read_background_pattern1(ppu, bus, bg, mapper);
            sp.evaluate(ppu);
            bg.update_shift_registers_render();
            ppu.addr_reg.y_increment();
            ppu.hpos += 1;
        }
        // sprite tile data fetched, garbage nt and attr fetched
        257 => {
            open_tile_index(ppu, bus, mapper);
            sp.clear_oam_addr();
            sp.fetch_sprite_data(ppu);
            // update V horizontal bits
            ppu.addr_reg.update_x_scroll();
            ppu.hpos += 1;
        }
        258 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        259 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        260 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        261 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        262 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        263 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        264 => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        265 => { open_tile_index(ppu, bus, mapper); sp.fetch_sprite_data(ppu); ppu.hpos += 1; }
        266 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        267 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        268 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        269 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        270 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        271 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        272 => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        273 => { open_tile_index(ppu, bus, mapper); sp.fetch_sprite_data(ppu); ppu.hpos += 1; }
        274 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1;}
        275 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        276 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        277 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        278 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        279 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        280 => { 
            read_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        281 => {
            open_tile_index(ppu, bus, mapper);
            sp.fetch_sprite_data(ppu);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        282 => {
            read_tile_index(ppu, bus, bg, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        283 => {
            open_background_attribute(ppu, bus, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        284 => {
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        285 => { 
            open_sprite_pattern0(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        286 => { 
            read_sprite_pattern0(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        287 => { 
            open_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        288 => { 
            read_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        289 => {
            open_tile_index(ppu, bus, mapper);
            sp.fetch_sprite_data(ppu);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        290 => {
             read_tile_index(ppu, bus, bg, mapper);
             ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        291 => {
             open_background_attribute(ppu, bus, mapper);
             ppu.addr_reg.update_vertical();
             ppu.hpos += 1;
        }
        292 => {
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        293 => { 
            open_sprite_pattern0(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        294 => { 
            read_sprite_pattern0(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        295 => { 
            open_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        296 => { 
            read_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        297 => {
            open_tile_index(ppu, bus, mapper);
            sp.fetch_sprite_data(ppu);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        298 => {
             read_tile_index(ppu, bus, bg, mapper);
             ppu.addr_reg.update_vertical();
              ppu.hpos += 1;
        }
        299 => {
             open_background_attribute(ppu, bus, mapper);
             ppu.addr_reg.update_vertical();
              ppu.hpos += 1;
        }
        300 => {
            read_background_attribute(ppu, bus, bg, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        301 => { 
            open_sprite_pattern0(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1;
        }
        302 => { 
            read_sprite_pattern0(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        303 => { 
            open_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        304 => { 
            read_sprite_pattern1(ppu, bus, sp, mapper);
            ppu.addr_reg.update_vertical();
            ppu.hpos += 1; 
        }
        305 => { open_tile_index(ppu, bus, mapper); sp.fetch_sprite_data(ppu); ppu.hpos += 1; }
        306 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        307 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        308 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        309 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        310 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        311 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        312 => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        313 => { open_tile_index(ppu, bus, mapper); sp.fetch_sprite_data(ppu); ppu.hpos += 1; }
        314 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        315 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        316 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        317 => { open_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        318 => { read_sprite_pattern0(ppu, bus, sp, mapper); ppu.hpos += 1; }
        319 => { open_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        320 => { read_sprite_pattern1(ppu, bus, sp, mapper); ppu.hpos += 1; }
        // tile data fetched for next scanline
        321 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        322 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        323 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        324 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        325 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        326 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        327 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        328 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        329 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        330 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        331 => { open_background_attribute(ppu, bus, mapper); ppu.hpos += 1; }
        332 => { read_background_attribute(ppu, bus, bg, mapper); ppu.hpos += 1; }
        333 => { open_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        334 => { read_background_pattern0(ppu, bus, bg, mapper); ppu.hpos += 1; }
        335 => { open_background_pattern1(ppu, bus, bg, mapper); ppu.hpos += 1; }
        336 => {
            read_background_pattern1(ppu, bus, bg, mapper);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
        // garbage nt fetches
        337 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        338 => { read_tile_index(ppu, bus, bg, mapper); ppu.hpos += 1; }
        339 => { open_tile_index(ppu, bus, mapper); ppu.hpos += 1; }
        340 => { 
            read_tile_index(ppu, bus, bg, mapper);
            ppu.vpos += 1;
            // on odd frames this cycle is skipped , simulate by skipping the next render idle cycle
            ppu.hpos = 0;
        }
        _ => { panic!("render visible out of bounds"); }
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
