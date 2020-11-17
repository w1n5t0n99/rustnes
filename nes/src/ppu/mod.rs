pub mod ppu_viewer;
pub mod rp2c02;
mod ppu_registers;
mod ppu_operations;
mod background;
mod sprites;

use std::fmt;

/*
The contents of the palette are unspecified at power on and unchanged at reset. 
During the warmup state, the PPU outputs a solid color screen based on the value at $3F00.ppu_viewer
This just gives and initial value for testing.
*/
pub static POWER_ON_PALETTE: [u8; 32] = [0x09, 0x01, 0x00, 0x01, 0x00, 0x02, 0x02, 0x0D, 0x08, 0x10, 0x08, 0x24, 0x00, 0x00, 0x04, 0x2C,
0x09, 0x01, 0x34, 0x03, 0x00, 0x04, 0x00, 0x14, 0x08, 0x3A, 0x00, 0x02, 0x00, 0x20, 0x2C, 0x08];

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
    address: u16,
    ale_latch: u8,
    ctrl: Ctrl,
}

impl Pinout {
    pub fn new() -> Self {
        Pinout {
            address: 0,
            ale_latch: 0,
            ctrl: Default::default(),
        }
    }

    pub fn set_address(&mut self, address: u16) {
        self.address = address;
    }

    pub fn latch_address(&mut self) {
        // latch = low byte of address bus
        self.ale_latch = self.address as u8;
        self.ctrl.set(Ctrl::ALE, true);
    }

    pub fn set_data(&mut self, data: u8) {
        self.address = (self.address & 0xFF00) | (data as u16);
    }

    pub fn clear_ctrl(&mut self) {
        self.ctrl = Default::default();
    }

    #[inline]
    pub fn address(&self) -> u16 {
        (self.address & 0xFF00) | (self.ale_latch as u16) 
    }

    #[inline]
    pub fn rd(&mut self) {
        self.ctrl.set(Ctrl::RD, false);
    }

    #[inline]
    pub fn wr(&mut self) {
        self.ctrl.set(Ctrl::WR, false);
    }

    #[inline]
    pub fn data(&self) -> u8 {
        self.address as u8
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IO {
    Idle,
    RDALE,
    RD,
    WRALE,
    WR,
    WRPALETTE,
}

impl fmt::Display for Pinout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "AB:{:#06X} - ALE:{}R:{}W:{} - Data:{:#04X} [{}]", self.address(), self.ctrl.contains(Ctrl::ALE) as u8,
            self.ctrl.contains(Ctrl::RD) as u8,  self.ctrl.contains(Ctrl::WR) as u8, self.data(), self.data())
        
    }
}

#[derive(Clone, Copy)]
pub struct Context {
    pub oam_ram_primary: [u8; 256],
    pub palette_ram: [u8; 32],
    pub cycle: u64,
    pub read_2002_cycle: u64,                           // Used to track NMI race condition
    pub addr_reg: ppu_registers::AddrReg,
    pub control_reg: ppu_registers::ControlRegister,
    pub mask_reg: ppu_registers::MaskRegister,
    pub status_reg: ppu_registers::StatusRegister,
    pub scanline_index: u16,
    pub scanline_dot: u16, 
    pub prev_scanline_index: u16,
    pub prev_scanline_dot: u16, 
    pub oam_addr_reg: u8,
    pub io_db: u8,                                      // Simulate latch created by long traces of data bus
    pub rd_buffer: u8,
    pub wr_buffer: u8,
    pub monochrome_mask: u8,
    pub io: IO,
    pub odd_frame: bool,
}

impl Context {
    pub fn new() -> Self {
        Context {
            oam_ram_primary: [0; 256],
            palette_ram: [0; 32],
            cycle: 0,
            read_2002_cycle: 0,
            addr_reg: ppu_registers::AddrReg::new(),
            control_reg: ppu_registers::ControlRegister::new(),
            mask_reg: ppu_registers::MaskRegister::new(),
            status_reg: ppu_registers::StatusRegister::new(),
            scanline_index: 0,
            scanline_dot: 1,
            prev_scanline_index: 0,
            prev_scanline_dot: 1,
            oam_addr_reg: 0,
            io_db: 0,
            rd_buffer: 0,
            wr_buffer: 0,
            monochrome_mask: 0xFF,
            io: IO::Idle,
            odd_frame: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pinout() {
        let mut pinout = Pinout::new();
        pinout.set_address(0xFF00);
        pinout.latch_address();
        pinout.set_address(0xFF00);

        assert_eq!(pinout.address(), 0xFF00);

        pinout.set_data(0x01);
        assert_eq!(pinout.address(), 0xFF00);
        pinout.latch_address();
        assert_eq!(pinout.address(), 0xFF01);

    }
}