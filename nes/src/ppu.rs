use super::ppu_registers::{AddrReg, StatusRegister, ControlRegister, MaskRegister};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PPUStatus {
    
}

pub struct PPU {
    pub palette_ram: [u8; 32],
    pub oam_ram: [u8; 256],
    pub scanline: u16,
    pub scanline_cycle: u16,
    io_latch: u8,
    ppudata_latch: u8,
    addr_reg: AddrReg,
    control_reg: ControlRegister,
    mask_reg: MaskRegister,
    status_reg: StatusRegister,
}

impl PPU {
    pub fn from_power_on() -> PPU {
        PPU {
            palette_ram: [0; 32],
            oam_ram: [0; 256],
            scanline: 261,
            scanline_cycle: 0,
            io_latch: 0,
            ppudata_latch: 0,
            addr_reg: AddrReg::new(),
            control_reg: ControlRegister::new(),
            mask_reg: MaskRegister::new(),
            status_reg: StatusRegister::new(),
        }
    }


}