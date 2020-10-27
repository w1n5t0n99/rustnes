use super::ppu_registers::{AddrReg, StatusRegister, ControlRegister, MaskRegister};
use super::mappers::Mapper;

#[inline]
const fn to_address(address: u16, latch: u8) -> u16 {
    (address & 0xFF00) | (latch as u16) 
}

#[inline] 
const fn to_latch(address: u16) -> u8 {
    address as u8
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IoCtrl {
    RD,           // Read pin
    WR,           // Write pin
    RDALE,        // Address latch enable
    WRALE,        // Address latch enable
    Good,
}

pub struct PPU {
    pub palette_ram: [u8; 32],
    pub oam_ram: [u8; 256],
    pub scanline: u16,
    pub scanline_cycle: u16,
    pub io_ctrl: IoCtrl,
    io_latch: u8,                       // Reading a write only port returns data on internal data bus which acts as a dynamic latch
    ale_latch: u8,                      // Holds lower byter of address to read/write to
    ppudata_buffer: u8,                 // Buffered data read from PPUDATA
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
            io_ctrl: IoCtrl::Good,
            io_latch: 0,
            ale_latch: 0,
            ppudata_buffer: 0,
            oam_addr: 0,
            addr_reg: AddrReg::new(),
            control_reg: ControlRegister::new(),
            mask_reg: MaskRegister::new(),
            status_reg: StatusRegister::new(),
        }
    }

    pub fn io_read_port(&self) -> u8 {
        self.io_latch
    }

    pub fn io_write_ppuctrl(&mut self, data: u8) {
        self.io_latch = data;
        self.control_reg.io_write(data);
    }

    pub fn io_write_ppumask(&mut self, data: u8) {
        self.io_latch = data;
        self.mask_reg.io_write(data);
    }

    pub fn io_read_ppustatus(&self) -> u8 {
        self.status_reg.io_read(self.io_latch)
    }

    pub fn io_write_ppustatus(&mut self, data: u8) {
        // Writing to a read only port fills the io latch
        self.io_latch = data;
    }

    pub fn io_write_oamaddr(&mut self, data: u8) {
        self.io_latch = data;
        self.oam_addr = data;
    }

    pub fn io_read_oamdata(&mut self) -> u8 {
        // Reads during vertical or forced blanking return the value from OAM at that address but do not increment
        self.io_latch = self.oam_ram[self.oam_addr as usize];
        self.io_latch
    }

    pub fn io_write_oamdata(&mut self, data: u8) {
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

    pub fn io_write_ppuscroll(&mut self, data: u8) {
        self.io_latch = data;
        self.addr_reg.io_write_2005(data);
    }

    pub fn io_write_ppuaddr(&mut self, data: u8) {
        self.io_latch = data;
        self.addr_reg.io_write_2006(data);
    }

    pub fn io_read_ppudata(&mut self) -> u8 {
        self.io_ctrl = IoCtrl::RDALE;
        let v = self.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // Reading palette updates latch with contents of nametable under palette address
                self.io_latch = self.palette_ram[(v & 0x00FF) as usize];
                self.io_latch
            }
            0x0000..=0x3EFF => {
                self.io_latch = self.ppudata_buffer;
                self.ppudata_buffer
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }
    }

    pub fn io_write_ppudata(&mut self, data: u8) {
        self.io_latch = data;
        let v = self.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // TODO not sure if the underlying address is written to like reading does
                self.palette_ram[(v & 0x00FF) as usize] = data;
            }
            0x0000..=0x3EFF => {
                self.io_ctrl = IoCtrl::WRALE;
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }
    }

    fn ale_cycle(&mut self, address: u16, mapper: &mut dyn Mapper) {
        match self.io_ctrl {
            IoCtrl::Good => {
                 self.ale_latch = to_latch(address);
                 }
            IoCtrl::RDALE => {
                self.ale_latch = to_latch(address);
                self.io_ctrl = IoCtrl::RD;
            }
            IoCtrl::WRALE => {
                self.ale_latch = to_latch(address);
                self.io_ctrl = IoCtrl::WR;
            }
            IoCtrl::RD => {
                let v = to_address(address, self.ale_latch);
                //let data = match v {
                    //0x0000..=0x1FFF => { mapper.read_pattern_table(pinout) }
                //}
            }
            IoCtrl::WR => {

            }
        }
    }

}