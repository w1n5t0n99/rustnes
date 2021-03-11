use super::{Pinout, Context};
use super::background::Background;
use super::sprites::Sprites;
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

use std::fmt;

const WRITE_BLOCK_CYCLES: u64 = 29658 * 3;

#[derive(Clone, Copy)]
pub struct Rp2c02 {
    context: Context,
    bg: Background,
    sp: Sprites,
    pinout: Pinout,   
    last_scanline_cycle: bool,
    last_frame_cycle: bool,
}

impl Rp2c02 {
    pub fn from_power_on() -> Rp2c02 {
        Rp2c02 {
            context: Context::new(),
            bg: Background::new(),
            sp: Sprites::new(),
            pinout: Pinout::new(),
            last_scanline_cycle: false,
            last_frame_cycle: false,
        }
    }

    pub fn is_odd_frame(&mut self) -> bool {
        self.context.odd_frame
    }

    pub fn is_end_of_frame(&mut self) -> bool {
        self.last_frame_cycle
    }

    pub fn is_end_of_scanline(&mut self) -> bool {
        self.last_scanline_cycle
    }

    pub fn reset_renderer(&mut self) {
        self.context.scanline_index = 261;
        self.context.scanline_dot = 0;
    }

    pub fn read_port(&self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data =  self.context.io_db;
        pinout
    }

    pub fn write_ppuctrl(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.prev_control_reg = self.context.control_reg;
        self.context.control_reg.io_write(pinout.data);
        self.context.addr_reg.io_write_2000(pinout.data);
        pinout
    }

    pub fn write_ppumask(&mut self, pinout: mos::Pinout) -> mos::Pinout{
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.mask_reg.io_write(pinout.data);

        self.context.monochrome_mask = if self.context.mask_reg.contains(MaskRegister::GREYSCALE) { 0x30 } else { 0xFF };
        pinout
    }

    pub fn read_ppustatus(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.context.read_2002_cycle = self.context.cycle;
        self.context.addr_reg.io_read_2002();
        pinout.data = self.context.status_reg.io_read(self.context.io_db);
        pinout
    }

    pub fn write_ppustatus(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // Writing to a read only port fills the io latch
        self.context.io_db = pinout.data;
        pinout
    }

    pub fn write_oamaddr(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        /*
        On the 2C02G, writes to OAMADDR reliably corrupt OAM.[3] This can then be worked around by writing all 256 bytes of OAM.
        It is also the case that if OAMADDR is not less than eight when rendering starts, the eight bytes starting at OAMADDR & 0xF8 are copied to the first eight bytes of OAM;
        it seems likely that this is related. On the Dendy, the latter bug is required for 2C02 compatibility.
        It is known that in the 2C03, 2C04, 2C05[4], and 2C07, OAMADDR works as intended. It is not known whether this bug is present in all revisions of the 2C02.
        */
        self.context.io_db = pinout.data;
        self.context.oam_addr_reg = pinout.data;
        pinout
    }

    pub fn read_oamdata(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match self.context.scanline_index {
            0..=239 | 261 if self.context.mask_reg.rendering_enabled() => {
                // Reading OAMDATA while the PPU is rendering will expose internal OAM accesses during sprite evaluation and loading
                if self.context.scanline_dot < 65 { 
                    self.context.io_db = 0xFF;
                }
                else {
                    self.context.io_db = self.context.oam_ram_primary[self.context.oam_addr_reg as usize];
                }
                pinout.data = self.context.io_db;
            }
            0..=239 | 261 => {
                // rendering disabled
                self.context.io_db = self.context.oam_ram_primary[self.context.oam_addr_reg as usize];
                pinout.data = self.context.io_db;
            }
            240..=260 => {
                // Reads during vertical or forced blanking return the value from OAM at that address but do not increment
                self.context.io_db = self.context.oam_ram_primary[self.context.oam_addr_reg as usize];
                pinout.data = self.context.io_db;
            }
            _ => {
                panic!("PPU Scanline out of range");
           }
        }

        pinout
    }

    pub fn write_oamdata(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        match self.context.scanline_index {
            0..=239 | 261 if self.context.mask_reg.rendering_enabled() => {
                // No oam write, but performs glitchy increment, only increments the high 6 bits
                // TODO possible implement glitchy increment
            }
            0..=239 | 261 => {
                self.context.oam_ram_primary[self.context.oam_addr_reg as usize] = pinout.data;
                self.context.oam_addr_reg = self.context.oam_addr_reg.wrapping_add(1);
            }
            240..=260 => {
                self.context.oam_ram_primary[self.context.oam_addr_reg as usize] = pinout.data;
                self.context.oam_addr_reg = self.context.oam_addr_reg.wrapping_add(1);
            }
            _ => {
                 panic!("PPU Scanline out of range");
            }
        }

        pinout
    }

    pub fn write_ppuscroll(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.addr_reg.io_write_2005(pinout.data);
        pinout
    }

    pub fn write_ppuaddr(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        if self.context.write_block {
            return pinout;
        }

        self.context.addr_reg.io_write_2006(pinout.data);
        pinout
    }

    pub fn read_ppudata(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // Reading palette updates latch with contents of nametable under palette address
                pinout.data = if  is_rendering(&mut self.context) { read_palette_rendering(&mut self.context, v) } else { read_palette_nonrender(&mut self.context, v) };
                // still need to update read buffer
                self.context.ppu_2007_rd_buffer = None;
            }
            0x0000..=0x3EFF => {
                let rdbuffer = self.context.ppu_2007_rd_buffer.take();
                pinout.data = rdbuffer.unwrap();
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }

        self.context.io_db = pinout.data;
        pinout
    }

    pub fn write_ppudata(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // TODO not sure if the underlying address is written to like reading does
                write_palette(&mut self.context, v, pinout.data);
                self.context.ppu_2007_wr_buffer = Some(pinout.data);
            }
            0x0000..=0x3EFF => {
                self.context.ppu_2007_wr_buffer = Some(pinout.data);
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }

        self.context.io_db = pinout.data;
        pinout
    }

    pub fn tick(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        self.last_frame_cycle = false;
        self.last_scanline_cycle = false;
        
        if self.context.cycle == WRITE_BLOCK_CYCLES {
            self.context.write_block = false;
        }
        
        self.context.prev_scanline_index = self.context.scanline_index;
        self.context.prev_scanline_dot = self.context.scanline_dot;

        match self.context.scanline_index {
            261 if self.context.mask_reg.rendering_enabled() => { self.scanline_prerender(mapper); }
            261 => { self.scanline_prerender_nonvisible(mapper) }
            0..=239 if self.context.mask_reg.rendering_enabled() => { self.scanline_render(fb, mapper); }
            0..=239 => { self.scanline_render_nonvisible(fb, mapper); }
            240 => { self.scanline_postrender(mapper); }
            241..=260 => { cpu_pinout = self.scanline_vblank(mapper, cpu_pinout); }
            _ => { panic!("Scanline index out of bounds"); }
        }

        self.context.cycle += 1;
        self.pinout = mapper.ppu_tick(self.pinout);
        cpu_pinout
    }

    pub fn select_blank_pixel(&mut self) -> u8 {
        let v = self.context.addr_reg.vram_address();
        if (v & 0x3F00) == 0x3F00 {
            read_palette_nonrender(&mut self.context, v) & self.context.monochrome_mask
        }
        else {
            read_palette_nonrender(&mut self.context, 0x00) & self.context.monochrome_mask
        }
    }

    // only call if rendering enbabled
    fn select_pixel(&mut self) -> u8 {
        // background pixel is default
        let mut pixel = self.bg.select_background_pixel(&mut self.context);
        pixel = self.sp.select_sprite_pixel(&mut self.context, pixel);
        
        read_palette_rendering(&mut self.context, pixel as u16) & self.context.monochrome_mask
    }

    fn scanline_prerender(&mut self, mapper: &mut dyn Mapper) {
        match self.context.scanline_dot {
            0 => {
                // Read first bytes of secondary OAM
                self.pinout = render_idle_cycle(&mut self.context, mapper, self.pinout);
            },
            1..=256 => {
                match self.context.scanline_dot & 0x07 {
                    1 => {
                        self.context.status_reg.set(StatusRegister::VBLANK_STARTED, false);
                        self.context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
                        self.context.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.pinout = open_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    6 => {
                        self.pinout = read_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    7 => {
                        self.pinout = open_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    0 => {
                        self.pinout = read_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                        self.bg.update_shift_registers_idle();
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
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.pinout = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    6 => {
                        self.pinout = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    7 => {
                        self.pinout = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    0 => {
                        self.pinout = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
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
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.pinout = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    6 => {
                        self.pinout = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    7 => {
                        self.pinout = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    0 => {
                        self.pinout = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
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
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.pinout = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    6 => {
                        self.pinout = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    7 => {
                        self.pinout = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    0 => {
                        self.pinout = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
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
                        // Read first bytes of secondary OAM
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        // Read first bytes of secondary OAM
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        // Read first bytes of secondary OAM
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        // Read first bytes of secondary OAM
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        // Read first bytes of secondary OAM
                        self.pinout = open_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    6 => {
                        // Read first bytes of secondary OAM
                        self.pinout = read_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    7 => {
                        // Read first bytes of secondary OAM
                        self.pinout = open_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    0 => {
                        // Read first bytes of secondary OAM
                        self.pinout = read_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                        self.bg.update_shift_registers_idle();
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
                        // Read first bytes of secondary OAM
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    338 => {
                        // Read first bytes of secondary OAM
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    339 => {
                        // Read first bytes of secondary OAM
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    340 => {
                        // Read first bytes of secondary OAM
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
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

        if self.context.scanline_dot == 340 {
            self.last_scanline_cycle = true;
            self.last_frame_cycle = true;
            self.context.scanline_index = 0;
            self.context.scanline_dot = if self.context.odd_frame { 1 } else { 0 };
        }
        else {
            self.context.scanline_dot += 1;
        }
    }

    fn scanline_prerender_nonvisible(&mut self, mapper: &mut dyn Mapper) {
        match self.context.scanline_dot {
            0 => {
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.status_reg.set(StatusRegister::VBLANK_STARTED, false);
                self.context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
                self.context.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
            }
            1..=340 => {
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
            }
            _ => {
                panic!("PPU prerender nonvisible 0-340 out of bounds");
            }
        }

        if self.context.scanline_dot == 340 {
            self.last_scanline_cycle = true;
            self.last_frame_cycle = true;
            self.context.scanline_index = 0;
            // render idle cycle is not skipped if rendering disabled
            self.context.scanline_dot = 0;
        }
        else {
            self.context.scanline_dot += 1;
        }
    }

    fn scanline_render(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper) {        
        match self.context.scanline_dot {
            0 => {
                // idle cycle
                self.pinout = render_idle_cycle(&mut self.context, mapper, self.pinout);
            }
            1..=64 => {
                 // render pixel
                 let index = ((self.context.scanline_dot - 1) + (self.context.scanline_index * 256)) as usize;
                 fb[index] = self.select_pixel() as u16 | self.context.mask_reg.emphasis_mask();

                 match self.context.scanline_dot & 0x07 {
                    1 => {
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.pinout = open_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    6 => {
                        self.pinout = read_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    7 => {
                        self.pinout = open_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    0 => {
                        self.pinout = read_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                        self.bg.update_shift_registers_render();
                    }
                    _ => {
                        panic!("ppu 2-65 out of bounds");
                    }
                }
            }
            65..=256 => {
                if self.context.scanline_dot == 65 {
                    self.sp.clear_secondary_oam(self.context.oam_addr_reg);
                }

                // render pixel
                let index = ((self.context.scanline_dot - 1) + (self.context.scanline_index * 256)) as usize;
                fb[index] = self.select_pixel() as u16 | self.context.mask_reg.emphasis_mask();

                match self.context.scanline_dot & 0x07 {
                    1 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = open_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    6 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = read_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    7 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = open_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    0 => {
                        self.sp.evaluate(&mut self.context);
                        self.pinout = read_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                        self.bg.update_shift_registers_render();
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
                self.context.oam_addr_reg = 0;

                if self.context.scanline_dot == 257 {
                    self.context.addr_reg.update_x_scroll();
                }

                // update sprite registers
                match self.context.scanline_dot & 0x07 {
                    1 => {
                        self.sp.fetch_sprite_data(&mut self.context);
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        self.pinout = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    6 => {
                        self.pinout = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    7 => {
                        self.pinout = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
                    }
                    0 => {
                        self.pinout = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, self.pinout);
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
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    2 => {
                        // eval sprites even
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    3 => {
                        // eval sprites odd
                        self.pinout = open_background_attribute(&mut self.context, mapper, self.pinout);
                    }
                    4 => {
                        // eval sprites even
                        self.pinout = read_background_attribute(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    5 => {
                        // eval sprites odd
                        self.pinout = open_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    6 => {
                        // eval sprites even
                        self.pinout = read_background_pattern0(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    7 => {
                        // eval sprites odd
                        self.pinout = open_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    0 => {
                        // eval sprites even
                        self.pinout = read_background_pattern1(&mut self.context, &mut self.bg, mapper, self.pinout);
                        self.bg.update_shift_registers_idle();
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
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    338 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
                    }
                    339 => {
                        self.pinout = open_tile_index(&mut self.context, mapper, self.pinout);
                    }
                    340 => {
                        self.pinout = read_tile_index(&mut self.context, &mut self.bg, mapper, self.pinout);
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

        if self.context.scanline_dot == 340 {
            self.last_scanline_cycle = true;
            self.context.scanline_index += 1;
            self.context.scanline_dot = 0;
        }
        else {
            self.context.scanline_dot += 1;
        }
    }

    fn scanline_render_nonvisible(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper) {
        match self.context.scanline_dot {
            0 => {
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
            }
            1..=256 => {
                // render blank pixel
                let index = ((self.context.scanline_dot - 1) + (self.context.scanline_index * 256)) as usize;
                fb[index] = self.select_blank_pixel() as u16 | self.context.mask_reg.emphasis_mask();

                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
            }
            257..=340 => {
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
            }
            _ => {
                panic!("PPU nonrender 0-340 out of bounds");
            }
        }

        if self.context.scanline_dot == 340 {
            self.last_scanline_cycle = true;
            self.context.scanline_index += 1;
            self.context.scanline_dot = 0;
        }
        else {
            self.context.scanline_dot += 1;
        }
    }

    fn scanline_postrender(&mut self, mapper: &mut dyn Mapper) {
        match self.context.scanline_dot {
            0..=339 => {
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.scanline_dot += 1;
            }
            340 => {
                self.last_scanline_cycle = true;
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.scanline_index += 1;
                self.context.scanline_dot = 0;
            }
            _ => {
                panic!("PPU postrender 0-340 out of bounds");
            }
        }
    }

    fn scanline_vblank(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // TODO add support for multipe NMIs
        match self.context.scanline_dot {
            0 => {
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.scanline_dot += 1;
            }
            1 => {
                if self.context.scanline_index == 241 {
                    cpu_pinout = enter_vblank(&mut self.context, cpu_pinout);
                }

                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.scanline_dot += 1;
            }
            2..=339 => {
                cpu_pinout = vblank_nmi_update(&mut self.context, cpu_pinout);
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.scanline_dot += 1;
            }
            340 => {
                self.last_scanline_cycle = true;
                cpu_pinout = vblank_nmi_update(&mut self.context, cpu_pinout);
                self.pinout = nonrender_cycle(&mut self.context, mapper, self.pinout);
                self.context.scanline_index += 1;
                self.context.scanline_dot = 0;
     
                if self.context.scanline_index == 260 { self.context.odd_frame = !self.context.odd_frame; }
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
        write!(f, "CYC: {} V:{:#06X}  T:{:#06X} Index:{} Dot:{} - Pinout {} BG {} SPR: {}",
        self.context.cycle, self.context.addr_reg.v, self.context.addr_reg.t, self.context.prev_scanline_index,
        self.context.prev_scanline_dot, self.pinout, self.bg, self.sp)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::mappers::*;
    use mos::Pinout;

    #[test]
    fn test_ppudata_port() {
        let mut fb: Vec<u16> = vec![0; 256*240];
        let mut ppu = Rp2c02::from_power_on();
        let mut cpu_pinout = Pinout::new();
        
    }

    #[test]
    fn test_palette_readwrite() {
        let mut ppu = Rp2c02::from_power_on();

        write_palette(&mut ppu.context, 0x3F00, 0xFF);
        let mut data = read_palette_nonrender(&mut ppu.context, 0x3F00);

        assert_eq!(0xFF, data);


        write_palette(&mut ppu.context, 0x3F10, 0x00);
        data = read_palette_nonrender(&mut ppu.context, 0x3F00);

        assert_eq!(0x00, data);
    }

}
