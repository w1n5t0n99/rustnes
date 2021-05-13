use super::{Pinout, Context};
use super::ppu_operations::*;
use crate::mappers::Mapper;

fn scanline_postrender_tick(ppu: &mut Context, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
    match ppu.hpos {
        0..=339 => {
            pinout = nonrender_cycle(ppu, mapper, pinout);
            ppu.hpos += 1;
        }
        340 => {
            pinout = nonrender_cycle(ppu, mapper, pinout);
            ppu.hpos = 0;
            ppu.vpos += 1;
        }
        _ => {
            panic!("PPU postrender 0-340 out of bounds");
        }
    }

    pinout
}