use super::{Pinout, Context};
use super::background::Background;
use super::sprites::Sprites;
use super::palette_ram::PaletteRam;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

// only call if rendering enbabled
fn select_pixel(ppu: &mut Context, pram: &mut PaletteRam, bg: &mut Background, sp: &mut Sprites) -> u16 {
    // background pixel is default
    let mut pixel = bg.select_background_pixel(ppu);
    pixel = sp.select_sprite_pixel(ppu, pixel);

    ((pram.read_during_render(pixel as u16) & ppu.mask_reg.monochrome_mask()) as u16) | ppu.mask_reg.emphasis_mask()    
}

fn scanline_render_tick(fb: &mut[u16], ppu: &mut Context, pram: &mut PaletteRam, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    match ppu.hpos {
        0 => { pinout = render_idle_cycle(ppu, bg, mapper, pinout); ppu.hpos += 1; }
        1 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = open_tile_index(ppu, mapper, pinout);
            sp.clear_secondary_oam();
            ppu.hpos += 1;
        }
        2 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = read_tile_index(ppu, bg, mapper, pinout);
            ppu.hpos += 1;
        }
        3 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = open_background_attribute(ppu, mapper, pinout);
            ppu.hpos += 1;
        }
        4 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = read_background_attribute(ppu, bg, mapper, pinout);
            ppu.hpos += 1;
        }
        5 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = open_background_pattern0(ppu, bg, mapper, pinout);
            ppu.hpos += 1;
        }
        6 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = read_background_pattern0(ppu, bg, mapper, pinout);
            ppu.hpos += 1;
        }
        7 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = open_background_pattern1(ppu, bg, mapper, pinout);
            ppu.hpos += 1;
        }
        8 => {
            let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
            fb[index] = select_pixel(ppu, pram, bg, sp);

            pinout = read_background_pattern1(ppu, bg, mapper, pinout);
            bg.update_shift_registers_idle();
            ppu.hpos += 1;
        }
    }

    pinout
}