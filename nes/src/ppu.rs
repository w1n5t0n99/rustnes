
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    
}

pub struct PPU {
    pub palette_ram: [u8; 32],
    pub oam_ram: [u8; 256],
    pub cycle: u64,
    pub scanline: u16,
    pub scanline_cycle: u16,
}

impl PPU {
    pub fn from_power_on() -> PPU {
        PPU {
            palette_ram: [0; 32],
            oam_ram: [0; 256],
            cycle: 0,
            scanline: 261,
            scanline_cycle: 0,
        }
    }
}