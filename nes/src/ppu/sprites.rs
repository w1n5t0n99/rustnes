use super::{Pinout, Context, IO};
use super::ppu_registers::*;

use std::fmt;

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

const PATTERN0_INDEX: usize = 0;
const PATTERN0_OFFSET: u16 = 0;
const PATTERN1_INDEX: usize = 1;
const PATTERN1_OFFSET: u16 = 8;
const SPRITE_8X_SIZE: u8 = 8;
const SPRITE_16X_SIZE: u8 = 16;
const SPRITE_8X_FLIPMASK: u8 = 0b00000111;
const SPRITE_16X_FLIPMASK: u8 = 0b00001111;

#[derive(Debug, Clone, Copy)]
struct OamIndex {
    index: u8,
}

impl OamIndex {
    pub fn new() -> Self {
        OamIndex {
            index: 0,
        }
    }

    pub fn from_oamaddr(addr: u8) -> Self {
        OamIndex {
            index: addr,
        }
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn n(&self) -> u8 {
        (self.index & 0xFC) >> 2
    }

    pub fn m(&self) -> u8 {
        self.index & 0x3
    }

    pub fn increment(&mut self) {
        self.index = self.index.wrapping_add(1);
    }

    pub fn increment_n(&mut self) {
        self.index = (self.index & 0x03) | (self.index & 0xFC).wrapping_add(4);
    }

    pub fn increment_m(&mut self) {
        self.index = (self.index & 0xFC) | (self.index & 0x03).wrapping_add(1);
    }
}

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
    OamSearchCompleted,
    MaxSpritesFound,
}

bitflags! {
    // 76543210
    // ||||||||
    // ||||||++- Palette (4 to 7) of sprite
    // |||+++--- Unimplemented
    // ||+------ Priority (0: in front of background; 1: behind background)
    // |+------- Flip sprite horizontally
    // +-------- Flip sprite vertically
    pub struct SpriteAttribute: u8 {
        const PALETTE0                  = 0b00000001;
        const PALETTE1                  = 0b00000010;
        const UNUSED0                   = 0b00000100;
        const UNUSED1                   = 0b00001000;
        const UNUSED2                   = 0b00010000;
        const BEHIND_BACKGROUND         = 0b00100000;
        const HFLIP                     = 0b01000000;
        const VFLIP                     = 0b10000000; 
    }
}

impl SpriteAttribute {
    pub fn new() -> Self {
        SpriteAttribute::from_bits_truncate(0xFF)
    }

    pub fn pallete_index(&self) -> u8 {
        match (self.contains(SpriteAttribute::PALETTE1), self.contains(SpriteAttribute::PALETTE0)) {
            (false, false) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (true, true) => 3,
        }
    }

    #[inline(always)]
    pub fn hflip(&self) -> bool {
        self.contains(SpriteAttribute::HFLIP)
    }

    #[inline(always)]
    pub fn vflip(&self) -> bool {
        self.contains(SpriteAttribute::VFLIP)
    }

    #[inline(always)]
    pub fn infront_of_background(&self) -> bool {
        !self.contains(SpriteAttribute::BEHIND_BACKGROUND)
    }

    #[inline(always)]
    pub fn behind_background(&self) -> bool {
        self.contains(SpriteAttribute::BEHIND_BACKGROUND)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpriteData {
    pub y_pos: u8,
    pub x_pos: u8,
    pub tile_index: u8,
    pub attribute: SpriteAttribute,
    pub active: bool,
    pub zero: bool,
    pub pattern: [u8; 2],
}

impl SpriteData {
    pub fn new() -> Self {
        SpriteData {
            y_pos: 0xFF,
            x_pos: 0xFF,
            tile_index: 0xFF,
            attribute: SpriteAttribute::new(),
            active: false,
            zero: false,
            pattern: [0xFF; 2],
        }
    }

    pub fn clear(&mut self) {
        self.y_pos = 0xFF;
        self.x_pos = 0xFF;
        self.tile_index = 0xFF;
        self.attribute = SpriteAttribute::new();
        self.active = false;
        self.zero = false;
        self.pattern[0] = 0xFF;
        self.pattern[1] = 0xFF;
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Sprites {
    next_sprite_data: [SpriteData; 8],
    render_sprite_data: [SpriteData; 8],
    next_sprite_index: u8,
    render_sprite_index: u8,
    next_pattern_address: u16,
    poam_index: OamIndex,
    oam_db: u8,
    left_most_x: u8,
    sprite_0_hit: bool,
    eval_state: EvalState,
}


impl Sprites {
    pub fn new() -> Self {
        Sprites {
            next_sprite_data: [SpriteData::new(); 8],
            render_sprite_data: [SpriteData::new(); 8],
            next_sprite_index: 0,
            render_sprite_index: 0,
            next_pattern_address: 0,
            poam_index: OamIndex::new(),
            oam_db: 0,
            left_most_x: 0xFF,
            sprite_0_hit: false,
            eval_state: EvalState::YRead,
        }
    }

    pub fn reset_for_eval(&mut self, oam_addr: u8) {
        self.next_sprite_index = 0;
        self.render_sprite_index = 0;
        self.next_pattern_address = 0;
        self.oam_db = 0;
        self.poam_index = OamIndex::from_oamaddr(oam_addr);
        self.left_most_x = 0xFF;
        self.sprite_0_hit = false;
        self.eval_state = EvalState::YRead;

        for x in self.next_sprite_data.iter_mut() {
            x.active = false;
        }
    }

    pub fn reset_for_frame(&mut self) {
        self.sprite_0_hit = false;
        self.render_sprite_index = 0;
        for x in self.render_sprite_data.iter_mut() {
            x.active = false;
        }
    }

    fn sprite_in_range(&mut self, ppu: &mut Context, sprite_size: u16) -> bool {
        // Sprite eval happens for the next scanline
        let sprite_line = (ppu.scanline_index + 1).saturating_sub(self.oam_db as u16);
        sprite_line < sprite_size
    }

    pub fn evaluate(&mut self, ppu: &mut Context) {
  
        ppu.oam_addr_reg = self.poam_index.index();
        let sprite_size = ppu.control_reg.sprite_size() as u16;

        match self.eval_state {
            EvalState::YRead => {
                self.oam_db = ppu.oam_ram_primary[self.poam_index.index() as usize];
                self.eval_state = EvalState::YWrite;
            }
            EvalState::YWrite => {
                let sprite_line =  (ppu.scanline_index + 1).saturating_sub(self.oam_db as u16);
                self.next_sprite_data[self.next_sprite_index as usize].y_pos = sprite_line as u8;

                if self.sprite_in_range(ppu, sprite_size) {
                    self.next_sprite_data[self.next_sprite_index as usize].active = true;
                    // check if zero sprite
                    if self.next_sprite_index == 0 {
                        self.next_sprite_data[self.next_sprite_index as usize].zero = true;
                    }

                    self.eval_state = EvalState::IndexRead;
                }
                else {
                    self.poam_index.increment_n();
                    if self.poam_index.index() == 0 { self.eval_state = EvalState::OamSearchCompleted; }
                }
            }
            EvalState::IndexRead => {
                self.oam_db = ppu.oam_ram_primary[(self.poam_index.index + 1) as usize];
                self.eval_state = EvalState::IndexWrite;
            }
            EvalState::IndexWrite => {
                self.next_sprite_data[self.next_sprite_index as usize].tile_index = self.oam_db;
                self.eval_state = EvalState::AttributeRead;
            }
            EvalState::AttributeRead => {
                self.oam_db = ppu.oam_ram_primary[(self.poam_index.index + 2) as usize];
                self.eval_state = EvalState::AttributeWrite;
            }
            EvalState::AttributeWrite => {
                self.next_sprite_data[self.next_sprite_index as usize].attribute = SpriteAttribute::from_bits_truncate(self.oam_db);
                self.eval_state = EvalState::XRead;
            }
            EvalState::XRead => {
                self.oam_db = ppu.oam_ram_primary[(self.poam_index.index + 3) as usize];
                self.eval_state = EvalState::XRead;
            }
            EvalState::XWrite => {
                self.next_sprite_data[self.next_sprite_index as usize].x_pos = self.oam_db;
                self.left_most_x = self.left_most_x.min(self.oam_db);
                self.eval_state = EvalState::YRead;
                // check if 8 sprites have been found
                self.next_sprite_index += 1;
                if self.next_sprite_index >= 8 { self.eval_state = EvalState::MaxSpritesFound; }
                //increment primary checking if overflow
                self.poam_index.increment_n();
                if self.poam_index.index() == 0 { self.eval_state = EvalState::OamSearchCompleted; }
            }
            EvalState::MaxSpritesFound => {
                // we already found 8 sprites check if we need to set sprite overlow flag
                self.oam_db = ppu.oam_ram_primary[self.poam_index.m() as usize];
                if self.sprite_in_range(ppu, sprite_size) {
                    ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, true);
                    self.poam_index.increment();
                    if self.poam_index.index == 0 { self.eval_state = EvalState::OamSearchCompleted; }
                }
                else {
                    self.poam_index.increment_m();
                    self.poam_index.increment_n();
                    if self.poam_index.n() == 0 { self.eval_state = EvalState::OamSearchCompleted; }
                }
            }
            EvalState::OamSearchCompleted => {
                self.poam_index.increment_n();
            }
        }
    }

    pub fn fetch_y(&mut self, ppu: &mut Context) {
        self.render_sprite_data[self.render_sprite_index as usize].y_pos = self.next_sprite_data[self.next_sprite_index as usize].y_pos;
        if self.next_sprite_data[self.next_sprite_index as usize].attribute.vflip() {
            if ppu.control_reg.large_sprite() { 
                self.render_sprite_data[self.render_sprite_index as usize].y_pos ^= SPRITE_16X_FLIPMASK;
            }
            else {
                self.render_sprite_data[self.render_sprite_index as usize].y_pos ^= SPRITE_8X_FLIPMASK;
            }
        }
    }

    pub fn fetch_tile_index(&mut self) {
        self.render_sprite_data[self.render_sprite_index as usize].tile_index = self.next_sprite_data[self.next_sprite_index as usize].tile_index;
    }

    pub fn fetch_attribute(&mut self) {
        self.render_sprite_data[self.render_sprite_index as usize].attribute = self.next_sprite_data[self.next_sprite_index as usize].attribute;
    }

    pub fn fetch_x(&mut self) {
        self.render_sprite_data[self.render_sprite_index as usize].x_pos = self.next_sprite_data[self.next_sprite_index as usize].x_pos;
    }

    pub fn pattern0_address(&mut self, ppu: &mut Context) -> u16 {
        let mut tile_index = 0xFF;
        let mut sprite_line = 0xFF;
        if self.render_sprite_data[self.render_sprite_index as usize].active {
            tile_index = self.render_sprite_data[self.render_sprite_index as usize].tile_index as u16;
            sprite_line = self.render_sprite_data[self.render_sprite_index as usize].y_pos as u16;
        }

        if ppu.control_reg.large_sprite() {
            (((tile_index & 1) << 12) | ((tile_index & 0xfe) << 4) | PATTERN0_OFFSET | (sprite_line & 7) | ((sprite_line & 0x08) << 1)) & 0xffff
        }
        else {
            (ppu.control_reg.sprite_table_address()| (tile_index << 4) | PATTERN0_OFFSET | sprite_line) & 0xffff
        }
    }

    pub fn pattern1_address(&mut self, ppu: &mut Context) -> u16 {
        let mut tile_index = 0xFF;
        let mut sprite_line = 0xFF;
        if self.render_sprite_data[self.render_sprite_index as usize].active {
            tile_index = self.render_sprite_data[self.render_sprite_index as usize].tile_index as u16;
            sprite_line = self.render_sprite_data[self.render_sprite_index as usize].y_pos as u16;
        }

        if ppu.control_reg.large_sprite() {
            (((tile_index & 1) << 12) | ((tile_index & 0xfe) << 4) | PATTERN1_OFFSET | (sprite_line & 7) | ((sprite_line & 0x08) << 1)) & 0xffff
        }
        else {
            (ppu.control_reg.sprite_table_address()| (tile_index << 4) | PATTERN1_OFFSET | sprite_line) & 0xffff
        }
    }

    pub fn set_pattern0(&mut self, mut data: u8) {
        if self.render_sprite_data[self.render_sprite_index as usize].attribute.hflip() {
            data = REVERSE_BITS[data as usize];
        }

        self.render_sprite_data[self.render_sprite_index as usize].pattern[0] = data;
    }

    pub fn set_pattern1(&mut self, mut data: u8) {
        if self.render_sprite_data[self.render_sprite_index as usize].attribute.hflip() {
            data = REVERSE_BITS[data as usize];
        }

        self.render_sprite_data[self.render_sprite_index as usize].pattern[1] = data;
        self.render_sprite_index += 1;
        if self.render_sprite_index >= 8 { self.render_sprite_index = 0; }
    }

    pub fn select_sprite_pixel(&mut self, ppu: &mut Context, mut bg_pixel: u8) -> u8 {
        let index = ppu.scanline_dot - 1;
        // Are any sprites in range
        if self.left_most_x as u16 <= index {
            // Then check sprites if they belong
            if (ppu.mask_reg.contains(MaskRegister::LEFTMOST_8PXL_SPRITE) || (ppu.scanline_dot >= 8)) && ppu.mask_reg.contains(MaskRegister::SHOW_SPRITES) {
                // Loop through sprites
                for spr in self.render_sprite_data.iter() {
                    if !spr.active {
                        continue;
                    }

                    let x_offset = index - (spr.x_pos as u16);
                    // Is this sprite visible on this pixel?
                    if x_offset < 8 {
                        let p0 = spr.pattern[0];
                        let p1 = spr.pattern[1];
                        let shift = 7 - x_offset;
                        let sprite_pixel = ((p0 >> shift) & 0x01) | (((p1 >> shift) << 0x01) & 0x02);

                        // This pixel is visible..
                        if (sprite_pixel & 0x03) > 0 {
                            // we rendered a sprite0 pixel which collided with a BG pixel
						    // NOTE: according to blargg's tests, a collision doesn't seem
                            //       possible to occur on the rightmost pixel
                            // bg pixel for sprite 0 hack
                            if spr.zero == true && index < 255 && (bg_pixel & 0x03) > 0 && self.sprite_0_hit == false {
                                ppu.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, true);
                                self.sprite_0_hit = true;
                            }

                            if spr.attribute.infront_of_background() && (bg_pixel & 0x03) == 0 && ppu.mask_reg.contains(MaskRegister::SHOW_SPRITES) {
                                bg_pixel = (0x10 | sprite_pixel | (spr.attribute.pallete_index() << 2)) & 0xff;
                            }

                            return bg_pixel;
                        }
                    }
                }
            }
        }

        return bg_pixel;
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oam_index() {
        let mut oam_index = OamIndex::from_oamaddr(248);
        oam_index.increment_n();
        assert_eq!(oam_index.index(), 252);
        assert_eq!(oam_index.n(), 63);
        assert_eq!(oam_index.m(), 0);
        oam_index.increment_n();
        assert_eq!(oam_index.index(), 0); 
        oam_index.increment_n();
        assert_eq!(oam_index.index(), 4); 
        oam_index.increment_n();
        assert_eq!(oam_index.index(), 8); 

        oam_index.increment_m();
        assert_eq!(oam_index.index(), 9); 
        oam_index.increment_m();
        assert_eq!(oam_index.index(), 10); 
        oam_index.increment_m();
        assert_eq!(oam_index.index(), 11); 
        oam_index.increment_m();
        //assert_eq!(oam_index.index(), 9); 
        
    }

    #[test]
    fn test_sprite_evaluation() {
        let mut sprites = Sprites::new();
        let mut ppu = Context::new();

        sprites.reset_for_eval(0x00);

        let y = 0x1_u8;
        let attrib = 0x0_u8;
        
        
        
    }

}

