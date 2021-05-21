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

    pub fn get_pinout(&self) -> Pinout {
        self.pinout
    }

    pub fn set_pinout(&mut self, pinout: Pinout) {
        self.pinout = pinout;
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

    pub fn read(&mut self, mapper: &mut dyn Mapper, address: u16) -> (u8, bool) {
        self.pinout.ctrl.set(Ctrl::WR, false);
        self.pinout.ctrl.set(Ctrl::RD, false);
        self.pinout.ctrl.set(Ctrl::ALE, false);

        self.pinout.address = address;

        match self.wr_buffer.take() {
            Some(data) => {
                // if write buffer was filled then perform write instead
                self.pinout.data = data;
                self.internal_write(mapper);
                return (data, true);
            }
            None => { }
        }

        self.internal_read(mapper);

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
                self.internal_write(mapper);
                return true;
            }
            None => { }
        }

        match self.rd_buffer {
            Some(_) => { }
            None => { 
                self.internal_read(mapper);
                self.rd_buffer = Some(self.pinout.data);
                return true
            }
        }

        let tmp = self.palette_write;
        self.palette_write = false;
        tmp
    }

    fn internal_read(&mut self, mapper: &mut dyn Mapper) {
        self.pinout.ctrl.set(Ctrl::RD, true);
        match self.pinout.address {
            0x0000..=0x1fff => { self.pinout = mapper.read_ppu_chr(self.pinout); }
            0x2000..=0x3fff => { self.pinout = mapper.read_ppu_nt(self.pinout); }
            _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", self.pinout.address)
        }
    }

    fn internal_write(&mut self, mapper: &mut dyn Mapper) {
        self.pinout.ctrl.set(Ctrl::WR, true);
         match self.pinout.address {
            0x0000..=0x1fff => { self.pinout = mapper.write_ppu_chr(self.pinout); }
            0x2000..=0x3fff => { self.pinout = mapper.write_ppu_nt(self.pinout); }
            _ => panic!("ppu write {:#X} - should not be able to read past 0x2fff during rendering", self.pinout.address)
        }
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

        // dummy io read
        let v0 = bus.io_read();
        assert_ne!(v0, 255);
        
        let (d, b) = bus.read(&mut mapper, 0x1FF);
        assert_eq!(b, true);
        assert_eq!(d, 255);

        let v1 = bus.io_read();
        assert_eq!(v1, 255);

        // test io write
        let v0 = mapper.peek_chr(0x2FF);
        assert_eq!(v0, 0);

        bus.io_write(255);

        let (d, b) = bus.read(&mut mapper, 0x2FF);
        assert_eq!(d, 255);
        assert_eq!(b, true ); 
        assert_eq!(mapper.peek_chr(0x2FF), 255);
    }

    #[test]
    fn test_io_during_idle() {
        let mut mapper = MapperDebug::new();
        let mut bus = Bus::new();

        mapper.poke_chr(0x1FF, 255);
        mapper.poke_chr(0x200, 240);

        let v0 = bus.io_read();
        assert_ne!(v0, 255);

        // read
        let b = bus.idle(&mut mapper, 0x1FF);
        assert_eq!(bus.io_read(), 255);
        assert_eq!(b, true);

        let b = bus.idle(&mut mapper, 0x1FF);
        let b = bus.idle(&mut mapper, 0x1FF);
        assert_eq!(b, false);

        // write
        bus.io_write(128);
        let b = bus.idle(&mut mapper, 0x200);
        assert_eq!(b, true);
        assert_eq!(mapper.peek_chr(0x200), 128);

        let b = bus.idle(&mut mapper, 0x200);
        assert_eq!(b, false);

        bus.io_palette_write();
        let b = bus.idle(&mut mapper, 0x200);
        assert_eq!(b, true);
        assert_eq!(mapper.peek_chr(0x200), 128);
    }
}


