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
enum IOState {
    Idle,
    LatchWrite,
    LatchRead,
    Write,
    Read,
}

// -- Bus --
#[derive(Clone, Copy)]
pub struct Bus {
    rd_buffer: u8,
    wr_buffer: u8,
    latch: u8,
    io_state: IOState,
    io_mem_access: bool,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            rd_buffer: 0,
            wr_buffer: 0,
            latch: 0,
            io_state: IOState::Idle,
            io_mem_access: false,
        }
    }

    pub fn is_io_mem_access(&self) -> bool {
        self.io_mem_access
    }

    pub fn io_read(&mut self) -> u8{
        self.io_state = IOState::LatchRead;
        self.rd_buffer
    }

    pub fn io_write(&mut self, data: u8) {
        self.io_state = IOState::LatchWrite;
        self.wr_buffer = data;
    }

    pub fn render_read(&mut self, mapper: &mut dyn Mapper, mut pinout: Pinout) -> Pinout {
        // clear ctrl pins
        pinout.ctrl.set(Ctrl::RD, true);
        pinout.ctrl.set(Ctrl::WR, true);
        pinout.ctrl.set(Ctrl::ALE, false);

        match self.io_state {
            IOState::Idle => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.read_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.read_ppu_nt(pinout); }
                    _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }
                
                self.io_mem_access = false;
            }
            IOState::LatchRead => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.read_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.read_ppu_nt(pinout); }
                    _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }

                self.internal_capture_latch(pinout);
                self.io_state = IOState::Read;
                self.io_mem_access = false;
            }
            IOState::Read => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.read_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.read_ppu_nt(pinout); }
                    _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }
                
                self.rd_buffer = pinout.data;
                self.io_state = IOState::Idle;
                self.io_mem_access = true;
            }
            IOState::LatchWrite => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.read_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.read_ppu_nt(pinout); }
                    _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }

                self.internal_capture_latch(pinout);
                self.io_state = IOState::Write;
                self.io_mem_access = false;
            }
            IOState::Write => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::WR, false);
                pinout.data = self.wr_buffer;
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.write_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.write_ppu_nt(pinout); }
                    _ => panic!("ppu read/write {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }
                
                self.io_state = IOState::Idle;
                self.io_mem_access = true;
            }
        }

        pinout
    }

    pub fn render_latch(&mut self, mapper: &mut dyn Mapper,  mut pinout: Pinout) -> Pinout {
        // clear ctrl pins
        pinout.ctrl.set(Ctrl::RD, true);
        pinout.ctrl.set(Ctrl::WR, true);
        pinout.ctrl.set(Ctrl::ALE, false);

        match self.io_state {
            IOState::Idle => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
            }
            IOState::LatchRead => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_state = IOState::Read;
            }
            IOState::Read => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.read_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.read_ppu_nt(pinout); }
                    _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }

                self.internal_capture_latch(pinout);
                self.io_state = IOState::Idle;
                self.io_mem_access = true;
            }
            IOState::LatchWrite => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_state = IOState::Write;
            }
            IOState::Write => {
                pinout.ctrl.set(Ctrl::ALE, true);
                pinout.ctrl.set(Ctrl::RD, false);
                pinout.ctrl.set(Ctrl::WR, false);
                pinout.data = self.wr_buffer;
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.write_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.write_ppu_nt(pinout); }
                    _ => panic!("ppu read/write {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }
                
                self.internal_capture_latch(pinout);
                self.io_state = IOState::Idle;
                self.io_mem_access = true;
            }
        }

        pinout
    }

    pub fn render_idle(&mut self, mapper: &mut dyn Mapper,  mut pinout: Pinout) -> Pinout {
        // clear ctrl pins
        pinout.ctrl.set(Ctrl::RD, true);
        pinout.ctrl.set(Ctrl::WR, true);
        pinout.ctrl.set(Ctrl::ALE, false);

        match self.io_state {
            IOState::Idle => { }
            IOState::LatchRead => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_state = IOState::Read;
            }
            IOState::Read => {
                pinout.ctrl.set(Ctrl::RD, false);
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.read_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.read_ppu_nt(pinout); }
                    _ => panic!("ppu read {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }

                self.io_state = IOState::Idle;
                self.io_mem_access = true;
            }
            IOState::LatchWrite => {
                pinout.ctrl.set(Ctrl::ALE, true);
                self.internal_capture_latch(pinout);
                self.io_state = IOState::Write;   
            }
            IOState::Write => {
                pinout.ctrl.set(Ctrl::WR, false);
                pinout.data = self.wr_buffer;
                pinout = self.internal_apply_latch(pinout);

                match pinout.address {
                    0x0000..=0x01fff => { pinout = mapper.write_ppu_chr(pinout); }
                    0x2000..=0x2fff => { pinout = mapper.write_ppu_nt(pinout); }
                    _ => panic!("ppu read/write {:#X} - should not be able to read past 0x2fff during rendering", pinout.address)
                }
                
                self.io_state = IOState::Idle;
                self.io_mem_access = true;
            }
        }

        pinout
    }

    fn internal_capture_latch(&mut self, pinout: Pinout) {
        self.latch = pinout.address as u8;
    }

    fn internal_apply_latch(&mut self, mut pinout: Pinout) -> Pinout {
        pinout.address = (pinout.address & 0xFF00) | (self.latch as u16);
        pinout
    }
}



