
#[inline]
const fn to_address(address: u16, latch: u8) -> u16 {
    (address & 0xFF00) | (latch as u16) 
}

#[inline] 
const fn to_latch(address: u16) -> u8 {
    address as u8
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ctrl {
    RD,           // Read pin
    WR,           // Write pin
    RDALE,        // Address latch enable
    WRALE,        // Address latch enable
    Good,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PpuBus {
    pub addr_bus: u16,
    pub ale_latch: u8,
    pub io_ctrl: Ctrl,
    pub ren_ctrl: Ctrl,
}

impl PpuBus {
    pub fn new() -> Self {
        PpuBus {
            addr_bus: 0,
            ale_latch: 0,
            io_ctrl: Ctrl::Good,
            ren_ctrl: Ctrl::Good,
        }
    }
}