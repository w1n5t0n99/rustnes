use super::Context;
use super::bus::Bus;
use super::ppu_operations::*;
use crate::mappers::Mapper;


pub fn scanline_vblank_tick(ppu: &mut Context, bus: &mut Bus, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
    match ppu.hpos {
        0 => {
            if ppu.vpos == 241 { ppu.last_frame_cycle = true; ppu.frame += 1; }
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        1 => {
            if ppu.vpos == 241 { cpu_pinout = enter_vblank(ppu, cpu_pinout); }
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        2..=339 => {
            cpu_pinout = vblank_nmi_update(ppu, cpu_pinout);
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos += 1;
        }
        340 => {
            cpu_pinout = vblank_nmi_update(ppu, cpu_pinout);
            nonrender_cycle(ppu, bus, mapper);
            ppu.hpos = 0;
            ppu.vpos += 1;
        }
        _ => {
            panic!("PPU postrender 0-340 out of bounds");
        }
    }

    cpu_pinout
}