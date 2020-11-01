use super::{Pinout, Context, IO};
use super::ppu_registers::*;
use crate::mappers::Mapper;

pub struct Rp2c02 {
    pub palette_ram: [u8; 32],
    pub oam_ram_primary: [u8; 256],
    context: Context,
    pinout: Pinout,   
}

impl Rp2c02 {
    pub fn from_power_on() -> Rp2c02 {
        Rp2c02 {
            palette_ram: [0; 32],
            oam_ram_primary: [0; 256],
            context: Context::new(),
            pinout: Pinout::new(),
        }
    }

    pub fn read_port(&self) -> u8 {
        self.context.io_db
    }

    pub fn write_ppuctrl(&mut self, data: u8) {
        self.context.io_db = data;
        self.context.control_reg.io_write(data);
    }

    pub fn write_ppumask(&mut self, data: u8) {
        self.context.io_db = data;
        self.context.mask_reg.io_write(data);
    }

    pub fn read_ppustatus(&self) -> u8 {
        self.context.status_reg.io_read(self.context.io_db)
    }

    pub fn write_ppustatus(&mut self, data: u8) {
        // Writing to a read only port fills the io latch
        self.context.io_db = data;
    }

    pub fn write_oamaddr(&mut self, data: u8) {
        self.context.io_db = data;
        self.context.oam_addr_reg = data;
    }

    pub fn read_oamdata(&mut self) -> u8 {
        match self.context.scanline_index {
            0..=239 | 261 if self.context.mask_reg.rendering_enabled() => {
                // TODO Reading OAMDATA while the PPU is rendering will expose internal OAM accesses during sprite evaluation and loading
                0
            }
            0..=239 | 261 => {
                // rendering disabled
                self.context.io_db = self.oam_ram_primary[self.context.oam_addr_reg as usize];
                self.context.io_db
            }
            240..=260 => {
                // Reads during vertical or forced blanking return the value from OAM at that address but do not increment
                self.context.io_db = self.oam_ram_primary[self.context.oam_addr_reg as usize];
                self.context.io_db
            }
            _ => {
                panic!("PPU Scanline out of range");
           }
        }
    }

    pub fn write_oamdata(&mut self, data: u8) {
        self.context.io_db = data;
        match self.context.scanline_index {
            0..=239 | 261 if self.context.mask_reg.rendering_enabled() => {
                // No oam write, but performs glitchy increment, only increments the high 6 bits
                // TODO possible implement glitchy increment
            }
            0..=239 | 261 => {
                self.oam_ram_primary[self.context.oam_addr_reg as usize] = data;
                self.context.oam_addr_reg = self.context.oam_addr_reg.wrapping_add(1);
            }
            240..=260 => {
                self.oam_ram_primary[self.context.oam_addr_reg as usize] = data;
                self.context.oam_addr_reg = self.context.oam_addr_reg.wrapping_add(1);
            }
            _ => {
                 panic!("PPU Scanline out of range");
            }
        }
    }

    pub fn write_ppuscroll(&mut self, data: u8) {
        self.context.io_db = data;
        self.context.addr_reg.io_write_2005(data);
    }

    pub fn write_ppuaddr(&mut self, data: u8) {
        self.context.io_db = data;
        self.context.addr_reg.io_write_2006(data);
    }

    pub fn read_ppudata(&mut self) -> u8 {
        self.context.io = IO::RDALE;
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // Reading palette updates latch with contents of nametable under palette address
                self.context.io_db = self.palette_ram[(v & 0x00FF) as usize];
                self.context.io_db
            }
            0x0000..=0x3EFF => {
                self.context.io_db = self.context.rd_buffer;
                self.context.rd_buffer
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }
    }

    pub fn write_ppudata(&mut self, data: u8) {
        self.context.io_db = data;
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // TODO not sure if the underlying address is written to like reading does
                self.palette_ram[(v & 0x00FF) as usize] = data;
            }
            0x0000..=0x3EFF => {
                self.context.io = IO::WRALE;
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }
    }

    pub fn tick(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.clear_ctrl();



        self.context.cycle += 1;
        cpu_pinout
    }

    fn open_tile_index(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.tile_address());
        self.pinout.latch_address();

        match self.context.io {
            IO::Idle => { },
            IO::RDALE => { self.context.io = IO::RD; },
            IO::WRALE => { self.context.io = IO::WR },
            IO::RD => {
                //self.context.rd_buffer = mapper.read_nametable(self.pinout.address_rd(), cpu_pinout);
                self.context.io = IO::Idle;
            },
            IO::WR => {
                //self.context.wr_buffer = mapper.read_nametable(self.pinout.address_rd(), cpu_pinout);
                self.context.io = IO::Idle;
            },
        }

        cpu_pinout
    }
}