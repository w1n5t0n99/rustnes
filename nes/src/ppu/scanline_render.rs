use super::{Pinout, Context};
use super::background::Background;
use super::sprites::Sprites;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

fn scanline_render(fb: &mut[u16], ppu: &mut Context, bg: &mut Background, sp: &mut Sprites, mapper: &mut dyn Mapper, mut pinout: Pinout) {
    match ppu.scanline_dot {
        0 => {
            if ppu.scanline_index == 0 && ppu.odd_frame {
                // idle cycle skipped
                pinout = open_tile_index(ppu, mapper, pinout);
                fb[index] = select_pixel() as u16 | ppu.mask_reg.emphasis_mask();
            }
        }
    }
}