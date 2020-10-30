pub mod ppu_viewer;
pub mod rp2c02;
mod ppu_bus;
mod ppu_registers;
mod core;

use std::fmt;

#[inline]
const fn to_address(address: u16, latch: u8) -> u16 {
    (address & 0xFF00) | (latch as u16) 
}

#[inline] 
const fn to_latch(address: u16) -> u8 {
    address as u8
}

#[inline] 
const fn to_data(address: u16) -> u8 {
    address as u8
}


bitflags! {
    pub struct Ctrl: u8 {
        const RD =   0b00000001;     // /RD read from VRAM. This is asserted when reading from palette
        const WR =   0b00000010;     // /WR write to VRAM. This is NOT asserted when writing to palette
        const ALE =  0b00000100;     // ALE goes high at beggining of VRAM access
    }
}

impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl::WR | Ctrl::RD
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pinout {
    pub address: u16,
    pub ale_latch: u8,
    pub ctrl: Ctrl,
}

impl fmt::Display for Pinout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{:#X} - ALE:{}R:{}W:{} - {:#X}", to_address(self.address, self.ale_latch), self.ctrl.contains(Ctrl::ALE) as u8,
            self.ctrl.contains(Ctrl::RD) as u8,  self.ctrl.contains(Ctrl::WR) as u8, to_data(self.address))
        
    }
}