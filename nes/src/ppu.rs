use super::ppu_registers::{AddrReg, StatusRegister, ControlRegister, MaskRegister};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IOPinout {
    Clear,
    ALE,
    WR,
    RD,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderPinout {
    Clear,
    ALE,
    RD,
}

pub struct PPU {
    pub palette_ram: [u8; 32],
    pub oam_ram: [u8; 256],
    pub scanline: u16,
    pub scanline_cycle: u16,
    io_latch: u8,
    ppudata_latch: u8,
    addr_latch: u8,
    oam_addr: u8,
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
            addr_latch: 0,
            oam_addr: 0,
            addr_reg: AddrReg::new(),
            control_reg: ControlRegister::new(),
            mask_reg: MaskRegister::new(),
            status_reg: StatusRegister::new(),
        }
    }

    pub fn read_port(&self) -> u8 {
        // Reading a write only port returns data on internal data bus which acts as a dynamic latch
        self.io_latch
    }

    pub fn write_ppuctrl(&mut self, data: u8) {
        self.io_latch = data;
        self.control_reg.io_write(data);
    }

    pub fn write_ppumask(&mut self, data: u8) {
        self.io_latch = data;
        self.mask_reg.io_write(data);
    }

    pub fn read_ppustatus(&self) -> u8 {
        self.status_reg.io_read(self.io_latch)
    }

    pub fn write_ppustatus(&mut self, data: u8) {
        // Writing to a read only port fills the io latch
        self.io_latch = data;
    }

    pub fn write_oamaddr(&mut self, data: u8) {
        self.io_latch = data;
        self.oam_addr = data;
    }

    pub fn read_oamdata(&self) -> u8 {
        // Reads during vertical or forced blanking return the value from OAM at that address but do not increment
        self.oam_ram[self.oam_addr as usize]
    }

    pub fn write_oamdata(&mut self, data: u8) {
        self.io_latch = data;
        match self.scanline {
            0..=239 | 261 =>  {
                // No oam write, but performs glitchy increment
                let lb = self.oam_addr & 0x03;
                // Only increments the high 6 bits
                self.oam_addr = (self.oam_addr.wrapping_add(1) & 0xFC) | lb;
             }
             240..=260 => {
                 self.oam_ram[self.oam_addr as usize] = data;
                 self.oam_addr = self.oam_addr.wrapping_add(1);
             }
             _ => {
                 panic!("PPU Scanline out of range");
             }
        }
    }

    pub fn write_ppuscroll(&mut self, data: u8) {
        self.io_latch = data;
        self.addr_reg.io_write_2005(data);
    }

    pub fn write_ppuaddr(&mut self, data: u8) {
        self.io_latch = data;
        self.addr_reg.io_write_2006(data);
    }

    pub fn read_ppudata(&self) -> u8 {
        
        0
    }

}