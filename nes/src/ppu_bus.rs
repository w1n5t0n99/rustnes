use super::mappers::Mapper;


#[inline]
const fn to_address(address: u16, latch: u8) -> u16 {
    (address & 0xFF00) | (latch as u16) 
}

#[inline] 
const fn to_latch(address: u16) -> u8 {
    address as u8
}

bitflags! {
    struct Ctrl: u8 {
        const IoRd               = 0b00000001;
        const IoWr               = 0b00000010;
        const Rd                 = 0b00000100;
        const Wr                 = 0b00001000;
        const Ale                = 0b00010000;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PpuBus {
    pub addr_bus: u16,
    pub ale_latch: u8,
    pub rd_buffer: u8,
    pub wr_buffer: u8,
    ctrl: Ctrl,
}

impl PpuBus {
    pub fn new() -> Self {
        PpuBus {
            addr_bus: 0,
            ale_latch: 0,
            rd_buffer: 0,
            wr_buffer: 0,
            ctrl: Ctrl::empty(),
        }
    }   

    
    pub fn io_read(&mut self) -> u8 {
        self.ctrl.set(Ctrl::Ale, true);
        
        self.rd_buffer
    }

    pub fn io_read_palette(&mut self) {
        s//elf.io_ctrl = Ctrl::RDALE;
    }

    pub fn io_write(&mut self, data: u8) {
        //self.io_ctrl = Ctrl::WRALE;
        //self.wr_buffer = data;
    }




}