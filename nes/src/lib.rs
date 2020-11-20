pub mod error;
pub mod palette;
pub mod consoles;

mod dma;
mod mappers;
mod bus;
mod ppu;
mod controllers;

bitflags! {
    pub struct StandardInput: u8 {
        const A                  = 0b00000001;
        const B                  = 0b00000010;
        const Select             = 0b00000100;
        const Start              = 0b00001000;
        const Up                 = 0b00010000;
        const Down               = 0b00100000; 
        const Left               = 0b01000000;
        const Right              = 0b10000000;
    }
}

bitflags! {
    pub struct ZapperInput: u8 {
        const VsSerialData            = 0b00000001;
        const LightSense              = 0b00001000;
        const Trigger                 = 0b00010000;
    }
}

#[macro_use]
extern crate bitflags;

