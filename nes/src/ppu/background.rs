
use super::Context;
use super::ppu_registers::*;

const PATTERN0_OFFSET: u16 = 0;
const PATTERN1_OFFSET: u16 = 8;
const PATTERN0_INDEX: usize = 0;
const PATTERN1_INDEX: usize = 1;

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

    pub fn set_next_pattern0(&mut self, data: u8) {
        self.next_pattern[PATTERN0_INDEX] = data;
    }

    pub fn set_next_pattern1(&mut self, data: u8) {
        self.next_pattern[PATTERN1_INDEX] = data;
    }

    pub fn select_background_pixel(&mut self, ppu: &mut Context) -> u8 {
        let pixel: u8;
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

    pub fn pattern0_address(&mut self, ppu: &mut Context) -> u16 {
        (ppu.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN0_OFFSET | ppu.addr_reg.tile_line()) & 0xFFFF
    }

    pub fn pattern1_address(&mut self, ppu: &mut Context) -> u16 {
        (ppu.control_reg.background_table_address() | (self.next_tile_index << 4)  | PATTERN1_OFFSET | ppu.addr_reg.tile_line()) & 0xFFFF
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