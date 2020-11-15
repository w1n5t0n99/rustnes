use super::{Pinout, Context, IO};
use super::ppu_registers::*;

use std::fmt;

pub const SPRITE_8X_VALUE: u16 = 8;
pub const SPRITE_16X_VALUE: u16 = 16;
const SPRITE_8X_FLIPMASK: u16 = 0b00000111;
const SPRITE_16X_FLIPMASK: u16 = 0b00001111;

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
    SOAMOverflow0,
    SOAMOverflow1,
    SOAMOverflow2,
    SOAMOverflow3,
}

#[derive(Debug, Clone, Copy)]
pub struct SpriteEval {
    soam_index: usize,
    poam_index: usize,
    counter: u8,
    oam_data: u8,
    left_most_x: u8,
    eval_state: EvalState,
}

impl SpriteEval {
    pub fn new() -> Self {
        SpriteEval {
            soam_index: 0,
            poam_index: 0,
            counter: 0,
            oam_data: 0,
            left_most_x: 0,
            eval_state: EvalState::YRead,
        }
    }

    fn sprite_in_range(&mut self, ppu: &mut Context, size: u16) -> bool {
        // Y was read into oam data in the previous cycle
        let sprite_line = ppu.scanline_index.saturating_sub(self.oam_data as u16);
        sprite_line < size
    }

    fn increment_poam(&mut self, ppu: &mut Context) {
        self.poam_index = self.poam_index.wrapping_add(4);
        ppu.oam_addr_reg = self.poam_index as u8;
        if self.poam_index == 0 {
            // poam index overflowed
            self.eval_state = EvalState::POAMOverflow;
        }
    }

    fn increment_soam(&mut self) {
        self.soam_index = self.soam_index.wrapping_add(4);
        if self.soam_index == 32 {
            self.soam_index = 0;
            // soam index overflowed
            self.eval_state = EvalState::SOAMOverflow0;
        }
    }

    pub fn reset_for_scanline(&mut self, ppu: &mut Context) {
        ppu.oam_addr_reg = 0;
        self.counter = 0;
        self.soam_index = 0;
        self.poam_index = 0;
        self.left_most_x = 0;
        self.eval_state = EvalState::YRead;
    }

    pub fn evaluate(&mut self, ppu: &mut Context, size: u16) {
        match self.eval_state {
            EvalState::YRead => {
                self.oam_data = ppu.oam_ram_primary[self.poam_index];
                self.eval_state = EvalState::YWrite;
            }
            EvalState::YWrite => {
                ppu.oam_ram_secondary[self.soam_index] = self.oam_data;
                if self.sprite_in_range(ppu, size) {
                    self.eval_state = EvalState::IndexRead;
                }
                else {
                    self.increment_poam(ppu);
                }
            }
            EvalState::IndexRead => {
                self.oam_data = ppu.oam_ram_primary[self.poam_index + 1];
                self.eval_state = EvalState::IndexWrite;
            }
            EvalState::IndexWrite => {
                ppu.oam_ram_secondary[self.soam_index + 1] = self.oam_data;
                self.eval_state = EvalState::AttributeRead;
            }
            EvalState::AttributeRead => {
                self.oam_data = ppu.oam_ram_primary[self.poam_index + 2];
                self.eval_state = EvalState::AttributeWrite;
            }
            EvalState::AttributeWrite => {
                ppu.oam_ram_secondary[self.soam_index + 2] = self.oam_data;
                self.eval_state = EvalState::XRead;
            }
            EvalState::XRead => {
                self.oam_data = ppu.oam_ram_primary[self.poam_index + 3];
                self.eval_state = EvalState::XRead;
            }
            EvalState::XWrite => {
                ppu.oam_ram_secondary[self.soam_index + 3] = self.oam_data;
                // increment secondary check if overflows
                self.increment_soam();
                //increment primary checking if overflow
                self.increment_poam(ppu);
            }
            EvalState::SOAMOverflow0 => {
                // we already found 8 sprites check if we need to set sprite overlow flag
                
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sprites {
   
}

impl Sprites {
    pub fn new() -> Self {
        // TODO implemet sprites
        Sprites { 
        }
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