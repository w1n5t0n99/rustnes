use super::Context;
use super::ppu_registers::*;

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
struct OamIndex (u8);
impl OamIndex {
    pub fn n(&self) -> u8 {
        (self.0 & 0xFC) >> 2
    }

    pub fn m(&self) -> u8 {
        self.0 & 0x3
    }

    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    pub fn increment_n(&mut self) {
        self.0 = (self.0 & 0x03) | (self.0 & 0xFC).wrapping_add(4);
    }

    pub fn increment_m(&mut self) {
        self.0 = (self.0 & 0xFC) | (self.0 & 0x03).wrapping_add(1);
    }
}

#[derive(Debug, Clone, Copy)]
enum EvalState {
    StateYRead,
    StateYWrite,
    StateIndexRead,
    StateIndexWrite,
    StateAttribRead,
    StateYAttribWrite,
    StateXRead,
    StateXWrite,
    StateSecondaryOamFull,
    StateOamIndexOverflow,
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
    pub pattern: [u8; 2],
}

impl SpriteData {
    pub fn new() -> Self {
        SpriteData {
            y_pos: 0xFF,
            x_pos: 0xFF,
            tile_index: 0xFF,
            attribute: SpriteAttribute::new(),
            pattern: [0xFF; 2],
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Sprites {
    pub secondary_oam: [u8; 32],
    pub sprite_data: [SpriteData; 8],
    pub sprites_count: u8,
    pub next_sprites_count: u8,
    primary_oam_index: OamIndex,
    //secondary_oam_index: u8,
    sprite_data_index: u8,
    left_most_x: u8,
    next_left_most_x: u8,
    eval_state: EvalState,
}


impl Sprites {
    pub fn new() -> Self {
        Sprites {
            secondary_oam: [0xFF; 32],
            sprite_data: [SpriteData::new(); 8],
            sprites_count: 0,
            next_sprites_count: 0,
            primary_oam_index: OamIndex(0x0),
            //secondary_oam_index: 0,
            sprite_data_index: 0,
            left_most_x: 0xFF,
            next_left_most_x: 0xFF,
            eval_state: EvalState::StateYRead,
        }
    }

    #[inline(always)]
    fn secondary_oam_index(&self, sprite_index: u8, offset: u8) -> u8 {
        sprite_index * 4 + offset
    }

    #[inline(always)]
    fn fetch_y(&mut self, sprite_index: usize) -> u8 {
        self.secondary_oam[sprite_index * 4 + 0]
    }

    #[inline(always)]
    fn fetch_tile_index(&mut self, sprite_index: usize) -> u8 {
        self.secondary_oam[sprite_index * 4 + 1]
    }

    #[inline(always)]
    fn fetch_attribute(&mut self, sprite_index: usize) -> SpriteAttribute {
        SpriteAttribute::from_bits_truncate(self.secondary_oam[sprite_index * 4 + 2])
    }

    #[inline(always)]
    fn fetch_x(&mut self, sprite_index: usize) -> u8 {
        self.secondary_oam[sprite_index * 4 + 3]
    }

    #[inline(always)]
    fn sprite_in_range(&mut self, ppu: &mut Context, sprite_size: u16, y_pos: u16) -> bool {
        // Sprite eval happens for the next scanline
        let sprite_line = ppu.scanline_index.wrapping_sub(y_pos);
        sprite_line < sprite_size
    }

    // Secondary sprite data is cleared between dots (1-64)
    // We'll just clear on dot 65 before first eval
    pub fn clear_secondary_oam(&mut self, oam_addr: u8) {
        self.primary_oam_index = OamIndex(oam_addr);
       // self.secondary_oam_index = 0;
        self.next_sprites_count = 0;
        self.next_left_most_x = 0xFF;
        self.eval_state = EvalState::StateYRead;
        for x in self.secondary_oam.iter_mut() { *x = 0xFF; }
    }

    pub fn evaluate(&mut self, ppu: &mut Context) {
        // OAMDATA exposes OAM accesses during sprite eval
        ppu.oam_addr_reg = self.primary_oam_index.0;
        let sprite_size = ppu.control_reg.sprite_size() as u16;

        match self.eval_state {
            EvalState::StateYRead => {
                self.eval_state = EvalState::StateYWrite;
            }
            EvalState::StateYWrite => {
                let data = ppu.oam_ram_primary[self.primary_oam_index.0 as usize];
        
                if self.sprite_in_range(ppu, sprite_size, data as u16) {
                    let sprite_line =  ppu.scanline_index.wrapping_sub(data as u16);
                    self.secondary_oam[self.secondary_oam_index(self.next_sprites_count, 0) as usize] = sprite_line as u8;
                    self.eval_state = EvalState::StateIndexRead;
                }
                else {
                    self.primary_oam_index.increment_n();
                    if self.primary_oam_index.0 == 0 { self.eval_state = EvalState::StateOamIndexOverflow; }
                }
            }
            EvalState::StateIndexRead => {
                self.eval_state = EvalState::StateIndexWrite;
            }
            EvalState::StateIndexWrite => {
                let data = ppu.oam_ram_primary[self.primary_oam_index.0 as usize + 1];
                self.secondary_oam[self.secondary_oam_index(self.next_sprites_count, 1) as usize] = data;
                self.eval_state = EvalState::StateAttribRead;
            }
            EvalState::StateAttribRead => {
                self.eval_state = EvalState::StateYAttribWrite;
            }
            EvalState::StateYAttribWrite => {
                let data = ppu.oam_ram_primary[self.primary_oam_index.0 as usize + 2];
                self.secondary_oam[self.secondary_oam_index(self.next_sprites_count, 2) as usize ] = data;
                self.eval_state = EvalState::StateXRead;
            }
            EvalState::StateXRead => {
                self.eval_state = EvalState::StateXWrite;
            }
            EvalState::StateXWrite => {
                let data = ppu.oam_ram_primary[self.primary_oam_index.0 as usize + 3];
                self.secondary_oam[self.secondary_oam_index(self.next_sprites_count, 3) as usize] = data;
                self.next_left_most_x = self.next_left_most_x.min(data);
                self.eval_state = EvalState::StateYRead;
                // check if 8 sprites have been found
                self.next_sprites_count += 1;
                //self.secondary_oam_index += 4;
                if self.next_sprites_count >= 8 {
                     self.eval_state = EvalState::StateSecondaryOamFull; 
                }
                //increment primary checking if overflow
                self.primary_oam_index.increment_n();
                if self.primary_oam_index.0 == 0 {
                     self.eval_state = EvalState::StateOamIndexOverflow;
                }
            }
            EvalState::StateSecondaryOamFull => {
                // we already found 8 sprites check if we need to set sprite overlow flag
                let data = ppu.oam_ram_primary[self.primary_oam_index.0 as usize];
                if self.sprite_in_range(ppu, sprite_size, data as u16) {
                    ppu.status_reg.set(StatusRegister::SPRITE_OVERFLOW, true);
                    self.primary_oam_index.increment();
                    if self.primary_oam_index.0 == 0 {
                         self.eval_state = EvalState::StateOamIndexOverflow; 
                    }
                }
                else {
                    self.primary_oam_index.increment_m();
                    self.primary_oam_index.increment_n();
                    if self.primary_oam_index.n() == 0 { 
                        self.eval_state = EvalState::StateOamIndexOverflow;
                    }
                }
            }
            EvalState::StateOamIndexOverflow => {
                self.primary_oam_index.increment_n();
            }
        }
    }

    pub fn fetch_sprite_data(&mut self, ppu: &mut Context) {
        self.left_most_x = self.next_left_most_x;
        self.sprites_count = self.next_sprites_count;

        let current_sprite_index= ((ppu.scanline_dot - 1) >> 3) & 0x07;
        let y = self.fetch_y(current_sprite_index as usize);

        self.sprite_data[current_sprite_index as usize].y_pos = y;
        if current_sprite_index < (self.sprites_count as u16) {
            self.sprite_data[current_sprite_index as usize].tile_index = self.fetch_tile_index( current_sprite_index as usize);
            self.sprite_data[current_sprite_index as usize].attribute = self.fetch_attribute(current_sprite_index as usize);
            self.sprite_data[current_sprite_index as usize].x_pos = self.fetch_x(current_sprite_index as usize);

            if self.sprite_data[current_sprite_index as usize].attribute.vflip() && ppu.control_reg.large_sprite() {
                self.sprite_data[current_sprite_index as usize].y_pos ^= SPRITE_16X_FLIPMASK;
            }
            else if self.sprite_data[current_sprite_index as usize].attribute.vflip() {
                self.sprite_data[current_sprite_index as usize].y_pos ^= SPRITE_8X_FLIPMASK;
            }
        }
    }

    pub fn pattern0_address(&mut self, ppu: &mut Context) -> u16 {
        let current_sprite_index= ((ppu.scanline_dot - 1) >> 3) & 0x07;

        if current_sprite_index < (self.sprites_count as u16) {
            let spr = &mut self.sprite_data[current_sprite_index as usize];
            if ppu.control_reg.large_sprite() {
                ((((spr.tile_index as u16) & 1) << 12) | (((spr.tile_index as u16) & 0xfe) << 4) | PATTERN0_OFFSET | ((spr.y_pos as u16) & 7) | (((spr.y_pos as u16) & 0x08) << 1)) & 0xffff
            }
            else {
                (ppu.control_reg.sprite_table_address()| (((spr.tile_index as u16)) << 4) | PATTERN0_OFFSET | (spr.y_pos as u16)) & 0xffff
            }
        }
        else {
            // dummy address
            if ppu.control_reg.large_sprite() {
                (((0xFF & 1) << 12) | ((0xFF & 0xfe) << 4) | PATTERN0_OFFSET | (0xFF & 7) | ((0xFF & 0x08) << 1)) & 0xffff
            }
            else {
                (ppu.control_reg.sprite_table_address()| (0xFF << 4) | PATTERN0_OFFSET | 0xFF) & 0xffff
            }
        }
    }

    pub fn pattern1_address(&mut self, ppu: &mut Context) -> u16 {
        let current_sprite_index= ((ppu.scanline_dot - 1) >> 3) & 0x07;

        if current_sprite_index < (self.sprites_count as u16) {
            let spr = &mut self.sprite_data[current_sprite_index as usize];
            if ppu.control_reg.large_sprite() {
                ((((spr.tile_index as u16) & 1) << 12) | (((spr.tile_index as u16) & 0xfe) << 4) | PATTERN1_OFFSET | ((spr.y_pos as u16) & 7) | (((spr.y_pos as u16) & 0x08) << 1)) & 0xffff
            }
            else {
                (ppu.control_reg.sprite_table_address()| (((spr.tile_index as u16)) << 4) | PATTERN1_OFFSET | (spr.y_pos as u16)) & 0xffff
            }
        }
        else {
            // dummy address
            if ppu.control_reg.large_sprite() {
                (((0xFF & 1) << 12) | ((0xFF & 0xfe) << 4) | PATTERN1_OFFSET | (0xFF & 7) | ((0xFF & 0x08) << 1)) & 0xffff
            }
            else {
                (ppu.control_reg.sprite_table_address()| (0xFF << 4) | PATTERN1_OFFSET | 0xFF) & 0xffff
            }
        }
    }

    pub fn set_pattern0(&mut self, ppu: &mut Context, mut data: u8) {
        let current_sprite_index= ((ppu.scanline_dot - 1) >> 3) & 0x07;
        let spr = &mut self.sprite_data[current_sprite_index as usize];

        if spr.attribute.hflip() {
            data = REVERSE_BITS[data as usize];
        }

        spr.pattern[PATTERN0_INDEX] = data;
    }

    pub fn set_pattern1(&mut self,  ppu: &mut Context, mut data: u8) {
        let current_sprite_index= ((ppu.scanline_dot - 1) >> 3) & 0x07;
        let spr = &mut self.sprite_data[current_sprite_index as usize];

        if spr.attribute.hflip() {
            data = REVERSE_BITS[data as usize];
        }

        spr.pattern[PATTERN1_INDEX] = data;
    }

    pub fn select_sprite_pixel(&mut self, ppu: &mut Context, mut bg_pixel: u8) -> u8 {
        let index = ppu.scanline_dot - 1;
        // Are any sprites in range

        if (self.left_most_x as u16) <= index {

            // Then check sprites if they belong
            if (ppu.mask_reg.contains(MaskRegister::LEFTMOST_8PXL_SPRITE) || (index >= 8)) && ppu.mask_reg.contains(MaskRegister::SHOW_SPRITES) {
                // Loop through sprites
                for i in 0..self.sprites_count {
                    let spr = &mut self.sprite_data[i as usize];
                    
                    let x_offset = index.wrapping_sub(spr.x_pos as u16);
                    if spr.y_pos == 0xFF {
                        continue;
                    }

                    // Is this sprite visible on this pixel?
                    if x_offset < 8 {
                        let p0 = spr.pattern[PATTERN0_INDEX];
                        let p1 = spr.pattern[PATTERN1_INDEX];
                        let shift = 7 - x_offset;
                        let sprite_pixel = ((p0 >> shift) & 0x01) | (((p1 >> shift) << 0x01) & 0x02);

                        // This pixel is visible..
                        if (sprite_pixel & 0x03) > 0 {
                            // we rendered a sprite0 pixel which collided with a BG pixel
						    // NOTE: according to blargg's tests, a collision doesn't seem
                            //       possible to occur on the rightmost pixel
                            // bg pixel for sprite 0 hack
                            if i == 0 && index < 255 && (bg_pixel & 0x03) > 0 {
                                ppu.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, true);
                            }

                            if spr.attribute.infront_of_background() || (bg_pixel & 0x03) == 0 {
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
        let mut oam_index = OamIndex(248);
        oam_index.increment_n();
        assert_eq!(oam_index.0, 252);
        assert_eq!(oam_index.n(), 63);
        assert_eq!(oam_index.m(), 0);
        oam_index.increment_n();
        assert_eq!(oam_index.0, 0); 
        oam_index.increment_n();
        assert_eq!(oam_index.0, 4); 
        oam_index.increment_n();
        assert_eq!(oam_index.0, 8); 

        oam_index.increment_m();
        assert_eq!(oam_index.0, 9); 
        oam_index.increment_m();
        assert_eq!(oam_index.0, 10); 
        oam_index.increment_m();
        assert_eq!(oam_index.0, 11); 
        oam_index.increment_m();
        //assert_eq!(oam_index.index(), 9); 
        
    }

    #[test]
    fn test_sprite_evaluation() {
        let mut sprites = Sprites::new();
        let mut ppu = Context::new();
        sprites.clear_secondary_oam(0x00);

        let y = 0x0_u8;
        let index = 0x02_u8;
        let attrib = 0x3_u8;
        let x = 0x4_u8;

        for i in (0..256).step_by(4) {
            ppu.oam_ram_primary[i] = y;
            ppu.oam_ram_primary[i+1] = index;
            ppu.oam_ram_primary[i+2] = attrib;
            ppu.oam_ram_primary[i+3] = x;
        }
        
        for i in 65..=256 {
            sprites.evaluate(&mut ppu);
        }

        assert_eq!(sprites.secondary_oam[0], 0);
        assert_eq!(sprites.next_sprites_count, 8);
        assert_eq!(ppu.status_reg.contains(StatusRegister::SPRITE_OVERFLOW), true);

        //=================================================

        ppu = Context::new();
        sprites.clear_secondary_oam(0x00);

        for i in (0..32).step_by(4) {
            ppu.oam_ram_primary[i] = y;
            ppu.oam_ram_primary[i+1] = index;
            ppu.oam_ram_primary[i+2] = attrib;
            ppu.oam_ram_primary[i+3] = x;
        }

        for i in 32..256 {
            ppu.oam_ram_primary[i] = 0xFF;
        }

        for _i in 65..=256 {
            sprites.evaluate(&mut ppu);
        }

        assert_eq!(sprites.secondary_oam[0], 0);
        assert_eq!(sprites.next_sprites_count, 8);
        assert_eq!(ppu.status_reg.contains(StatusRegister::SPRITE_OVERFLOW), false);

        //===============================================

        ppu = Context::new();
        sprites.clear_secondary_oam(0x00);

        for i in (0..28).step_by(4) {
            ppu.oam_ram_primary[i] = y;
            ppu.oam_ram_primary[i+1] = index;
            ppu.oam_ram_primary[i+2] = attrib;
            ppu.oam_ram_primary[i+3] = x;
        }

        for i in 28..256 {
            ppu.oam_ram_primary[i] = 0xFF;
        }

        for _i in 65..=256 {
            sprites.evaluate(&mut ppu);
        }

        assert_eq!(sprites.secondary_oam[0], 0);
        assert_eq!(sprites.next_sprites_count, 7);
        assert_eq!(ppu.status_reg.contains(StatusRegister::SPRITE_OVERFLOW), false);

    }

    #[test]
    fn test_sprite_fetching() {
        let mut sprites = Sprites::new();
        let mut ppu = Context::new();
        sprites.clear_secondary_oam(0x00);

        let y = 0x0_u8;
        let index = 0x02_u8;
        let attrib = 0x3_u8;
        let x = 0x4_u8;

        for i in (0..256).step_by(4) {
            ppu.oam_ram_primary[i] = 0x80;
            ppu.oam_ram_primary[i+1] = 0x80;
            ppu.oam_ram_primary[i+2] = 0;
            ppu.oam_ram_primary[i+3] = 0x80;
        }

        ppu.oam_ram_primary[0] = y;
        ppu.oam_ram_primary[1] = index;
        ppu.oam_ram_primary[2] = attrib;
        ppu.oam_ram_primary[3] = x;

        ppu.oam_ram_primary[252] = y;
        ppu.oam_ram_primary[253] = index;
        ppu.oam_ram_primary[254] = attrib;
        ppu.oam_ram_primary[255] = x;
        
        for i in 65..=256 {
            sprites.evaluate(&mut ppu);
        }

        assert_eq!(sprites.next_sprites_count, 2);
        assert_eq!(ppu.status_reg.contains(StatusRegister::SPRITE_OVERFLOW), false);

        ppu.scanline_dot = 257;
        let mut addr = 0xff_u16;

        for _i in 257..=320 {
            let data = (ppu.scanline_dot & 0x07) as u8;
            match ppu.scanline_dot & 0x07 {
                1 => {
                    sprites.fetch_sprite_data(&mut ppu);
                }
                2 => {
                }
                3 => {
                }
                4 => {
                }
                5 => {
                    addr = sprites.pattern0_address(&mut ppu);
                }
                6 => {
                    addr = sprites.pattern0_address(&mut ppu);
                    sprites.set_pattern0(&mut ppu, data);
                    
                }
                7 => {
                    addr = sprites.pattern1_address(&mut ppu);
                }
                0 => {
                    addr = sprites.pattern1_address(&mut ppu);
                    sprites.set_pattern1(&mut ppu, data);

                }
                _ => {
                    panic!("ppu 305-321 out of bounds");
                }
            }

            ppu.scanline_dot += 1;
        }

        assert_eq!(sprites.sprites_count, 2);
        assert_eq!(sprites.secondary_oam[0], 0);
        assert_eq!(sprites.secondary_oam[4], 0);
        assert_eq!(sprites.secondary_oam[8], 0xFF);

        assert_eq!(sprites.sprite_data[0].y_pos, 0);
        assert_eq!(sprites.sprite_data[0].x_pos, 4);
        assert_eq!(sprites.sprite_data[1].y_pos, 0);
        assert_eq!(sprites.sprite_data[1].x_pos, 4);

    }

}

