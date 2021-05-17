use std::pin::Pin;

use super::{Context, Pinout, Ctrl};
use super::ppu_registers::*;
use super::background::Background;
use super::sprites::Sprites;
use crate::mappers::Mapper;

// -- operations --
// open_tile_index
// read_tile_index
// open_background_attribute
// read_sprite_pattern
// render_idle
// read_pattern
// .etc

// RenderAction
// IOAction

// -- Bus --
#[derive(Clone, Copy)]
pub struct Bus {
    pinout: Pinout,
    rd_buffer: Option<u8>,
    wr_buffer: Option<u8>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            pinout: Pinout::new(),
            rd_buffer: Some(0),
            wr_buffer: None,
        }
    }

    pub fn io_read(&mut self) -> u8{
        self.io_action = IOAction::LatchRead;
        self.rd_buffer
    }

    pub fn io_write(&mut self, data: u8) {
        self.io_action = IOAction::LatchWrite;
        self.wr_buffer = data;
    }

    pub fn read(&mut self, mapper: &mut dyn Mapper, address: u16) {
        
    }

    pub fn execute(&mut self, mapper: &mut dyn Mapper, render_action: RenderAction, mut pinout: Pinout) -> Pinout { 
        // clear ctrl pins
        pinout.ctrl.set(Ctrl::RD, true);
        pinout.ctrl.set(Ctrl::WR, true);
        pinout.ctrl.set(Ctrl::ALE, false);
        self.io_mem_access = false;

        match (render_action, self.io_action) {
            (RenderAction::Idle, IOAction::Idle) => { }
            (RenderAction::Idle, IOAction::LatchRead) => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_action = IOAction::Read;
            }
            (RenderAction::Idle, IOAction::LatchWrite) => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_action = IOAction::Write;
            }
            (RenderAction::Idle, IOAction::Read) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout = self.internal_read(mapper, pinout);
                self.rd_buffer = pinout.data;
                self.io_mem_access = true;
                self.io_action = IOAction::Idle;
            }
            (RenderAction::Idle, IOAction::Write) => {
                pinout.ctrl.set(Ctrl::WR, false);
                pinout = self.internal_write(mapper, pinout);
                self.io_mem_access = true;
                self.io_action = IOAction::Idle;
            }
            (RenderAction::Latch, IOAction::Idle) => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
            }
            (RenderAction::Latch, IOAction::LatchRead) => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_action = IOAction::Read;
            }
            (RenderAction::Latch, IOAction::LatchWrite) => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_action = IOAction::Write;
            }
            (RenderAction::Latch, IOAction::Read) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout = self.internal_read(mapper, pinout);
                self.internal_capture_latch_io_during_render(pinout);
                self.rd_buffer = pinout.data;
                self.io_action = IOAction::Idle;
                self.io_mem_access = true;
            }
            (RenderAction::Latch, IOAction::Write) => {
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout.ctrl.set(Ctrl::WR, false);
                pinout = self.internal_write(mapper, pinout);
                self.internal_capture_latch_io_during_render(pinout);
                self.io_action = IOAction::Idle;
                self.io_mem_access = true;
            }
            (RenderAction::Read, IOAction::Idle) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout = self.internal_read(mapper, pinout);
            }
            (RenderAction::Read, IOAction::LatchRead) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout = self.internal_read(mapper, pinout);
                self.internal_capture_latch_io_during_render(pinout);
                self.io_action = IOAction::Read;
            }
            (RenderAction::Read, IOAction::LatchWrite) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout = self.internal_read(mapper, pinout);
                self.internal_capture_latch_io_during_render(pinout);
                self.io_action = IOAction::Write;
            }
            (RenderAction::Read, IOAction::Read) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout = self.internal_read(mapper, pinout);
                self.rd_buffer = pinout.data;
                self.io_action = IOAction::Idle;
                self.io_mem_access = true;
            }
            (RenderAction::Read, IOAction::Write) => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::WR, false);
                pinout = self.internal_write(mapper, pinout);
                self.io_action = IOAction::Idle;
                self.io_mem_access = true;
            }
        }

        pinout
    }

    fn internal_read(&mut self, mapper: &mut dyn Mapper,  mut pinout: Pinout) -> Pinout {
        pinout = self.internal_apply_latch(pinout);

        match pinout.address {
            0x0000..=0x1fff => { pinout = mapper.read_ppu_chr(pinout); }
            0x2000..=0x3fff => { pinout = mapper.read_ppu_nt(pinout); }
            _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
        }

        pinout
    }

    fn internal_write(&mut self, mapper: &mut dyn Mapper,  mut pinout: Pinout) -> Pinout {
        // rendering never writes
        pinout.data = self.wr_buffer;
        pinout = self.internal_apply_latch(pinout);

        match pinout.address {
            0x0000..=0x1fff => { pinout = mapper.write_ppu_chr(pinout); }
            0x2000..=0x3fff => { pinout = mapper.write_ppu_nt(pinout); }
            _ => panic!("ppu write {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
        }

        pinout
    }

    fn internal_capture_latch(&mut self, pinout: Pinout) {
        self.latch = pinout.address as u8;
    }

    fn internal_capture_latch_io_during_render(&mut self, pinout: Pinout) {
        // cause when conflict between io and render latch/reads
        self.latch = pinout.data;
    }

    fn internal_apply_latch(&mut self, mut pinout: Pinout) -> Pinout {
        pinout.address = (pinout.address & 0xFF00) | (self.latch as u16);
        pinout
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mappers::mapper_debug::MapperDebug;

    #[test]
    fn test_render_access() {
        let mut p = Pinout::new();
        let mut mapper = MapperDebug::new();
        let mut bus = Bus::new();

        mapper.poke_chr(0x1FF, 255);
        p.address = 0x1FF;

        p = bus.execute(&mut mapper, RenderAction::Latch, p);
        assert_eq!(p.address, 0x1FF);
        p = bus.execute(&mut mapper, RenderAction::Read, p);
        assert_eq!(255, p.data);
    }

    #[test]
    fn test_io_access() {
        let mut p = Pinout::new();
        let mut mapper = MapperDebug::new();
        let mut bus = Bus::new();

        mapper.poke_chr(0x1FF, 255);

        // test io read
        p.address = 0x1FF;
        let v0 = bus.io_read();
        assert_ne!(v0, 255);
        
        p = bus.execute(&mut mapper, RenderAction::Idle, p);
        assert_eq!(p.address, 0x1FF);
        assert_eq!(bus.is_io_mem_access(), false);
        p = bus.execute(&mut mapper, RenderAction::Idle, p);
        assert_eq!(255, p.data);
        assert_eq!(bus.is_io_mem_access(), true);

        let v0 = bus.io_read();
        assert_eq!(v0, 255);

        // test io write
        p.address = 0x2FF;
        let v0 = mapper.peek_chr(0x2FF);
        assert_eq!(v0, 0);

        bus.io_write(255);

        p = bus.execute(&mut mapper, RenderAction::Idle, p);
        assert_eq!(p.address, 0x2FF);
        assert_eq!(bus.is_io_mem_access(), false);
        p = bus.execute(&mut mapper, RenderAction::Idle, p);
        assert_eq!(255, p.data);
        assert_eq!(bus.is_io_mem_access(), true);

        let v1 = mapper.peek_chr(0x2FF);
        assert_eq!(v1, 255);

        p.address = 0x300;
        p = bus.execute(&mut mapper, RenderAction::Idle, p);
        assert_eq!(p.address, 0x300);
        assert_eq!(bus.is_io_mem_access(), false);
    }

    #[test]
    fn test_io_read_during_render() {
        let mut p = Pinout::new();
        let mut mapper = MapperDebug::new();
        let mut bus = Bus::new();

        mapper.poke_chr(0x1FF, 255);
        mapper.poke_chr(0x200, 240);

        let v0 = bus.io_read();
        assert_ne!(v0, 255);

        // normal render read
        p.address = 0x1FF;
        p = bus.execute(&mut mapper, RenderAction::Latch, p);
        assert_eq!(p.address, 0x1FF);
        assert_eq!(bus.is_io_mem_access(), false);

        p = bus.execute(&mut mapper, RenderAction::Read, p);
        assert_eq!(255, p.data);
        assert_eq!(bus.is_io_mem_access(), true);

        // render read interrupted by io read
        p.address = 0x200;
        p = bus.execute(&mut mapper, RenderAction::Latch, p);
        assert_eq!(p.address, 0x200);
        assert_eq!(bus.is_io_mem_access(), false);

        let v0 = bus.io_read();
        assert_eq!(v0, 255);

        p = bus.execute(&mut mapper, RenderAction::Read, p);
        assert_eq!(p.address, 0x200);
        assert_eq!(p.data, 240);
        assert_eq!(bus.is_io_mem_access(), false);

        p.address = 0x201;
        p = bus.execute(&mut mapper, RenderAction::Latch, p);
        assert_eq!(p.address, (0x201 & 0xFF00) | 240);
        assert_eq!(bus.is_io_mem_access(), true);

    }
}


