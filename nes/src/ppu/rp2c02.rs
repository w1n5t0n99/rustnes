use super::{Pinout, Context, IO};
use super::ppu_renderer::{Background, Sprites};
use super::ppu_registers::*;
use super::ppu_operations::*;
use crate::mappers::Mapper;

use std::fmt;

#[derive(Clone, Copy)]
enum PpuStatus {
    OpenTileIndex,
    ReadTileIndex,
    OpenAttribute,
    ReadAttribute,
    OpenBackgroundPattern,
    ReadBackgroundPattern,
    OpenSpritePattern,
    ReadSpritePattern,
    Idle,
    NonRender,
}

#[derive(Clone, Copy)]
pub struct Rp2c02 {
    context: Context,
    bg: Background,
    sp: Sprites,
    pinout: Pinout,   
    status: PpuStatus,
}

impl Rp2c02 {
    pub fn from_power_on() -> Rp2c02 {
        Rp2c02 {
            context: Context::new(),
            bg: Background::new(),
            sp: Sprites::new(),
            pinout: Pinout::new(),
            status: PpuStatus::Idle,
        }
    }

    pub fn enable_rendering(&mut self, enable_flag: bool) {
        if enable_flag == true {
            self.context.mask_reg.set(MaskRegister::SHOW_BACKGROUND, true);
            self.context.mask_reg.set(MaskRegister::SHOW_SPRITES, true);
            self.context.mask_reg.set(MaskRegister::LEFTMOST_8PXL_BACKGROUND, true);
            self.context.mask_reg.set(MaskRegister::LEFTMOST_8PXL_BACKGROUND, true);
        }
        else {
            self.context.mask_reg.set(MaskRegister::SHOW_BACKGROUND, false);
            self.context.mask_reg.set(MaskRegister::SHOW_SPRITES, false);
            self.context.mask_reg.set(MaskRegister::LEFTMOST_8PXL_BACKGROUND, false);
            self.context.mask_reg.set(MaskRegister::LEFTMOST_8PXL_BACKGROUND, false);    
        }
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
        self.context.control_reg.io_write(pinout.data);
        self.context.addr_reg.io_write_2000(pinout.data);
        pinout
    }

    pub fn write_ppumask(&mut self, pinout: mos::Pinout) -> mos::Pinout{
        self.context.io_db = pinout.data;
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
        self.context.addr_reg.io_write_2005(pinout.data);
        pinout
    }

    pub fn write_ppuaddr(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        self.context.addr_reg.io_write_2006(pinout.data);

        pinout
    }

    pub fn read_ppudata(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        self.context.io = IO::RDALE;
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // Reading palette updates latch with contents of nametable under palette address
                self.context.io_db = if  is_rendering(&mut self.context) { read_palette_rendering(&mut self.context, v) } else { read_palette_nonrender(&mut self.context, v) };
                pinout.data = self.context.io_db;
            }
            0x0000..=0x3EFF => {
                self.context.io_db = self.context.rd_buffer;
                pinout.data = self.context.rd_buffer;
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }

        pinout
    }

    pub fn write_ppudata(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.context.io_db = pinout.data;
        let v = self.context.addr_reg.vram_address();
        match v {
            0x3F00..=0x3FFF => {
                // TODO not sure if the underlying address is written to like reading does
                self.context.io = IO::WRPALETTE;
                write_palette(&mut self.context, v, pinout.data);
            }
            0x0000..=0x3EFF => {
                //println!("WRITE PPUDATA: {:#X} - {:#X}",  self.context.addr_reg.v, pinout.data);
                self.context.io = IO::WRALE;
                self.context.wr_buffer = self.context.io_db;
            }
            _ => {
                panic!("PPU 0x2007 address out of range");
            }
        }

        pinout
    }

    pub fn tick(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        // TODO add power on write block
        self.pinout.clear_ctrl();
        self.context.prev_scanline_index = self.context.scanline_index;
        self.context.prev_scanline_dot = self.context.scanline_dot;

        match self.context.scanline_index {
            261 => {
                cpu_pinout = if self.context.mask_reg.rendering_enabled() { self.scanline_prerender(mapper, cpu_pinout) } else { self.scanline_prerender_nonvisible(mapper, cpu_pinout) };
            }
            0..=239 => {
                cpu_pinout = if self.context.mask_reg.rendering_enabled() { self.scanline_render(fb, mapper, cpu_pinout) } else { self.scanline_render_nonvisible(fb, mapper, cpu_pinout) };
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
        // TODO see if sprite pixel overlaps
        
        read_palette_rendering(&mut self.context, pixel as u16) & self.context.monochrome_mask
    }

    fn scanline_prerender(&mut self, mapper: &mut dyn Mapper, cpu_pinout: mos::Pinout) -> mos::Pinout {
        let mut pinouts = (self.pinout, cpu_pinout);

        match self.context.scanline_dot {
            0 => {
                // Read first bytes of secondary OAM
                pinouts = render_idle_cycle(&mut self.context, mapper, pinouts);
                 self.status = PpuStatus::Idle;
            },
            1..=256 => {
                match self.context.scanline_dot & 0x07 {
                    1 => {
                        self.context.status_reg.set(StatusRegister::VBLANK_STARTED, false);
                        self.context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
                        self.context.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
                        // eval sprites odd
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        // eval sprites even
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        // eval sprites odd
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        // eval sprites even
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        // eval sprites odd
                        pinouts = open_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    6 => {
                        // eval sprites even
                        pinouts = read_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadBackgroundPattern;
                    }
                    7 => {
                        // eval sprites odd
                        pinouts = open_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    0 => {
                        // eval sprites even
                        pinouts = read_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadBackgroundPattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        pinouts = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    6 => {
                        pinouts = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
                    }
                    7 => {
                        pinouts = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    0 => {
                        pinouts = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        pinouts = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    6 => {
                        pinouts = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
                    }
                    7 => {
                        pinouts = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    0 => {
                        pinouts = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        pinouts = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    6 => {
                        pinouts = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
                    }
                    7 => {
                        pinouts = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    0 => {
                        pinouts = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        // Read first bytes of secondary OAM
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        // Read first bytes of secondary OAM
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        // Read first bytes of secondary OAM
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        // Read first bytes of secondary OAM
                        pinouts = open_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    6 => {
                        // Read first bytes of secondary OAM
                        pinouts = read_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadBackgroundPattern;
                    }
                    7 => {
                        // Read first bytes of secondary OAM
                        pinouts = open_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    0 => {
                        // Read first bytes of secondary OAM
                        pinouts = read_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadBackgroundPattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    338 => {
                        // Read first bytes of secondary OAM
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    339 => {
                        // Read first bytes of secondary OAM
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    340 => {
                        // Read first bytes of secondary OAM
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
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
            self.sp.reset_for_scanline(&mut self.context);
            self.context.scanline_index = 0;
            self.context.scanline_dot = if self.context.odd_frame { 1 } else { 0 };
        }
        else {
            self.context.scanline_dot += 1;
        }

        self.pinout = pinouts.0;
        pinouts.1
    }

    fn scanline_prerender_nonvisible(&mut self, mapper: &mut dyn Mapper, cpu_pinout: mos::Pinout) -> mos::Pinout {
        let mut pinouts = (self.pinout, cpu_pinout);

        pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
        self.status = PpuStatus::NonRender;

        if self.context.scanline_dot == 1 {
            self.context.status_reg.set(StatusRegister::VBLANK_STARTED, false);
            self.context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, false);
            self.context.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, false);
            self.context.scanline_dot += 1;
        }
        else if self.context.scanline_dot == 340 {
            self.sp.reset_for_scanline(&mut self.context);
            self.context.scanline_index = 0;
            self.context.scanline_dot = if self.context.odd_frame { 1 } else { 0 };
        }
        else {
            self.context.scanline_dot += 1;
        }

        self.pinout = pinouts.0;
        pinouts.1
    }

    fn scanline_render(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let mut pinouts = (self.pinout, cpu_pinout);
        
        match self.context.scanline_dot {
            0 => {
                // idle cycle
                pinouts = render_idle_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::Idle;
            }
            1..=256 => {
                // render pixel
                let index = ((self.context.scanline_dot - 1) + (self.context.scanline_index * 256)) as usize;
                fb[index] = self.select_pixel() as u16 | self.context.mask_reg.emphasis_mask();

                match self.context.scanline_dot & 0x07 {
                    1 => {
                        // eval sprites odd
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        // eval sprites even
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        // eval sprites odd
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        // eval sprites even
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        // eval sprites odd
                        pinouts = open_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    6 => {
                        // eval sprites even
                        pinouts = read_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    7 => {
                        // eval sprites odd
                        pinouts = open_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    0 => {
                        // eval sprites even
                        pinouts = read_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
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
                if self.context.scanline_dot == 257 {
                    self.context.addr_reg.update_x_scroll();
                }

                // update sprite registers
                match self.context.scanline_dot & 0x07 {
                    1 => {
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        pinouts = open_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    6 => {
                        pinouts = read_sprite_pattern0(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
                    }
                    7 => {
                        pinouts = open_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::OpenSpritePattern;
                    }
                    0 => {
                        pinouts = read_sprite_pattern1(&mut self.context, &mut self.sp, mapper, pinouts);
                        self.status = PpuStatus::ReadSpritePattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    2 => {
                        // eval sprites even
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    3 => {
                        // eval sprites odd
                        pinouts = open_background_attribute(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenAttribute;
                    }
                    4 => {
                        // eval sprites even
                        pinouts = read_background_attribute(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadAttribute;
                    }
                    5 => {
                        // eval sprites odd
                        pinouts = open_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    6 => {
                        // eval sprites even
                        pinouts = read_background_pattern0(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    7 => {
                        // eval sprites odd
                        pinouts = open_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
                    }
                    0 => {
                        // eval sprites even
                        pinouts = read_background_pattern1(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::OpenBackgroundPattern;
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
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    338 => {
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
                    }
                    339 => {
                        pinouts = open_tile_index(&mut self.context, mapper, pinouts);
                        self.status = PpuStatus::OpenTileIndex;
                    }
                    340 => {
                        pinouts = read_tile_index(&mut self.context, &mut self.bg, mapper, pinouts);
                        self.status = PpuStatus::ReadTileIndex;
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
            self.context.scanline_index += 1;
            self.context.scanline_dot = 0;
        }
        else {
            self.context.scanline_dot += 1;
        }

        self.pinout = pinouts.0;
        pinouts.1
    }

    fn scanline_render_nonvisible(&mut self, fb: &mut[u16], mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let mut pinouts = (self.pinout, cpu_pinout);
    
        match self.context.scanline_dot {
            0 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
            }
            1..=256 => {
                // render blank pixel
                let index = ((self.context.scanline_dot - 1) + (self.context.scanline_index * 256)) as usize;
                fb[index] = self.select_blank_pixel() as u16 | self.context.mask_reg.emphasis_mask();

                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
            }
            257..=340 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
            }
            _ => {
                panic!("PPU nonrender 0-340 out of bounds");
            }
        }

        if self.context.scanline_dot == 340 {
            self.context.scanline_index += 1;
            self.context.scanline_dot = 0;
        }
        else {
            self.context.scanline_dot += 1;
        }
    
        self.pinout = pinouts.0;
        pinouts.1
    }

    fn scanline_postrender(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let mut pinouts = (self.pinout, cpu_pinout);

        match self.context.scanline_dot {
            0..=339 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
                self.context.scanline_dot += 1;
            }
            340 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
                self.context.scanline_index += 1;
                self.context.scanline_dot = 0;
            }
            _ => {
                panic!("PPU postrender 0-340 out of bounds");
            }
        }

        self.pinout = pinouts.0;
        pinouts.1
    }

    fn scanline_vblank(&mut self, mapper: &mut dyn Mapper, mut cpu_pinout: mos::Pinout) -> mos::Pinout {
        let mut pinouts = (self.pinout, cpu_pinout);

        // TODO add support for multipe NMIs
        match self.context.scanline_dot {
            0 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
                self.context.scanline_dot += 1;
            }
            1 => {
                if self.context.scanline_index == 241 {
                    pinouts.1 =  enter_vblank(&mut self.context, pinouts.1);
                }

                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
                self.context.scanline_dot += 1;
            }
            2..=339 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
                self.context.scanline_dot += 1;
            }
            340 => {
                pinouts = nonrender_cycle(&mut self.context, mapper, pinouts);
                self.status = PpuStatus::NonRender;
                self.context.scanline_index += 1;
                self.context.scanline_dot = 0;
     
                if self.context.scanline_index == 260 { self.context.odd_frame = !self.context.odd_frame; }
            }
            _ => {
                panic!("PPU vblank 0-340 out of bounds - index:{} dot:{}", self.context.scanline_index, self.context.scanline_dot);
            }
        }

        self.pinout = pinouts.0;
        pinouts.1
    }
}

impl fmt::Display for Rp2c02 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let status_str = match self.status {
            PpuStatus::Idle => " Idle",
            PpuStatus::NonRender => "NonReneder",
            PpuStatus::OpenAttribute => "Open Attribute",
            PpuStatus::OpenBackgroundPattern => "Open Background Pattern",
            PpuStatus::OpenSpritePattern => "Open Sprite Pattern",
            PpuStatus::OpenTileIndex => "Open Tile Index",
            PpuStatus::ReadAttribute => "Read Attribute",
            PpuStatus::ReadBackgroundPattern => "Read background pattern",
            PpuStatus::ReadTileIndex => "Read Tile Index",
            PpuStatus::ReadSpritePattern => "Read Sprite Pattern",
        };

        write!(f, "CYC: {} V:{:#06X}  T:{:#06X} Index:{} Dot:{} - {} Pinout {} BG: {}",
        self.context.cycle, self.context.addr_reg.v, self.context.addr_reg.t, self.context.prev_scanline_index,
        self.context.prev_scanline_dot, status_str, self.pinout, self.bg)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::mappers::*;
    use crate::mappers::mapper_debug::MapperDebug;
    use mos::Pinout;

    #[test]
    fn test_ppudata_port() {
        let mut fb: Vec<u16> = vec![0; 256*240];
        let mut ppu = Rp2c02::from_power_on();
        let mut cpu_pinout = Pinout::new();
        let mut mapper = MapperDebug::new();

        mapper.set_nt_mirroring(NametableType::Horizontal);
        cpu_pinout.data = 0x23;
        cpu_pinout = ppu.write_ppuaddr(cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        cpu_pinout.data = 0xC0;
        cpu_pinout = ppu.write_ppuaddr(cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        cpu_pinout.data = 0x01;
        cpu_pinout = ppu.write_ppudata(cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);
        ppu.tick(&mut fb, &mut mapper, cpu_pinout);

        assert_eq!(0x01, mapper.peek_ppu(0x23C0));
    }

    #[test]
    fn test_palette_readwrite() {
        let mut ppu = Rp2c02::from_power_on();
        let mut cpu_pinout = Pinout::new();

        write_palette(&mut ppu.context, 0x3F00, 0xFF);
        let mut data = read_palette_nonrender(&mut ppu.context, 0x3F00);

        assert_eq!(0xFF, data);


        write_palette(&mut ppu.context, 0x3F10, 0x00);
        data = read_palette_nonrender(&mut ppu.context, 0x3F00);

        assert_eq!(0x00, data);
    }

}
