use super::Context;
use super::bus::Bus;
use super::ppu_operations::*;
use crate::mappers::Mapper;

pub fn scanline_postrender_tick(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper) {
    match ppu.hpos {
        0..=339 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        340 => {
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos = 0;
            ppu.vpos += 1;
        }
        _ => {
            panic!("PPU postrender 0-340 out of bounds");
        }
    }
}