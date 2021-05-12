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

fn scanline_render(fb: &mut[u16], ppu: &mut Context, pram: &mut PaletteRam, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) {
    match ppu.scanline_dot {
        0 => {
            if ppu.scanline_index == 0 && ppu.odd_frame {
                // idle cycle skipped
                pinout = open_tile_index(ppu, mapper, pinout);
                // render pixel
                let index = ((ppu.hpos - 1) + (ppu.vpos * 256)) as usize;
                fb[index] = select_pixel(ppu, pram, bg, sp);
            }
        }
    }
}