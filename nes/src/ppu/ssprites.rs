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
// internal flag to track sprite zero from OAM
const OAM_ZERO: u8 = 0b00010000;
const OAM_PRIORITY: u8 = 0b00100000;
const VFLIP: u8 = 0b10000000;
const HFLIP: u8 = 0b01000000;
const WRITE: bool = true;
const READ: bool = false;


#[derive(Debug, Clone, Copy)]
struct SpriteInfo {
	pub pattern_queue: [u8; 2],
	pub sprite_line: u8, // used to get the correct tile data, adjusted for vflip/hflip
	pub attribute: u8,
	pub xpos_counter: u8,
	pub tile_index: u8,
	pub valid_sprite: bool,
	pub sprite_0: bool,
}

impl SpriteInfo {
	pub fn new() -> Self {
		Self {
			pattern_queue: [0; 2],
			sprite_line: 0,
			attribute: 0,
			xpos_counter: 0,
			tile_index: 0,
			valid_sprite: false,
			sprite_0: false,
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum SpriteEvalState {
	SpriteSearch(u8, bool),
    OverflowSearch(u8, bool),
    End(bool),
}

impl SpriteEvalState {
	pub fn from_start_state() -> Self {
		SpriteEvalState::SpriteSearch(0, READ)
	}

	pub fn from_overflow_state() -> Self {
		SpriteEvalState::OverflowSearch(0, READ)
	}

	pub fn from_end_state() -> Self {
		SpriteEvalState::End(READ)
	}

	pub fn transition(&mut self) {
		*self = match *self {
			Self::SpriteSearch(index, READ) => Self::SpriteSearch(index, WRITE),
			Self::SpriteSearch(index, WRITE) => Self::SpriteSearch(index+1, READ),
			Self::OverflowSearch(index, READ) => Self::OverflowSearch(index, WRITE),
			Self::OverflowSearch(index, WRITE) => Self::OverflowSearch(index+1, READ),
			Self::End(rw) => Self::End(!rw),
		};
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Sprites {
    primary_oam: [u8; 256],
    secondary_oam: [u8; 32],
    sprites: [SpriteInfo; 8],
	oam_addr: usize,
	secondary_oam_addr: usize,
	oam_data_buffer: u8,
	eval_state: SpriteEvalState,
	sprite_count: u8,
}

impl Sprites {
	pub fn new() -> Self {
		Sprites {
			primary_oam: [0xFF; 256],
			secondary_oam: [0xFF; 32],
			sprites: [SpriteInfo::new(); 8],
			oam_addr: 0,
			secondary_oam_addr: 0,
			oam_data_buffer: 0,
			eval_state: SpriteEvalState::from_start_state(),
			sprite_count: 0,
		}
	}

	pub fn write_oamaddr_reg(&mut self, data: u8) {
		self.oam_addr = data as usize;
	}

	pub fn read_oamdata_reg(&mut self, context: &Context) -> u8 {
		if context.vpos <= 239 && context.mask_reg.rendering_enabled() {
			// while screen is being drawn
			self.oam_data_buffer
		}
		else {
			self.primary_oam[self.oam_addr]
		}
	}

	pub fn write_oamdata_reg(&mut self, context: &Context, mut data: u8) {
		if context.vpos >= 240 || !context.mask_reg.rendering_enabled() {
			if(self.oam_addr & 0x03) == 0x02 {
				//"The three unimplemented bits of each sprite's byte 2 do not exist in the context and always read back as 0 on context revisions that allow reading context OAM through OAMDATA ($2004)"
				data &= 0xE3;
			}
			self.primary_oam[self.oam_addr] = data;
			self.oam_addr = (self.oam_addr + 1) & 0xFF;
		}
		else {
			//"Writes to OAMDATA during rendering (on the pre-render line and the visible lines 0-239, provided either sprite
			// or background rendering is enabled) do not modify values in OAM, but do perform a glitchy increment of 
			// OAMADDR, bumping only the high 6 bits"
			self.oam_addr = (self.oam_addr + 4) & 0xFF;
		}
	}

	pub fn clear_secondary_oam(&mut self) {
		for d in self.secondary_oam.iter_mut() { *d = 0xFF; }
		self.oam_data_buffer = 0xFF;
	}

	pub fn process_sprite_evaluation(&mut self, context: &mut Context) {
		if context.hpos == 65 {
			self.begin_evaluation();
		}

		match self.eval_state {
			SpriteEvalState::SpriteSearch(0, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::SpriteSearch(0, WRITE) => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				if self.sprite_in_range(context, self.oam_data_buffer) {
					// copy remaining bytes for sprite
					self.secondary_oam_addr += 1;
					self.increment_low_m();
					self.eval_state.transition();
				}
				else {
					self.increment_high_n();
					// has oam address overflowed
					if (self.oam_addr & 0xFC) == 0 { self.eval_state = SpriteEvalState::from_end_state(); }
					// else continue searching
					else { self.eval_state = SpriteEvalState::from_start_state(); }
				}
			}
			SpriteEvalState::SpriteSearch(1, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::SpriteSearch(1, WRITE) => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				self.secondary_oam_addr += 1;
				self.increment_low_m();
				self.eval_state.transition();
			}
			SpriteEvalState::SpriteSearch(2, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				if self.oam_addr < 4 {
					// sprite 0 would be OAM[0] - OAM[3]
					self.oam_data_buffer |= OAM_ZERO;
				}

				self.eval_state.transition();
			}
			SpriteEvalState::SpriteSearch(2, WRITE) => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				self.secondary_oam_addr += 1;
				self.increment_low_m();
				self.eval_state.transition();
			}
			SpriteEvalState::SpriteSearch(3, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::SpriteSearch(3, WRITE) => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				self.increment_low_m();
				self.increment_high_n();
				self.secondary_oam_addr += 1;
				self.sprite_count += 1;
				if (self.oam_addr & 0xFC) == 0 { self.eval_state = SpriteEvalState::from_end_state(); }
				else if self.secondary_oam_addr >= 0x20 {  self.secondary_oam_addr = 0x1F; self.eval_state = SpriteEvalState::from_overflow_state(); }
				else { self.eval_state = SpriteEvalState::from_start_state(); }
			}
			SpriteEvalState::End(READ)=> {
				// attempt (and fail) to copy OAM[n][0] into the next free slot in secondary OAM
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::End(WRITE) => {
				// a side effect of the OAM write disable signal is to turn writes to the secondary OAM into reads from it
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_high_n();
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(0, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(0, WRITE) => {
				let buffer = self.oam_data_buffer;
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];

				if self.sprite_in_range(context, buffer) {
					//  If the value is in range, set the sprite overflow flag in $2002 and read the next 3 entries of OAM
					// (incrementing 'm' after each byte and incrementing 'n' when 'm' overflows); if m = 3, increment n
					context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, true);
					self.increment_low_m();  
					self.eval_state.transition();
				}
				else {
					//  If the value is not in range, increment n and m (without carry). If n overflows to 0, go to 4; otherwise go to 3
					self.increment_high_n();
					self.increment_low_m();
					if (self.oam_addr & 0xFC) == 0 { self.eval_state = SpriteEvalState::from_end_state(); }
					else { self.eval_state = SpriteEvalState::from_overflow_state(); }
				}
			}
			SpriteEvalState::OverflowSearch(1, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(1, WRITE) => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_low_m();
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(2, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(2, WRITE) => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_low_m();
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(3, READ) => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state.transition();
			}
			SpriteEvalState::OverflowSearch(3, WRITE) => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_high_n();
				self.increment_low_m();
				if (self.oam_addr & 0xFC) == 0 { self.eval_state = SpriteEvalState::from_end_state(); }
				else { self.eval_state = SpriteEvalState::from_overflow_state(); }
			}
			_ => {
				panic!("Invalid sprite evaluation state");
			}
		}
	}

	pub fn fetch_sprite_tile_data(&mut self, context: &mut Context) {
		// doesn't appear to have any side effects, should be able to grab all at once
		let sc = self.sprite_count as usize;
		let mut sprite_index = 0;
		for i in (0..(sc*4)).step_by(4) {
			self.sprites[sprite_index].xpos_counter = self.secondary_oam[i+3];
			self.sprites[sprite_index].attribute = self.secondary_oam[i+2];
			self.sprites[sprite_index].tile_index = self.secondary_oam[i+1];
			self.sprites[sprite_index].sprite_line = (context.vpos - (self.secondary_oam[i+0] as u16)) as u8;
			self.sprites[sprite_index].valid_sprite = if sprite_index < (self.sprite_count as usize) {
				 true
			}
			else {
				false
			};

			// adjust for vflip
			if (self.sprites[sprite_index].attribute & VFLIP) > 0 && context.control_reg.large_sprite() {
                self.sprites[sprite_index].sprite_line ^= SPRITE_16X_FLIPMASK;
            }
            else if (self.sprites[sprite_index].attribute & VFLIP) > 0 {
                self.sprites[sprite_index].sprite_line ^= SPRITE_8X_FLIPMASK;
            }

			sprite_index += 1;
		}
	}

	pub fn pattern0_address(&mut self, context: &mut Context) -> u16 {
        let current_sprite_index= (((context.hpos - 1) >> 3) & 0x07) as usize;
        if context.control_reg.large_sprite() {
			((((self.sprites[current_sprite_index].tile_index as u16) & 1) << 12) | (((self.sprites[current_sprite_index].tile_index as u16) & 0xfe) << 4) | PATTERN0_OFFSET | ((self.sprites[current_sprite_index].sprite_line as u16) & 7) | (((self.sprites[current_sprite_index].sprite_line as u16) & 0x08) << 1)) & 0xffff
		}
		else {
			(context.control_reg.sprite_table_address()| (((self.sprites[current_sprite_index].tile_index as u16)) << 4) | PATTERN0_OFFSET | (self.sprites[current_sprite_index].sprite_line as u16)) & 0xffff
		}
    }

	pub fn pattern1_address(&mut self, context: &mut Context) -> u16 {
        let current_sprite_index= (((context.hpos - 1) >> 3) & 0x07) as usize;
        if context.control_reg.large_sprite() {
			((((self.sprites[current_sprite_index].tile_index as u16) & 1) << 12) | (((self.sprites[current_sprite_index].tile_index as u16) & 0xfe) << 4) | PATTERN1_OFFSET | ((self.sprites[current_sprite_index].sprite_line as u16) & 7) | (((self.sprites[current_sprite_index].sprite_line as u16) & 0x08) << 1)) & 0xffff
		}
		else {
			(context.control_reg.sprite_table_address()| (((self.sprites[current_sprite_index].tile_index as u16)) << 4) | PATTERN1_OFFSET | (self.sprites[current_sprite_index].sprite_line as u16)) & 0xffff
		}
    }

	pub fn set_pattern0(&mut self, context: &mut Context, mut data: u8) {
		let current_sprite_index= (((context.hpos - 1) >> 3) & 0x07) as usize;
		if current_sprite_index >= (self.sprite_count as usize) {
			//load pattern tables with transparent data
			self.sprites[current_sprite_index].pattern_queue[PATTERN0_INDEX] = 0;
		}
		else {
			if (self.sprites[current_sprite_index].attribute & HFLIP) > 0 {
				// horizontal flip pattern
				data = REVERSE_BITS[data as usize];
			}

			self.sprites[current_sprite_index].pattern_queue[PATTERN0_INDEX] = data;
		}
    }

    pub fn set_pattern1(&mut self, context: &mut Context, mut data: u8) {
		let current_sprite_index= (((context.hpos - 1) >> 3) & 0x07) as usize;
        if current_sprite_index >= (self.sprite_count as usize) {
			//load pattern tables with transparent data
			self.sprites[current_sprite_index].pattern_queue[PATTERN1_INDEX] = 0;
		}
		else {
			if (self.sprites[current_sprite_index].attribute & HFLIP) > 0 {
				// horizontal flip pattern
				data = REVERSE_BITS[data as usize];
			}

			self.sprites[current_sprite_index].pattern_queue[PATTERN1_INDEX] = data;
		}
    }

	pub fn select_sprite_pixel(&mut self, context: &mut Context, mut bg_pixel: u8) -> u8 {
		let pixel_index = context.hpos - 1;

		// check if sprites should be drawn
		if (context.mask_reg.contains(MaskRegister::LEFTMOST_8PXL_SPRITE) || (pixel_index >= 8)) && context.mask_reg.contains(MaskRegister::SHOW_SPRITES) {
			for (sprite_index, sprite) in self.sprites.iter().enumerate() {
				if sprite.valid_sprite == false {
					continue;
				}

				let x_offset = pixel_index.saturating_sub(sprite.xpos_counter as u16);
				// check if sprite is visible on this pixel, first sprite found takes priority
				if x_offset < 8 {
					let shift = 7 - x_offset;
					let spr_pixel = ((sprite.pattern_queue[PATTERN0_INDEX] >> shift) & 0x01) | (((sprite.pattern_queue[PATTERN1_INDEX] >> shift) << 0x01) & 0x02);
					// check if pixel is visible
					if (spr_pixel & 0x03) > 0 {
						// check for sprite 0 hit
						// according to Mesen, a sprite 0 hit does not occur on hpos 255
						if (sprite.attribute & OAM_ZERO) > 0 && context.hpos < 255 && (bg_pixel & 0x03) > 0 {
							context.status_reg.set(StatusRegister::SPRITE_ZERO_HIT, true);
						}

						if (sprite.attribute & OAM_PRIORITY) == 0 || (bg_pixel & 0x03) == 0 {
							bg_pixel = 0x10 | spr_pixel | ((sprite.attribute & 0x02) << 2) & 0xff;
						}

						return bg_pixel;
					}
				}
			}
		}

		bg_pixel
	}

	pub fn clear_oam_addr(&mut self) {
		//OAMADDR is set to 0 during each of ticks 257-320 (the sprite tile loading interval) of the pre-render and visible scanlines
		self.oam_addr = 0;
	}

	pub fn clear_sprites(&mut self) {
		// call during prerender so no sprites will be drawn on render line 0
		for spr in &mut self.sprites {
			spr.valid_sprite = false;
			spr.pattern_queue[PATTERN0_INDEX] = 0;
			spr.pattern_queue[PATTERN1_INDEX] = 0;
		}
	}

	fn begin_evaluation(&mut self) {
		self.secondary_oam_addr = 0;
		self.sprite_count = 0;
		self.eval_state = SpriteEvalState::from_start_state();
	}

	fn sprite_in_range(&self, context: &Context, y_pos: u8) -> bool {
		//println!("++++++ Y in range: {} - {}", y_pos, context.vpos);
		if (y_pos as u16) >= context.vpos && (y_pos as u16) <  (context.vpos + (context.control_reg.sprite_size() as u16)) {
			true
		}
		else {
			false
		}
	}

	fn increment_high_n(&mut self) {
		// high 6 bits of OAMADDR
		self.oam_addr = (self.oam_addr & 0x03) | ((self.oam_addr & 0xFC).wrapping_add(4) & 0xFC)
	}

	fn increment_low_m(&mut self) {
		// low 2 bits of OAMADDR
		self.oam_addr = (self.oam_addr & 0xfc) | (((self.oam_addr & 0x03) + 1) & 0x03);
	}

}

#[cfg(test)]
mod test {
    use super::*;

	fn init_oam(oam: &mut [u8]) {
		for data in oam {
			*data = 0xFF;
		}
	}

	fn set_sprite(oam: &mut [u8], ypos: u8, other_data: u8, index: usize) {
		oam[(index*4)+0] = ypos;
		oam[(index*4)+1] = other_data;
		oam[(index*4)+2] = other_data;
		oam[(index*4)+3] = other_data;
	}

	fn set_sprite_full(oam: &mut [u8], ypos: u8, xpos: u8, at: u8, ti: u8, index: usize) {
		oam[(index*4)+0] = ypos;
		oam[(index*4)+1] = ti;
		oam[(index*4)+2] = at;
		oam[(index*4)+3] = xpos;
	}

	#[test]
	fn test_oam_addr_increment() {
		let mut sprites = Sprites::new();
		sprites.increment_high_n();
		assert_eq!(sprites.oam_addr, 4);
		sprites.increment_high_n();
		assert_eq!(sprites.oam_addr, 8);
		sprites.increment_low_m();
		sprites.increment_low_m();
		sprites.increment_low_m();
		assert_eq!(sprites.oam_addr, 11);
		// test m overflow
		sprites.increment_low_m();
		assert_eq!(sprites.oam_addr, 8);
		sprites.increment_high_n();
		assert_eq!(sprites.oam_addr, 12);

		sprites.oam_addr = 0;
		for _i in 0..63 {
			sprites.increment_high_n();
		}

		assert_eq!(sprites.oam_addr, 252);
		// test n overflow
		sprites.increment_high_n();
		assert_eq!(sprites.oam_addr, 0);
	}

	#[test]
	fn test_sprite_evaluation_basic() {
		let mut context = Context::new();
		let mut sprites = Sprites::new();
		// set scanline position
		context.vpos = 1;
		context.hpos = 65;
		// init oam test in range sprites
		init_oam(&mut sprites.primary_oam);
		set_sprite(&mut sprites.primary_oam, 0x1, 0x1, 0);
		set_sprite(&mut sprites.primary_oam, 0x9, 0x9, 1);
		set_sprite(&mut sprites.primary_oam, 0x2, 0x2, 2);
		set_sprite(&mut sprites.primary_oam, 0x3, 0x3, 3);
		set_sprite(&mut sprites.primary_oam, 0x4, 0x4, 4);

		for _i in 65..=256 {
			sprites.process_sprite_evaluation(&mut context);
			context.hpos += 1;
		}
		
		//println!("++++++SPRITE COUNT: {}", sprites.sprite_count);
		assert_eq!(sprites.sprite_count, 4);
	}

	#[test]
	fn test_sprite_evaluation_soam_overflow() {
		let mut context = Context::new();
		let mut sprites = Sprites::new();
		// set scanline position
		context.vpos = 1;
		context.hpos = 65;
		// init oam test in range sprites
		init_oam(&mut sprites.primary_oam);
		set_sprite(&mut sprites.primary_oam, 0x1, 0x1, 0);
		set_sprite(&mut sprites.primary_oam, 0x2, 0x2, 1);
		set_sprite(&mut sprites.primary_oam, 0x3, 0x3, 2);
		set_sprite(&mut sprites.primary_oam, 0x4, 0x4, 3);
		set_sprite(&mut sprites.primary_oam, 0x5, 0x5, 4);
		set_sprite(&mut sprites.primary_oam, 0x6, 0x6, 5);
		set_sprite(&mut sprites.primary_oam, 0x7, 0x7, 6);
		set_sprite(&mut sprites.primary_oam, 0x7, 0x8, 7);
		// more than 8 sprites on line should overflow
		set_sprite(&mut sprites.primary_oam, 0x7, 0x9, 8);
		set_sprite(&mut sprites.primary_oam, 0x7, 0x9, 9);


		for _i in 65..=256 {
			sprites.process_sprite_evaluation(&mut context);
			context.hpos += 1;
		}
		
		assert_eq!(sprites.sprite_count, 8);
		assert_eq!(context.status_reg.contains(StatusRegister::SPRITE_OVERFLOW), true);
	}

	#[test]
	fn test_sprite_evaluation_soam_no_overflow() {
		let mut context = Context::new();
		let mut sprites = Sprites::new();
		// set scanline position
		context.vpos = 200;
		context.hpos = 65;
		// init oam test in range sprites
		init_oam(&mut sprites.primary_oam);
		// fill oam entirely with sprites but only 8 on line
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 0);
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 1);
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 2);
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 3);
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 4);
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 5);
		set_sprite(&mut sprites.primary_oam, 200, 0x1, 6);
		for n in 7..31 {
			set_sprite(&mut sprites.primary_oam, 0x1, 0x8, n);
		}

		set_sprite(&mut sprites.primary_oam, 200, 0x1, 31);

		for _i in 65..=256 {
			sprites.process_sprite_evaluation(&mut context);
			context.hpos += 1;
		}
		
		assert_eq!(sprites.sprite_count, 8);
		assert_eq!(context.status_reg.contains(StatusRegister::SPRITE_OVERFLOW), false);
	}

	#[test]
	fn test_sprite_fetch_full() {
		let mut context = Context::new();
		let mut sprites = Sprites::new();
		// set scanline position
		context.vpos = 200;
		context.hpos = 65;
		// init oam test in range sprites
		init_oam(&mut sprites.primary_oam);
		// fill oam entirely with sprites but only 8 on line
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  0);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  1);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  2);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  3);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  4);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  5);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  6);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  7);

		for n in 8..31 {
			set_sprite(&mut sprites.primary_oam, 0x1, 0x8, n);
		}

		// sprite evaluation
		for _i in 65..=256 {
			sprites.process_sprite_evaluation(&mut context);
			context.hpos += 1;
		}

		// sprite tile fetch
		sprites.fetch_sprite_tile_data(&mut context);

		assert_eq!(sprites.sprites[7].valid_sprite, true);
		assert_eq!(sprites.sprites[7].sprite_line, 0);
		assert_eq!(sprites.sprites[7].xpos_counter, 1);
		assert_eq!(sprites.sprites[7].attribute, 2);
		assert_eq!(sprites.sprites[7].tile_index, 3);
	}

	#[test]
	fn test_sprite_fetch_partial() {
		let mut context = Context::new();
		let mut sprites = Sprites::new();
		// set scanline position
		context.vpos = 200;
		context.hpos = 65;
		// init oam test in range sprites
		init_oam(&mut sprites.primary_oam);
		// fill oam entirely with sprites but only 8 on line
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  0);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  1);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  2);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  3);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  4);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  5);
		set_sprite_full(&mut sprites.primary_oam, 200, 0x1, 0x2, 0x3,  6);

		for n in 7..31 {
			set_sprite(&mut sprites.primary_oam, 0x1, 0x8, n);
		}

		// sprite evaluation
		for _i in 65..=256 {
			sprites.process_sprite_evaluation(&mut context);
			context.hpos += 1;
		}

		// sprite tile fetch
		sprites.fetch_sprite_tile_data(&mut context);
		
		assert_eq!(sprites.sprites[6].valid_sprite, true);
		assert_eq!(sprites.sprites[6].sprite_line, 0);
		assert_eq!(sprites.sprites[6].xpos_counter, 1);
		assert_eq!(sprites.sprites[6].attribute, 2);
		assert_eq!(sprites.sprites[6].tile_index, 3);

		assert_eq!(sprites.sprites[7].valid_sprite, false);
		
		//println!("----SPRITE FETCH COUNT: {}----", sprites.sprite_count);
		//assert_eq!(sprites.sprite_count, 7);
	}

}