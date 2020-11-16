use super::{Pinout, Context, IO};
use super::ppu_registers::*;

use std::fmt;

const PATTERN0_INDEX: usize = 0;
const PATTERN0_OFFSET: u16 = 0;
const PATTERN1_INDEX: usize = 1;
const PATTERN1_OFFSET: u16 = 8;
const SPRITE_8X_VALUE: u16 = 8;
const SPRITE_16X_VALUE: u16 = 16;
const SPRITE_8X_FLIPMASK: u16 = 0b00000111;
const SPRITE_16X_FLIPMASK: u16 = 0b00001111;


const REVERSE_BITS: [u8; 256] = [
	0x00, 0x80, 0x40, 0xc0, 0x20, 0xa0, 0x60, 0xe0, 0x10, 0x90, 0x50, 0xd0, 0x30, 0xb0, 0x70, 0xf0,
	0x08, 0x88, 0x48, 0xc8, 0x28, 0xa8, 0x68, 0xe8, 0x18, 0x98, 0x58, 0xd8, 0x38, 0xb8, 0x78, 0xf8,
	0x04, 0x84, 0x44, 0xc4, 0x24, 0xa4, 0x64, 0xe4, 0x14, 0x94, 0x54, 0xd4, 0x34, 0xb4, 0x74, 0xf4,
	0x0c, 0x8c, 0x4c, 0xcc, 0x2c, 0xac, 0x6c, 0xec, 0x1c, 0x9c, 0x5c, 0xdc, 0x3c, 0xbc, 0x7c, 0xfc,
	0x02, 0x82, 0x42, 0xc2, 0x22, 0xa2, 0x62, 0xe2, 0x12, 0x92, 0x52, 0xd2, 0x32, 0xb2, 0x72, 0xf2,
	0x0a, 0x8a, 0x4a, 0xca, 0x2a, 0xaa, 0x6a, 0xea, 0x1a, 0x9a, 0x5a, 0xda, 0x3a, 0xba, 0x7a, 0xfa,
	0x06, 0x86, 0x46, 0xc6, 0x26, 0xa6, 0x66, 0xe6, 0x16, 0x96, 0x56, 0xd6, 0x36, 0xb6, 0x76, 0xf6,
	0x0e, 0x8e, 0x4e, 0xce, 0x2e, 0xae, 0x6e, 0xee, 0x1e, 0x9e, 0x5e, 0xde, 0x3e, 0xbe, 0x7e, 0xfe,
	0x01, 0x81, 0x41, 0xc1, 0x21, 0xa1, 0x61, 0xe1, 0x11, 0x91, 0x51, 0xd1, 0x31, 0xb1, 0x71, 0xf1,
	0x09, 0x89, 0x49, 0xc9, 0x29, 0xa9, 0x69, 0xe9, 0x19, 0x99, 0x59, 0xd9, 0x39, 0xb9, 0x79, 0xf9,
	0x05, 0x85, 0x45, 0xc5, 0x25, 0xa5, 0x65, 0xe5, 0x15, 0x95, 0x55, 0xd5, 0x35, 0xb5, 0x75, 0xf5,
	0x0d, 0x8d, 0x4d, 0xcd, 0x2d, 0xad, 0x6d, 0xed, 0x1d, 0x9d, 0x5d, 0xdd, 0x3d, 0xbd, 0x7d, 0xfd,
	0x03, 0x83, 0x43, 0xc3, 0x23, 0xa3, 0x63, 0xe3, 0x13, 0x93, 0x53, 0xd3, 0x33, 0xb3, 0x73, 0xf3,
	0x0b, 0x8b, 0x4b, 0xcb, 0x2b, 0xab, 0x6b, 0xeb, 0x1b, 0x9b, 0x5b, 0xdb, 0x3b, 0xbb, 0x7b, 0xfb,
	0x07, 0x87, 0x47, 0xc7, 0x27, 0xa7, 0x67, 0xe7, 0x17, 0x97, 0x57, 0xd7, 0x37, 0xb7, 0x77, 0xf7,
	0x0f, 0x8f, 0x4f, 0xcf, 0x2f, 0xaf, 0x6f, 0xef, 0x1f, 0x9f, 0x5f, 0xdf, 0x3f, 0xbf, 0x7f, 0xff ];

//===============================================
// Background
//===============================================
#[derive(Debug, Clone, Copy)]
pub struct Background {
    pattern_queue: [u16; 2],
    attribute_queue: [u16; 2],
    pub next_pattern: [u8; 2],
    pub next_tile_index: u16,
    pub next_attribute: u8,
}

impl Background {
    pub fn new() -> Self {
        Background {
            pattern_queue: [0; 2],
            attribute_queue: [0; 2],
            next_pattern: [0; 2],
            next_attribute: 0,
            next_tile_index: 0,
        }
    }

    pub fn select_background_pixel(&mut self, ppu: &mut Context) -> u8 {
        let mut pixel: u8 = 0;
        if (ppu.mask_reg.contains(MaskRegister::LEFTMOST_8PXL_BACKGROUND) | (ppu.scanline_dot >= 8)) && ppu.mask_reg.contains(MaskRegister::SHOW_BACKGROUND) {
            let mask: u16 = 0x8000 >> ppu.addr_reg.x;

            pixel = (((self.pattern_queue[0] & mask) >> (15 - ppu.addr_reg.x)) |
            ((self.pattern_queue[1] & mask) >> (14 - ppu.addr_reg.x)) |
            ((self.attribute_queue[0] & mask) >> (13 - ppu.addr_reg.x)) |
            ((self.attribute_queue[1] & mask) >> (12 - ppu.addr_reg.x)) &
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

    pub fn update_shift_registers_render(&mut self) {
        self.pattern_queue[0] |= self.next_pattern[0] as u16;
	    self.pattern_queue[1] |= self.next_pattern[1] as u16;
	    self.attribute_queue[0] |= (((self.next_attribute >> 0) & 0x01) * 0xff) as u16; // we multiply here to "replicate" this bit 8 times (it is used for a whole tile)
	    self.attribute_queue[1] |= (((self.next_attribute >> 1) & 0x01) * 0xff) as u16; // we multiply here to "replicate" this bit 8 times (it is used for a whole tile)
    }

    pub fn update_shift_registers_idle(&mut self) {
        self.pattern_queue[0] <<= 8;
        self.pattern_queue[1] <<= 8;
        self.attribute_queue[0] <<= 8;
        self.attribute_queue[1] <<= 8;
    
        self.update_shift_registers_render();
    }
}

impl fmt::Display for Background {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "Shift 0:{:#06X}  Shift 1:{:#06X} Attribute 0:{:#04X} Attribute 1:{:#04X}",
        self.pattern_queue[0], self.pattern_queue[1], self.attribute_queue[0], self.attribute_queue[1])
    }
}

//===============================================
// Sprites
//===============================================
#[derive(Debug, Clone, Copy)]
enum EvalState {
    YRead,
    YWrite,
    IndexRead,
    IndexWrite,
    AttributeRead,
    AttributeWrite,
    XRead,
    XWrite,
    POAMOverflow,
    SOAMOverflow,
}

bitflags! {
    // 76543210
    // ||||||||
    // ||||||++- Palette (4 to 7) of sprite
    // |||+++--- Unimplemented
    // ||+------ Priority (0: in front of background; 1: behind background)
    // |+------- Flip sprite horizontally
    // +-------- Flip sprite vertically
    struct SpriteAttrib: u8 {
        const PALETTE0                   = 0b00000001;
        const PALETTE1                  = 0b00000010;
        const UNUSED0                   = 0b00000100;
        const UNUSED1                   = 0b00001000;
        const UNUSED2                   = 0b00010000;
        const BEHIND_BACKGROUND         = 0b00100000;
        const HFLIP                     = 0b01000000;
        const VFLIP                     = 0b10000000; 
    }
}

impl SpriteAttrib {
    pub fn new() -> Self {
        SpriteAttrib::from_bits_truncate(0xFF)
    }

    pub fn pallete_index(&mut self) -> u8 {
        match (self.contains(SpriteAttrib::PALETTE1), self.contains(SpriteAttrib::PALETTE0)) {
            (false, false) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (true, true) => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SpriteData {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_index: u8,
    pub attribute: SpriteAttrib,
    pub active: bool,
    pub pattern: [u8; 2],
}

impl SpriteData {
    pub fn new() -> Self {
        SpriteData {
            y_pos: 0xFF,
            x_pos: 0xFF,
            tile_index: 0xFF,
            attribute: SpriteAttrib::new(),
            active: false,
            pattern: [0xFF; 2],
        }
    }

    pub fn clear(&mut self) {
        self.y_pos = 0xFF;
        self.x_pos = 0xFF;
        self.tile_index = 0xFF;
        self.attribute = SpriteAttrib::new();
        self.active = false;
        self.pattern[0] = 0xFF;
        self.pattern[1] = 0xFF;
    }
}

#[derive(Debug, Clone, Copy)]
struct OamIndex {
    pub index: u8,
}

impl OamIndex {
    pub fn new() -> Self {
        OamIndex {
            index: 0,
        }
    }

    pub fn n(&self) -> u8 {
        (self.index & 0xFC) >> 2
    }

    pub fn m(&self) -> u8 {
        self.index & 0x3
    }

    pub fn increment(&mut self) {
        self.index = self.index.wrapping_add(4);
    }

    pub fn increment_n(&mut self) {
        self.index = (self.index & 0x03) | (((self.index & 0xFC) + 4) & 0xFC);
    }

    pub fn increment_m(&mut self) {
        self.index = (self.index & 0xFC) | (((self.index & 0x03) + 1) & 0x03);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sprites {
    soam: [SpriteData; 8],
    soam_index: u8,
    poam_index: OamIndex,
    cur_sprite_index: u16,
    oam_data: u8,
    left_most_x: u8,
    eval_state: EvalState,
}

impl Sprites {
    pub fn new() -> Self {
        Sprites {
            soam: [SpriteData::new(); 8],
            soam_index: 0,
            poam_index: OamIndex::new(),
            cur_sprite_index: 0,
            oam_data: 0,
            left_most_x: 0xFF,
            eval_state: EvalState::YRead,
        }
    }

    pub fn reset_for_scanline(&mut self, ppu: &mut Context) {
        ppu.oam_addr_reg = 0;
        self.soam_index = 0;
        self.poam_index = OamIndex::new();
        self.cur_sprite_index = 0;
        self.left_most_x = 0xFF;
        self.eval_state = EvalState::YRead;

        for x in self.soam.iter_mut() {
            x.clear();
        }
    }

    pub fn sprite_pattern_address_low(&mut self, ppu: &mut Context, sprite_index: u16, sprite_line: u16) -> u16 {
        if ppu.control_reg.contains(ControlRegister::SPRITE_SIZE) {
            // 8x16
            (((sprite_index & 1) << 12) | ((sprite_index & 0xfe) << 4) | PATTERN0_OFFSET | (sprite_line & 7) | ((sprite_line & 0x08) << 1)) & 0xffff
        }
        else {
            // 8x8
            (ppu.control_reg.sprite_table_address()| (sprite_index << 4) | PATTERN0_OFFSET | sprite_line) & 0xffff
        }
    }

    pub fn sprite_pattern_address_high(&mut self, ppu: &mut Context, sprite_index: u16, sprite_line: u16) -> u16 {
        if ppu.control_reg.contains(ControlRegister::SPRITE_SIZE) {
            // 8x16
            (((sprite_index & 1) << 12) | ((sprite_index & 0xfe) << 4) | PATTERN1_OFFSET | (sprite_line & 7) | ((sprite_line & 0x08) << 1)) & 0xffff
        }
        else {
            // 8x8
            (ppu.control_reg.sprite_table_address()| (sprite_index << 4) | PATTERN1_OFFSET | sprite_line) & 0xffff
        }
    }

    fn sprite_in_range(&mut self, ppu: &mut Context, size: u16) -> bool {
        // Y was read into oam data in the previous cycle
        let sprite_line = (ppu.scanline_index + 1).saturating_sub(self.oam_data as u16);
        sprite_line < size
    }

    fn increment_poam_buggy(&mut self, ppu: &mut Context) {
        // 3b. If the value is not in range, increment n AND m (without carry). If n overflows to 0, go to 4; otherwise go to 3
		self.poam_index = (self.poam_index & 0x03) | (((self.poam_index & 0xfc) + 4) & 0xfc);
        self.poam_index = (self.poam_index & 0xfc) | (((self.poam_index & 0x03) + 1) & 0x03); 
        ppu.oam_addr_reg = self.poam_index as u8;
        if self.poam_index >= 256 {
            self.poam_index = 0;
            // poam index overflowed
            self.eval_state = EvalState::POAMOverflow;
        }
    }

    fn increment_poam(&mut self, ppu: &mut Context) {
        self.poam_index = self.poam_index.wrapping_add(1);
        ppu.oam_addr_reg = self.poam_index as u8;
        if self.poam_index >= 256 {
            self.poam_index = 0;
            // poam index overflowed
            self.eval_state = EvalState::POAMOverflow;
        }
    }

    fn increment_soam(&mut self) {
        self.soam_index = self.soam_index.wrapping_add(1);
        if self.soam_index >= 8 {
            self.soam_index = 0;
        }
    }

    pub fn evaluate(&mut self, ppu: &mut Context) {
        if ppu.scanline_dot < 65 {
            // ppu is zeroing secondary oam
            return;
        }

        let size = if ppu.control_reg.contains(ControlRegister::SPRITE_SIZE) { 16 } else { 8 };

        match self.eval_state {
            EvalState::YRead => {
                self.oam_data = ppu.oam_ram_primary[self.poam_index.index as usize];
                ppu.oam_addr_reg = self.poam_index.index;
                self.eval_state = EvalState::YWrite;
            }
            EvalState::YWrite => {
                if self.sprite_in_range(ppu, size) {
                    let sprite_line =  (ppu.scanline_index + 1).saturating_sub(self.oam_data as u16);
                    self.soam[self.soam_index as usize].y_pos = sprite_line as u8;
                    self.eval_state = EvalState::IndexRead;
                }
                else {
                    self.poam_index.increment();
                    if self.poam_index.index == 0 { self.eval_state = EvalState::POAMOverflow; }
                }
            }
            EvalState::IndexRead => {
                self.oam_data = ppu.oam_ram_primary[(self.poam_index.index + 1) as usize];
                self.eval_state = EvalState::IndexWrite;
            }
            EvalState::IndexWrite => {
                self.soam[self.soam_index as usize].tile_index = self.oam_data;
                self.eval_state = EvalState::AttributeRead;
            }
            EvalState::AttributeRead => {
                self.oam_data = ppu.oam_ram_primary[(self.poam_index.index + 2) as usize];
                self.eval_state = EvalState::AttributeWrite;
            }
            EvalState::AttributeWrite => {
                self.soam[self.soam_index as usize].attribute = SpriteAttrib::from_bits_truncate(self.oam_data);
                self.eval_state = EvalState::XRead;
            }
            EvalState::XRead => {
                self.oam_data = ppu.oam_ram_primary[(self.poam_index.index + 3) as usize];
                self.eval_state = EvalState::XRead;
            }
            EvalState::XWrite => {
                self.soam[self.soam_index as usize].x_pos = self.oam_data;
                self.left_most_x = self.left_most_x.min(self.oam_data);
                self.eval_state = EvalState::YRead;
                // increment secondary check if overflows
                self.increment_soam();
                if self.soam_index == 0 { self.eval_state = EvalState::SOAMOverflow; }
                //increment primary checking if overflow
                self.poam_index.increment();
                if self.poam_index.index == 0 { self.eval_state = EvalState::POAMOverflow; }
            }
            EvalState::SOAMOverflow => {
                // we already found 8 sprites check if we need to set sprite overlow flag
                self.oam_data = ppu.oam_ram_primary[self.poam_index];
                if self.sprite_in_range(ppu, size) {
                    ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, true);
                    self.increment_poam(ppu);
                }
                else {
                    self.increment_poam_buggy(ppu);
                }
            }
            EvalState::POAMOverflow => {
                self.increment_poam_sprite(ppu);
            }
        }
    }

    pub fn select_sprite_pixel(&mut self, ppu: &mut Context) -> u8 {

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_background_rendering() {
        let mut ppu_context = Context::new();
        ppu_context.mask_reg.set(MaskRegister::SHOW_BACKGROUND, true);
        ppu_context.mask_reg.set(MaskRegister::SHOW_SPRITES, true);
        ppu_context.scanline_dot = 10;

        let mut bg = Background::new();
        bg.next_pattern[0] = 0xFF;
        bg.next_pattern[1] = 0xFF;
        bg.update_shift_registers_idle();

        assert_eq!(bg.pattern_queue[0], 0xFF);
        assert_eq!(bg.pattern_queue[1], 0xFF);

        bg.next_pattern[0] = 0x3C;
        bg.next_pattern[1] = 0x3C;
        bg.next_attribute = 0xFF;
        bg.update_shift_registers_idle();

        assert_eq!(bg.pattern_queue[0], 0xFF3C);
        assert_eq!(bg.pattern_queue[1], 0xFF3C);

        let mut pixel = bg.select_background_pixel(&mut ppu_context);
        //println!("\npattern 0 - {:#0b} pattern 1- {:#0b} attrib 0 - {:#0b} attrib 1 -  {:#0b} pixel {:#0b} ", 
        //    bg.pattern_queue[0], bg.pattern_queue[1], bg.attribute_queue[0], bg.attribute_queue[1], pixel);
        assert_eq!(pixel, 0x03);

        // test the registers are shifting correctly
        pixel = bg.select_background_pixel(&mut ppu_context);
        pixel = bg.select_background_pixel(&mut ppu_context);
        pixel = bg.select_background_pixel(&mut ppu_context);
        pixel = bg.select_background_pixel(&mut ppu_context);
        pixel = bg.select_background_pixel(&mut ppu_context);
        pixel = bg.select_background_pixel(&mut ppu_context);
        pixel = bg.select_background_pixel(&mut ppu_context);

        bg.next_pattern[0] = 0x0;
        bg.next_pattern[1] = 0x0;
        bg.next_attribute = 0x0;
        bg.update_shift_registers_render();

        pixel = bg.select_background_pixel(&mut ppu_context);
        assert_eq!(pixel, 0x0C);
        pixel = bg.select_background_pixel(&mut ppu_context);
        assert_eq!(pixel, 0x0C);
        pixel = bg.select_background_pixel(&mut ppu_context);
        assert_eq!(pixel, 0x0F);
    }

}