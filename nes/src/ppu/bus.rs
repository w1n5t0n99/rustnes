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

#[derive(Clone, Copy)]
enum IOAction {
    Idle,
    Read,
    Write,
    PaletteWrite,   // doesn't actually write on bus but still need to notify of bus action
}

// -- Bus --
#[derive(Clone, Copy)]
pub struct Bus {
    pinout: Pinout,
    rd_buffer: u8,
    wr_buffer: u8,
    io_action: IOAction,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            pinout: Pinout::new(),
            rd_buffer: 0,
            wr_buffer: 0,
            io_action: IOAction::Idle,
        }
    }

    pub fn get_pinout(&self) -> Pinout {
        self.pinout
    }

    pub fn set_pinout(&mut self, pinout: Pinout) {
        self.pinout = pinout;
    }

    pub fn io_read(&mut self) -> u8{
        self.io_action = IOAction::Read;
        self.rd_buffer
    }

    pub fn io_write(&mut self, data: u8) {
        self.io_action = IOAction::Write;
        self.wr_buffer = data;
    }

    pub fn io_palette_read(&mut self) {
        self.io_action = IOAction::Read;
    }

    pub fn io_palette_write(&mut self) {
        self.io_action = IOAction::PaletteWrite;
    }

    pub fn latch(&mut self, mapper: &mut dyn Mapper, address: u16) {
        // used for debugging
        self.pinout.ctrl.set(Ctrl::ALE, true);
        self.pinout.ctrl.set(Ctrl::WR, true);
        self.pinout.ctrl.set(Ctrl::RD, true);

        self.pinout.address = address;
    }

    pub fn read(&mut self, mapper: &mut dyn Mapper, address: u16) -> (u8, bool) {
        self.pinout.ctrl.set(Ctrl::ALE, false);
        self.pinout.ctrl.set(Ctrl::WR, true);
        // read is always asserted on read action
        self.pinout.ctrl.set(Ctrl::RD, false);


        self.pinout.address = address;

        match self.io_action {
            IOAction::Idle => {
                self.internal_read(mapper);
                return (self.pinout.data, false);
            }
            IOAction::Read => {
                self.io_action = IOAction::Idle;
                self.internal_read(mapper);
                self.rd_buffer = self.pinout.data;
                return (self.pinout.data, true);
            }
            IOAction::Write => {
                self.io_action = IOAction::Idle;
                 // if write buffer was filled then perform write instead
                 self.pinout.ctrl.set(Ctrl::WR, false);
                 self.pinout.data = self.wr_buffer;
                 self.internal_write(mapper);
                 return (self.pinout.data, true);
            }
            IOAction::PaletteWrite => {
                self.io_action = IOAction::Idle;
                self.internal_read(mapper);
                return (self.pinout.data, true);
            }
        }      
    }

    pub fn idle(&mut self, mapper: &mut dyn Mapper, address: u16) -> bool {
        self.pinout.ctrl.set(Ctrl::ALE, false);
        self.pinout.ctrl.set(Ctrl::WR, true);
        self.pinout.ctrl.set(Ctrl::RD, true);

        self.pinout.address = address;

        match self.io_action {
            IOAction::Idle => {
                return false;
            }
            IOAction::Read => {
                self.io_action = IOAction::Idle;
                self.pinout.ctrl.set(Ctrl::RD, false);
                self.internal_read(mapper);
                self.rd_buffer = self.pinout.data;
                return true;
            }
            IOAction::Write => {
                println!("WRITE BUS Address:{:#X} Data:{:#X}", self.pinout.address, self.pinout.data);

                self.io_action = IOAction::Idle;
                 self.pinout.ctrl.set(Ctrl::WR, false);
                 self.pinout.data = self.wr_buffer;
                 self.internal_write(mapper);
                 return  true;
            }
            IOAction::PaletteWrite => {
                self.io_action = IOAction::Idle;
                return true;
            }
        }      
    }

    fn internal_read(&mut self, mapper: &mut dyn Mapper) {
        self.pinout.ctrl.set(Ctrl::RD, false);
        match self.pinout.address {
            0x0000..=0x1fff => { self.pinout = mapper.read_ppu_chr(self.pinout); }
            0x2000..=0x3fff => { self.pinout = mapper.read_ppu_nt(self.pinout); }
            _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", self.pinout.address)
        }
    }

    fn internal_write(&mut self, mapper: &mut dyn Mapper) {
        self.pinout.ctrl.set(Ctrl::WR, false);
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


