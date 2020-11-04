use super::{Pinout, Context, IO};
use super::ppu_registers::*;
use crate::mappers::Mapper;

use std::fmt;

const PATTERN0_INDEX: usize = 0;
const PATTERN0_OFFSET: u16 = 0;
const PATTERN1_INDEX: usize = 1;
const PATTERN1_OFFSET: u16 = 8;

pub struct Rp2c02 {
    pub palette_ram: [u8; 32],
    pub oam_ram_primary: [u8; 256],
    pattern_queue: [u16; 2],
    attribute_queue: [u16; 2],
    next_pattern: [u8; 2],
    context: Context,
    pinout: Pinout,   
    next_tile_index: u16,
    next_attribute: u8,
    monochrome_mask: u8,
}

impl Rp2c02 {
    pub fn from_power_on() -> Rp2c02 {
        Rp2c02 {
            palette_ram: [0; 32],
            oam_ram_primary: [0; 256],
            pattern_queue: [0; 2],
            attribute_queue: [0; 2],
            next_pattern: [0; 2],
            context: Context::new(),
            pinout: Pinout::new(),
            next_tile_index: 0,
            next_attribute: 0,
            monochrome_mask: 0xFF,
        }
    }

    pub fn from_debug_values() -> Rp2c02 {
        let mut ppu = Rp2c02 {
            palette_ram: [0; 32],
            oam_ram_primary: [0; 256],
            pattern_queue: [0; 2],
            attribute_queue: [0; 2],
            next_pattern: [0; 2],
            context: Context::new(),
            pinout: Pinout::new(),
            next_tile_index: 0,
            next_attribute: 0,
            monochrome_mask: 0xFF
        };

        ppu.context.mask_reg.set(MaskRegister::SHOW_BACKGROUND, true);
        ppu.context.mask_reg.set(MaskRegister::SHOW_SPRITES, true);

        ppu
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

        self.monochrome_mask = if self.context.mask_reg.contains(MaskRegister::GREYSCALE) { 0x30 } else { 0xFF };
    }

    pub fn read_ppustatus(&mut self) -> u8 {
        self.context.read_2002_cycle = self.context.cycle;
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
                self.context.io_db = self.read_palette(v);
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
                self.write_palette(v, data);
            }
            0x0000..=0x3EFF => {
                self.context.io = IO::WRALE;
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }
    }

    pub fn tick(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // TODO add power on write block
        self.pinout.clear_ctrl();

        match self.context.scanline_index {
            261 => {
                cpu_pinout = self.scanline_prerender(mapper, cpu_pinout);
            }
            0..=239 => {
                cpu_pinout = self.scanline_render(fb, mapper, cpu_pinout);
            }
            240 => {
                cpu_pinout = self.scanline_postrender(mapper, cpu_pinout);
            }
            241..=260 => {
                cpu_pinout = self.scanline_vblank(mapper, cpu_pinout);
            }
            _ => {
                panic!("Scanline index out of bounds");
            }
        }


        self.context.cycle += 1;
        cpu_pinout
    }

    fn read_palette(&mut self, vaddr: u16) -> u8 { 
        /* 
        Addresses $3F04/$3F08/$3F0C can contain unique data, though these values are not used by the PPU when normally rendering
        (since the pattern values that would otherwise select those cells select the backdrop color instead)
        They can still be shown using the background palette hack during forced vblank
        */
        let addr = vaddr & 0x1F;        
        if self.context.status_reg.contains(StatusRegister::VBLANK_STARTED) == false && self.context.mask_reg.rendering_enabled()  == true {
            match addr {
                0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C => self.palette_ram[0x00],
                _ => self.palette_ram[addr as usize],
            }
        }
        else {
            match addr {
                0x10 => self.palette_ram[0x00],
                0x14 => self.palette_ram[0x04],
                0x18 => self.palette_ram[0x08],
                0x1C => self.palette_ram[0x0C],
                _ => self.palette_ram[addr as usize]
            }
        }
    }

    fn read_palette_rendering(&mut self, vaddr: u16) -> u8 { 
        // During rendering we don't need to check if rendering
        let addr = vaddr & 0x1F;        
        match addr {
            0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C => self.palette_ram[0x00],
            _ => self.palette_ram[addr as usize],
        }
    }

    fn write_palette(&mut self, vaddr: u16, data: u8) { 
        /*
        Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C.
        Note that this goes for writing as well as reading
        */
        let addr = vaddr & 0x1F;
        match addr {
            0x10 => { self.palette_ram[0x00] = data; }
            0x14 => { self.palette_ram[0x04] = data; }
            0x18 => { self.palette_ram[0x08] = data; }
            0x1C => { self.palette_ram[0x0C] = data; }
            _ => { self.palette_ram[addr as usize] = data; }
        }
    }

    pub fn select_blank_pixel(&self) -> u16 {
        let v = self.context.addr_reg.vram_address();
        if (v & 0x3F00) == 0x3F00 {
            (self.palette_ram[(v & 0x1F) as usize] & self.monochrome_mask) as u16  & self.context.mask_reg.emphasis_mask()
        }
        else {
            (self.palette_ram[0] & self.monochrome_mask) as u16  & self.context.mask_reg.emphasis_mask()
        }
    }

    // only call if rendering enbabled
    fn select_background_pixel(&mut self) -> u8 {
        let mut pixel: u8 = 0;
        if (self.context.mask_reg.contains(MaskRegister::LEFTMOST_8PXL_BACKGROUND) | (self.context.scanline_dot >= 8)) && self.context.mask_reg.contains(MaskRegister::SHOW_BACKGROUND) {
            let mask: u16 = 0x8000 >> self.context.addr_reg.x;

            pixel = (((self.pattern_queue[0] & mask) >> (15 - self.context.addr_reg.x)) |
            ((self.pattern_queue[1] & mask) >> (14 - self.context.addr_reg.x)) |
            ((self.attribute_queue[0] & mask) >> (13 - self.context.addr_reg.x)) |
            ((self.attribute_queue[1] & mask) >> (12 - self.context.addr_reg.x)) &
            0xFF) as u8;
        }
        else {
            pixel = 0x0;
        }

        self.pattern_queue[0] <<= 1;
	    self.pattern_queue[1] <<= 1;
	    self.attribute_queue[0] <<= 1;
        self.attribute_queue[1] <<= 1;
        
        pixel
    }

    // only call if rendering enbabled
    fn select_pixel(&mut self) -> u16 {
        // background pixel is default
        let mut pixel = self.select_background_pixel();
        // TODO see if sprite pixel overlaps

        // append color attenuation bits
        (pixel as u16) & self.context.mask_reg.emphasis_mask()
    }

    fn update_shift_registers_render(&mut self) {
        self.pattern_queue[0] |= (self.next_pattern[0] as u16);
	    self.pattern_queue[1] |= (self.next_pattern[1] as u16);
	    self.attribute_queue[0] |= (((self.next_attribute >> 0) & 0x01) * 0xff) as u16; // we multiply here to "replicate" this bit 8 times (it is used for a whole tile)
	    self.attribute_queue[1] |= (((self.next_attribute >> 1) & 0x01) * 0xff) as u16; // we multiply here to "replicate" this bit 8 times (it is used for a whole tile)
    }

    fn update_shift_registers_idle(&mut self) {
        self.pattern_queue[0] <<= 8;
        self.pattern_queue[1] <<= 8;
        self.attribute_queue[0] <<= 8;
        self.attribute_queue[1] <<= 8;
    
        self.update_shift_registers_render();
    }
    
    #[inline(always)]
    fn io_read(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // assert rd pin, basically only used for debug info
        self.pinout.rd();
        // read data
        let pinouts = mapper.read_ppu(self.pinout, cpu_pinout);
        self.pinout = pinouts.0;
        cpu_pinout = pinouts.1;
        // set io rd buffer and io state
        self.context.rd_buffer = self.pinout.data();
        self.context.io = IO::Idle;

        cpu_pinout
    }

    #[inline(always)]
    fn read(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // assert rd pin, basically only used for debug info
        self.pinout.rd();
        // read data
        let pinouts = mapper.read_ppu(self.pinout, cpu_pinout);
        self.pinout = pinouts.0;
        cpu_pinout = pinouts.1;

        cpu_pinout
    }

    #[inline(always)]
    fn io_write(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // assert wr pin, basically only used for debug info
        self.pinout.wr();
        // write data, must place on address bus
        self.pinout.set_data(self.context.wr_buffer);
        let pinouts = mapper.write_ppu(self.pinout, cpu_pinout);
        self.pinout = pinouts.0;
        cpu_pinout = pinouts.1;
        // set io state
        self.context.io = IO::Idle;

        cpu_pinout
    }

    fn idle_cycle(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.vram_address());

        match self.context.io {
            IO::Idle => { },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
        }

        cpu_pinout
    }

    fn nonrender_cycle(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.vram_address());

        match self.context.io {
            IO::Idle => { },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.increment(self.context.control_reg.vram_addr_increment()); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.increment(self.context.control_reg.vram_addr_increment()); },
        }

        cpu_pinout
    }

    fn open_tile_index(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.tile_address());

        match self.context.io {
            IO::Idle => { self.pinout.latch_address(); },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
        }

        cpu_pinout
    }

    fn read_tile_index(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.tile_address());

        match self.context.io {
            IO::Idle => { cpu_pinout = self.read(mapper, cpu_pinout);  },
            IO::RDALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
        }

        self.next_tile_index = self.pinout.data() as u16;
        cpu_pinout
    }

    fn open_background_attribute(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.attribute_address());

        match self.context.io {
            IO::Idle => { self.pinout.latch_address(); },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
        }

        cpu_pinout
    }

    fn read_background_attribute(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.pinout.set_address(self.context.addr_reg.attribute_address());

        match self.context.io {
            IO::Idle => { cpu_pinout = self.read(mapper, cpu_pinout);  },
            IO::RDALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
        }

        self.next_attribute = self.pinout.data();
        cpu_pinout
    }

    fn open_background_pattern0(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let next_addr = (self.context.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN0_OFFSET | self.context.addr_reg.tile_line()) & 0xFFFF;
        self.pinout.set_address(next_addr);

        match self.context.io {
            IO::Idle => { self.pinout.latch_address(); },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
        }

        cpu_pinout
    }

    fn read_background_pattern0(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let next_addr = (self.context.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN0_OFFSET | self.context.addr_reg.tile_line()) & 0xFFFF;
        self.pinout.set_address(next_addr);

        match self.context.io {
            IO::Idle => { cpu_pinout = self.read(mapper, cpu_pinout);  },
            IO::RDALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
        }

        self.next_pattern[PATTERN0_INDEX] = self.pinout.data();
        cpu_pinout
    }

    fn open_background_pattern1(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let next_addr = (self.context.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN1_OFFSET | self.context.addr_reg.tile_line()) & 0xFFFF;
        self.pinout.set_address(next_addr);

        match self.context.io {
            IO::Idle => { self.pinout.latch_address(); },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
        }

        cpu_pinout
    }

    fn read_background_pattern1(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let next_addr = (self.context.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN1_OFFSET | self.context.addr_reg.tile_line()) & 0xFFFF;
        self.pinout.set_address(next_addr);

        match self.context.io {
            IO::Idle => { cpu_pinout = self.read(mapper, cpu_pinout); self.context.addr_reg.coarse_x_increment(); },
            IO::RDALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::RD; self.context.addr_reg.coarse_x_increment(); },
            IO::WRALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::WR; self.context.addr_reg.coarse_x_increment(); },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
        }

        self.next_pattern[PATTERN1_INDEX] = self.pinout.data();
        cpu_pinout
    }

    fn open_garbage_pattern(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let next_addr = (self.context.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN1_OFFSET | self.context.addr_reg.tile_line()) & 0xFFFF;
        self.pinout.set_address(next_addr);

        match self.context.io {
            IO::Idle => { self.pinout.latch_address(); },
            IO::RDALE => { self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.pinout.latch_address(); self.context.addr_reg.quirky_increment(); },
        }

        cpu_pinout
    }

    fn read_garbage_pattern(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let next_addr = (self.context.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN1_OFFSET | self.context.addr_reg.tile_line()) & 0xFFFF;
        self.pinout.set_address(next_addr);

        match self.context.io {
            IO::Idle => { cpu_pinout = self.read(mapper, cpu_pinout); },
            IO::RDALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::RD; },
            IO::WRALE => { cpu_pinout = self.read(mapper, cpu_pinout); self.pinout.latch_address(); self.context.io = IO::WR; },
            IO::RD => { cpu_pinout = self.io_read(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
            IO::WR => { cpu_pinout = self.io_write(mapper, cpu_pinout); self.context.addr_reg.quirky_increment(); },
        }

        self.next_pattern[PATTERN1_INDEX] = self.pinout.data();
        cpu_pinout
    }

    fn enter_vblank(&mut self) {
        // Reading one PPU clock before reads it as clear and never sets the flag
        // or generates NMI for that frame.
        if self.context.cycle != self.context.read_2002_cycle {
            self.context.status_reg.set(StatusRegister::VBLANK_STARTED, true);
        }
    }

    fn scanline_prerender(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        if self.context.scanline_dot == 0 {
            self.context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
            self.context.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
        }

        if self.context.scanline_dot == 1 {
            self.context.status_reg.set(StatusRegister::VBLANK_STARTED, false);
        }

        if self.context.mask_reg.rendering_enabled() == true {
            match self.context.scanline_dot {
                0 => {
                     cpu_pinout = self.idle_cycle(mapper, cpu_pinout) 
                },
                1..=256 => {
                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            // eval sprites odd
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            // eval sprites even
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern0(mapper, cpu_pinout);
                        }
                        6 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern0(mapper, cpu_pinout);
                        }
                        7 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern1(mapper, cpu_pinout);
                        }
                        0 => {
                            self.update_shift_registers_idle();
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern1(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 1-256 out of bounds");
                        }
                    }

                    if self.context.scanline_dot == 256 {
                        self.context.addr_reg.y_increment();
                    }
                },
                257..=279 => {
                    if self.context.scanline_dot == 257 {
                        self.context.addr_reg.update_x_scroll();
                    }

                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        6 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        7 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        0 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 257-279 out of bounds");
                        }
                    }
                },
                280..=304 => {
                    self.context.addr_reg.update_vertical();
                    // update sprite registers

                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        6 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        7 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        0 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 280-304 out of bounds");
                        }
                    }
                }
                305..=320 => {
                    // update sprite registers

                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        6 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        7 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        0 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 305-321 out of bounds");
                        }
                    }
                }
                321..=336 => {
                    // two tiles for next scanline fetched
                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            // eval sprites odd
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            // eval sprites even
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern0(mapper, cpu_pinout);
                        }
                        6 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern0(mapper, cpu_pinout);
                        }
                        7 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern1(mapper, cpu_pinout);
                        }
                        0 => {
                            self.update_shift_registers_idle();
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern1(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 321-336 out of bounds");
                        }
                    }
                }
                337..=340 => {
                    // garbage nametable fetchs
                    match self.context.scanline_dot {
                        337 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        338 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        339 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        340 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 337-340 out of bounds");
                        }
                    }
                }
                _ => {
                    panic!("PPU prerender 0-340 out of bounds");
                }
            }
        }
        else {
            cpu_pinout = self.nonrender_cycle(mapper, cpu_pinout);
        }

        if self.context.scanline_dot == 340 {
            self.context.scanline_index = 0;
            if self.context.odd_frame == true { self.context.scanline_dot = 1; }
            else { self.context.scanline_dot = 0; }
        }
        else {
            self.context.scanline_dot += 1;
        }

        cpu_pinout
    }

    fn scanline_render(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {

        if self.context.mask_reg.rendering_enabled() == true {
            match self.context.scanline_dot {
                0 => {
                    // idle cycle
                    cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                }
                1..=256 => {
                    // render pixel
                    let index = ((self.context.scanline_dot - 1) * self.context.scanline_index) as usize;
                    fb[index] = 0x1F;

                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            // eval sprites odd
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            // eval sprites even
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern0(mapper, cpu_pinout);
                        }
                        6 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern0(mapper, cpu_pinout);
                        }
                        7 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern1(mapper, cpu_pinout);
                        }
                        0 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern1(mapper, cpu_pinout);
                            self.update_shift_registers_render();
                        }
                        _ => {
                            panic!("ppu 1-256 out of bounds");
                        }
                    }

                    if self.context.scanline_dot == 256 {
                        self.context.addr_reg.y_increment();
                    }
                },
                257..=320 => {
                    if self.context.scanline_dot == 257 {
                        self.context.addr_reg.update_x_scroll();
                    }

                    // update sprite registers
                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        6 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        7 => {
                            // open sprite pattern
                            cpu_pinout = self.open_garbage_pattern(mapper, cpu_pinout);
                        }
                        0 => {
                            // read sprite pattern
                            cpu_pinout = self.read_garbage_pattern(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 305-321 out of bounds");
                        }
                    }
                }
                321..=336 => {
                    // two tiles for next scanline fetched
                    match self.context.scanline_dot & 0x07 {
                        1 => {
                            // eval sprites odd
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        2 => {
                            // eval sprites even
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        3 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_attribute(mapper, cpu_pinout);
                        }
                        4 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_attribute(mapper, cpu_pinout);
                        }
                        5 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern0(mapper, cpu_pinout);
                        }
                        6 => {
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern0(mapper, cpu_pinout);
                        }
                        7 => {
                            // eval sprites odd
                            cpu_pinout = self.open_background_pattern1(mapper, cpu_pinout);
                        }
                        0 => {
                            self.update_shift_registers_idle();
                            // eval sprites even
                            cpu_pinout = self.read_background_pattern1(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 321-336 out of bounds");
                        }
                    }
                }
                337..=340 => {
                    // garbage nametable fetchs
                    match self.context.scanline_dot {
                        337 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        338 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        339 => {
                            cpu_pinout = self.open_tile_index(mapper, cpu_pinout);
                        }
                        340 => {
                            cpu_pinout = self.read_tile_index(mapper, cpu_pinout);
                        }
                        _ => {
                            panic!("ppu 337-340 out of bounds");
                        }
                    }
                }
                _ => {
                    panic!("PPU render 0-340 out of bounds");
                }
            }
        }
        else {
            // render blank pixel
            let index = ((self.context.scanline_dot - 1) * self.context.scanline_index) as usize;
            fb[index] = self.select_blank_pixel();
            
            cpu_pinout = self.nonrender_cycle(mapper, cpu_pinout);
        }


        if self.context.scanline_dot == 340 {
            self.context.scanline_index += 1;
            self.context.scanline_dot = 0;
        }
        else {
            self.context.scanline_dot += 1;
        }

        cpu_pinout
    }

    fn scanline_postrender(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        match self.context.scanline_dot {
            0..=339 => {
                cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                self.context.scanline_dot += 1;
            }
            340 => {
                cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                self.context.scanline_index += 1;
                self.context.scanline_dot = 0;
            }
            _ => {
                panic!("PPU postrender 0-340 out of bounds");
            }
        }

        cpu_pinout
    }

    fn scanline_vblank(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // TODO add support for multipe NMIs
        match self.context.scanline_dot {
            0 => {
                cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                self.context.scanline_dot += 1;
            }
            1 => {
                self.enter_vblank();
                if self.context.status_reg.contains(StatusRegister::VBLANK_STARTED) && self.context.control_reg.contains(ControlRegister::GENERATE_NMI) {
                    cpu_pinout.ctrl.set(mos::Ctrl::NMI, false);
                }

                cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                self.context.scanline_dot += 1;
            }
            2..=339 => {
                cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                self.context.scanline_dot += 1;
            }
            340 => {
                cpu_pinout = self.idle_cycle(mapper, cpu_pinout);
                self.context.scanline_index += 1;
                self.context.scanline_dot = 0;
            }
            _ => {
                panic!("PPU vblank 0-340 out of bounds - index:{} dot:{}", self.context.scanline_index, self.context.scanline_dot);
            }
        }

        cpu_pinout
    }
}

impl fmt::Display for Rp2c02 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CYC: {} V:{:#06X}  T:{:#06X} Index:{} Dot:{} - Pinout {} Pattern Shift {:#06X}",
        self.context.cycle, self.context.addr_reg.v, self.context.addr_reg.t, self.context.scanline_index, self.context.scanline_dot, self.pinout, self.pattern_queue[0])
    }
}