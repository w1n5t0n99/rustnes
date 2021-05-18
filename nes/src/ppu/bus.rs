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
    palette_write: bool,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            pinout: Pinout::new(),
            rd_buffer: Some(0),
            wr_buffer: None,
            palette_write: false,
        }
    }

    pub fn io_read(&mut self) -> u8{
        self.rd_buffer.take().unwrap()
    }

    pub fn io_write(&mut self, data: u8) {
        self.wr_buffer = Some(data);
    }

    pub fn io_palette_read(&mut self) {
        self.rd_buffer = None;
    }

    pub fn io_palette_write(&mut self) {
        self.palette_write = true;
    }

    pub fn latch(&mut self, mapper: &mut dyn Mapper, address: u16) {
        self.pinout.ctrl.set(Ctrl::WR, false);
        self.pinout.ctrl.set(Ctrl::RD, false);
        self.pinout.ctrl.set(Ctrl::ALE, true);
    }

    pub fn read(&mut self, mapper: &mut dyn Mapper, address: u16) -> (u8, bool) {
        self.pinout.ctrl.set(Ctrl::WR, false);
        self.pinout.ctrl.set(Ctrl::RD, false);
        self.pinout.ctrl.set(Ctrl::ALE, false);

        self.pinout.address = address;

        match self.wr_buffer.take() {
            Some(data) => {
                // if write buffer was filled then perform write instead
                self.pinout.data = data;
                self.pinout = self.internal_write(mapper, self.pinout);
                return (data, true);
            }
            None => { }
        }

        self.pinout = self.internal_read(mapper, self.pinout);

        match self.rd_buffer {
            Some(_) => { }
            None => { 
                self.rd_buffer = Some(self.pinout.data);
                return (self.pinout.data, true);
            }
        }

        let tmp = self.palette_write;
        self.palette_write = false;

        (self.pinout.data, tmp)        
    }

    pub fn idle(&mut self, mapper: &mut dyn Mapper, address: u16) -> bool {
        self.pinout.ctrl.set(Ctrl::WR, false);
        self.pinout.ctrl.set(Ctrl::RD, false);
        self.pinout.ctrl.set(Ctrl::ALE, false);

        self.pinout.address = address;

        match self.wr_buffer.take() {
            Some(data) => {
                // if write buffer was filled then perform write instead
                self.pinout.data = data;
                self.pinout = self.internal_write(mapper, self.pinout);
                return true;
            }
            None => { }
        }

        match self.rd_buffer {
            Some(_) => { }
            None => { 
                self.pinout = self.internal_read(mapper, self.pinout);
                self.rd_buffer = Some(self.pinout.data);
                return true
            }
        }

        let tmp = self.palette_write;
        self.palette_write = false;
        tmp
    }

    fn internal_read(&mut self, mapper: &mut dyn Mapper,  mut pinout: Pinout) -> Pinout {
        pinout.ctrl.set(Ctrl::RD, true);
        match pinout.address {
            0x0000..=0x1fff => { pinout = mapper.read_ppu_chr(pinout); }
            0x2000..=0x3fff => { pinout = mapper.read_ppu_nt(pinout); }
            _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
        }

        pinout
    }

    fn internal_write(&mut self, mapper: &mut dyn Mapper,  mut pinout: Pinout) -> Pinout {
        pinout.ctrl.set(Ctrl::WR, true);
         match pinout.address {
            0x0000..=0x1fff => { pinout = mapper.write_ppu_chr(pinout); }
            0x2000..=0x3fff => { pinout = mapper.write_ppu_nt(pinout); }
            _ => panic!("ppu write {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
        }

        pinout
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mappers::mapper_debug::MapperDebug;

    #[test]
    fn test_read_access() {
        let mut mapper = MapperDebug::new();
        let mut bus = Bus::new();

        mapper.poke_chr(0x1FF, 255);

        let (d, b) = bus.read(&mut mapper, 0x1FF);
        assert_eq!(255, d);
    }

    #[test]
    fn test_io_during_read() {
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


