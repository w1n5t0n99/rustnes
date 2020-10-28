use super::ppu_registers::{AddrReg, StatusRegister, ControlRegister, MaskRegister};
use super::mappers::Mapper;

bitflags! {
    pub struct Ctrl: u8 {
        const RD =     0b00000001;       // Read pin
        const WR =     0b00000010;       // Write pin
        const RDALE =  0b00000100;       // Address latch enable
        const WRALE =  0b00001000;       // Address latch enable
    }
}

impl Ctrl {
    pub fn new() -> Self {
        Ctrl::empty()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pinout {
    pub address: u16,
    pub latch: u8,
    ctrl: Ctrl,
}

impl Pinout {
    pub fn new() -> Self {
        Pinout {
            address: 0,
            latch: 0,
            ctrl: Ctrl::new(),
        }
    }
}

pub struct PPU {
    pub palette_ram: [u8; 32],
    pub oam_ram: [u8; 256],
    pub scanline: u16,
    pub scanline_cycle: u16,
    pub pinout: Pinout,
    io_latch: u8,                       // Reading a write only port returns data on internal data bus which acts as a dynamic latch
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
            io_latch: 0,
            ppudata_buffer: 0,
            oam_addr: 0,
            addr_reg: AddrReg::new(),
            control_reg: ControlRegister::new(),
            mask_reg: MaskRegister::new(),
            status_reg: StatusRegister::new(),
            pinout: Pinout::new(),
        }
    }

    pub fn read_port(&self) -> u8 {
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

    pub fn read_oamdata(&mut self) -> u8 {
        // Reads during vertical or forced blanking return the value from OAM at that address but do not increment
        self.io_latch = self.oam_ram[self.oam_addr as usize];
        self.io_latch
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

    pub fn read_ppudata(&mut self) -> u8 {
        self.pinout.ctrl.set(Ctrl::RDALE, true);
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

    pub fn write_ppudata(&mut self, data: u8) {
        self.io_latch = data;
        let v = self.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // TODO not sure if the underlying address is written to like reading does
                self.palette_ram[(v & 0x00FF) as usize] = data;
            }
            0x0000..=0x3EFF => {
                self.pinout.ctrl.set(Ctrl::WRALE, true);
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }
    }

    fn ale_cycle(&mut self, mapper: &mut dyn Mapper) {
        

    }

}