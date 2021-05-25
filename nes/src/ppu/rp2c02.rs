use std::{borrow::Borrow, thread::sleep};

use super::{Context, Pinout};
use super::bus::Bus;
use super::palette_ram::PaletteRam;
use super::background::Background;
use super::sprites::Sprites;
use super::scanline_postrender::scanline_postrender_tick;
use super::scanline_vblank::scanline_vblank_tick;
use super::scanline_prerender::{scanline_prerender_nonvisible_tick, scanline_prerender_tick};
use super::scanline_render::{scanline_render_nonvisible_tick, scanline_render_tick};
use super::ppu_operations::*;
use crate::mappers::Mapper;

const WRITE_BLOCK_CYCLES: u64 = 29658 * 3;

#[derive(Clone, Copy)]
pub struct Rp2c02 {
    context: Context,
    bus: Bus,
    palette_ram: PaletteRam,
    bg: Background,
    sp: Sprites,
}

impl Rp2c02 {
    pub fn from_power_on() -> Rp2c02 {
        Rp2c02 {
            context: Context::new(),
            bus: Bus::new(),
            palette_ram: PaletteRam::from_power_on(),
            bg: Background::new(),
            sp: Sprites::new(),
        }
    }

    pub fn from_reset(&self) -> Rp2c02 {
        let mut rp2c02 = Rp2c02 {
            context: Context::new(),
            bus: Bus::new(),
            palette_ram: self.palette_ram.from_reset(),
            bg: Background::new(),
            sp: Sprites::new(),
        };

        // ppuaddr is unchanged after reset
        rp2c02.context.addr_reg = self.context.addr_reg;
        rp2c02.context.addr_reg.w = false;
        rp2c02
    }

    pub fn frame_number(&self) -> u64 {
        self.context.frame
    }

    pub fn is_end_of_frame(&mut self) -> bool {
        self.context.last_frame_cycle
    }

    pub fn read_port(&self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data =  self.context.io_db;
        pinout
    }

    pub fn write_ppuctrl(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.prev_control_reg = self.context.control_reg;
        self.context.control_reg.io_write(pinout.data);
        self.context.addr_reg.io_write_2000(pinout.data);
        pinout
    }

    pub fn write_ppumask(&mut self, pinout: mos::Pinout) -> mos::Pinout{
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.mask_reg.io_write(pinout.data);
        pinout
    }

    pub fn read_ppustatus(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.context.read_2002_cycle = self.context.cycle;
        self.context.addr_reg.io_read_2002();
        pinout.data = self.context.status_reg.io_read(self.context.io_db);
        pinout
    }

    pub fn write_ppustatus(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // Writing to a read only port fills the io latch
        self.context.io_db = pinout.data;
        pinout
    }

    pub fn write_oamaddr(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        self.sp.io_write_2003(pinout.data);
        pinout
    }

    pub fn read_oamdata(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.sp.io_read_2004(&self.context);
        pinout
    }

    pub fn write_oamdata(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        self.sp.io_write_2004(&self.context, pinout.data);
        pinout
    }

    pub fn write_ppuscroll(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.addr_reg.io_write_2005(pinout.data);
        pinout
    }

    pub fn write_ppuaddr(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.addr_reg.io_write_2006(pinout.data);
        pinout
    }

    pub fn read_ppudata(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // Reading palette updates latch with contents of nametable under palette address
                pinout.data = if  is_rendering(&mut self.context) { self.palette_ram.read_during_render(v) } else { self.palette_ram.read(v) };
                // still need to update read buffer
                self.bus.io_palette_read();
            }
            0x0000..=0x3EFF => {
                pinout.data = self.bus.io_read();
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }

        self.context.io_db = pinout.data;
        pinout
    }

    pub fn write_ppudata(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // TODO not sure if the underlying address is written to like reading does
                self.palette_ram.write(v, pinout.data);
                self.bus.io_palette_write();
            }
            0x0000..=0x3EFF => {
                self.bus.io_write(pinout.data);
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }

        self.context.io_db = pinout.data;
        pinout
    }

    pub fn get_context(&self) -> Context {
        self.context
    }

    pub fn get_pinout(&self) -> Pinout {
        self.bus.get_pinout()
    }

    pub fn tick(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.context.last_frame_cycle = false;
        
        if self.context.cycle == WRITE_BLOCK_CYCLES {
            self.context.write_block = false;
        }

        match self.context.vpos {
            261 if self.context.mask_reg.rendering_enabled() => { scanline_prerender_tick(&mut self.context, &mut self.bus, &mut self.bg, &mut self.sp, mapper); }
            261 => { scanline_prerender_nonvisible_tick(&mut self.context, &mut self.bus, mapper); }
            0..=239 if self.context.mask_reg.rendering_enabled() => { scanline_render_tick(fb, &mut self.context, &mut self.bus, &mut self.palette_ram, &mut self.bg, &mut self.sp, mapper); }
            0..=239 => { scanline_render_nonvisible_tick(fb, &mut self.context, &mut self.bus, &mut self.palette_ram, mapper) }
            240 => { scanline_postrender_tick(&mut self.context, &mut self.bus, mapper); }
            241..=260 => { cpu_pinout = scanline_vblank_tick(&mut self.context, &mut self.bus,mapper, cpu_pinout); }
            _ => { panic!("Scanline index out of bounds"); }
        }

        self.context.cycle += 1;
        self.bus.set_pinout(mapper.ppu_tick(self.bus.get_pinout()));

        cpu_pinout
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::mappers::*;
    use mos::Pinout;

    #[test]
    fn test_ppudata_port() {
        let mut fb: Vec<u16> = vec![0; 256*240];
        let mut ppu = Rp2c02::from_power_on();
        let mut cpu_pinout = Pinout::new();
        
    }
}
