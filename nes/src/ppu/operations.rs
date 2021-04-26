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
}

impl Bus {
    pub fn io_read(&mut self) -> u8{
        0
    }

    pub fn read(&mut self, pinout: Pinout) -> Pinout {
        pinout
    }
}



