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
enum EvalState {
    SpriteFetchY,
    SpriteWriteY,
    SpriteFetchTileIndex,
    SpriteWriteTileIndex,
    SpriteFetchAttribute,
    SpriteWriteAttribute,
    SpriteFetchX,
    SpriteWriteX,
    OverflowFetchY,
    OverflowWriteY,
    OverflowFetchTileIndex,
    OverflowWriteTileIndex,
    OverflowFetchAttribute,
    OverflowWriteAttribute,
    OverflowFetchX,
    OverflowWriteX,
    FinishedRead,
	FinishedWrite,
}

#[derive(Debug, Clone, Copy)]
enum FetchState {
	ReadY,
	ReadTileIndex,
	ReadAttribue,
	ReadX,
	Dummy0,
	Dummy1,
	Dummy2,
	Dummy3,
}

#[derive(Debug, Clone, Copy)]
pub struct Sprites {
    pub primary_oam: [u8; 256],
    pub secondary_oam: [u8; 32],
    pattern_queue_left: [u8; 8],
    pattern_queue_right: [u8; 8],
    attribute_latches: [u8; 8],
    xpos_counters: [u8; 8],
	oam_addr: usize,
	secondary_oam_addr: usize,
	oam_data_buffer: u8,
	tile_data_buffer: u8,
	y_data_buffer: u8,
	sprite_index: usize,
	eval_state: EvalState,
	fetch_state: FetchState,
	sprite_0_evaluated: bool,
	sprite_0_visible: bool,
	sprite_count: u8,

}

impl Sprites {
	pub fn new() -> Self {
		Sprites {
			primary_oam: [0xFF; 256],
			secondary_oam: [0xFF; 32],
			pattern_queue_left: [0; 8],
			pattern_queue_right: [0; 8],
			attribute_latches: [0; 8],
			xpos_counters: [0; 8],
			oam_addr: 0,
			secondary_oam_addr: 0,
			oam_data_buffer: 0,
			tile_data_buffer: 0xFF,
			y_data_buffer: 0xFF,
			sprite_index: 0,
			eval_state: EvalState::SpriteFetchY,
			fetch_state: FetchState::ReadY,
			sprite_0_evaluated: false,
			sprite_0_visible: false,
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
		else if context.hpos == 256 {
			self.end_evaluation();
		}

		match self.eval_state {
			EvalState::SpriteFetchY => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::SpriteWriteY;
			}
			EvalState::SpriteWriteY => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				if self.sprite_in_range(context, self.oam_data_buffer) {
					// copy remaining bytes for sprite
					self.secondary_oam_addr += 1;
					self.increment_low_m();
					self.eval_state = EvalState::SpriteFetchTileIndex;
				}
				else {
					self.increment_high_n();
					if (self.oam_addr & 0xFC) == 0 { self.eval_state = EvalState::FinishedRead; }
					else if self.secondary_oam_addr >= 0x20 { self.eval_state = EvalState::OverflowFetchY; }
					else { self.eval_state = EvalState::SpriteFetchY; }
				}
			}
			EvalState::SpriteFetchTileIndex => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::SpriteFetchTileIndex;
			}
			EvalState::SpriteWriteTileIndex => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				self.secondary_oam_addr += 1;
				self.increment_low_m();
				self.eval_state = EvalState::SpriteFetchAttribute;
			}
			EvalState::SpriteFetchAttribute => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::SpriteWriteAttribute;
			}
			EvalState::SpriteWriteAttribute => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				self.secondary_oam_addr += 1;
				self.increment_low_m();
				self.eval_state = EvalState::SpriteFetchAttribute;
			}
			EvalState::SpriteFetchX => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::SpriteWriteAttribute;
			}
			EvalState::SpriteWriteX => {
				self.secondary_oam[self.secondary_oam_addr] = self.oam_data_buffer;
				self.secondary_oam_addr += 1;
				self.increment_low_m();
				self.increment_high_n();
				if (self.oam_addr & 0xFC) == 0 { self.eval_state = EvalState::FinishedRead; }
				else if self.secondary_oam_addr >= 0x20 { self.eval_state = EvalState::OverflowFetchY; }
				else { self.eval_state = EvalState::SpriteFetchY; }
			}
			EvalState::FinishedRead => {
				// attempt (and fail) to copy OAM[n][0] into the next free slot in secondary OAM
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::FinishedWrite;
			}
			EvalState::FinishedWrite => {
				// a side effect of the OAM write disable signal is to turn writes to the secondary OAM into reads from it
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_high_n();
				self.eval_state = EvalState::FinishedRead;
			}
			EvalState::OverflowFetchY => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::OverflowWriteY;
			}
			EvalState::OverflowWriteY => {
				let buffer = self.oam_data_buffer;
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];

				if self.sprite_in_range(context, buffer) {
					//  If the value is in range, set the sprite overflow flag in $2002 and read the next 3 entries of OAM
					// (incrementing 'm' after each byte and incrementing 'n' when 'm' overflows); if m = 3, increment n
					context.status_reg.set(StatusRegister::SPRITE_OVERFLOW, true);
					self.increment_low_m();  
					self.eval_state = EvalState::SpriteFetchTileIndex;
				}
				else {
					//  If the value is not in range, increment n and m (without carry). If n overflows to 0, go to 4; otherwise go to 3
					self.increment_high_n();
					self.increment_low_m();
					if (self.oam_addr & 0xFC) == 0 { self.eval_state = EvalState::FinishedRead; }
					else { self.eval_state = EvalState::OverflowFetchY; }
				}
			}
			EvalState::OverflowFetchTileIndex => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::OverflowWriteTileIndex;
			}
			EvalState::OverflowWriteTileIndex => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_low_m();
				self.eval_state = EvalState::OverflowFetchAttribute;
			}
			EvalState::OverflowFetchAttribute => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::OverflowWriteAttribute;
			}
			EvalState::OverflowWriteAttribute => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_low_m();
				self.eval_state = EvalState::OverflowFetchX;
			}
			EvalState::OverflowFetchX => {
				self.oam_data_buffer = self.primary_oam[self.oam_addr];
				self.eval_state = EvalState::OverflowWriteX;
			}
			EvalState::OverflowWriteX => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr & 0x1F];
				self.increment_low_m();
				self.eval_state = EvalState::FinishedRead;
			}
		}
	}

	pub fn fetch_sprites(&mut self, context: &mut Context) {
		if context.hpos == 257 {
			self.begin_sprite_fetch();
		}

		match self.fetch_state {
			FetchState::ReadY => {
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr];
				self.y_data_buffer = self.oam_data_buffer;
				self.fetch_state = FetchState::ReadTileIndex;
			}
			FetchState::ReadTileIndex => {
				self.secondary_oam_addr += 1;
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr];
				self.tile_data_buffer = self.oam_data_buffer;
				self.fetch_state = FetchState::ReadAttribue;
			}
			FetchState::ReadAttribue => {
				self.secondary_oam_addr += 1;
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr];
				self.attribute_latches[self.sprite_index] = self.oam_data_buffer;

				// apply vertical flip
				if (self.oam_data_buffer & 0x80) > 0 && context.control_reg.large_sprite() {
					self.y_data_buffer ^= SPRITE_16X_FLIPMASK;
				}
				else if (self.oam_data_buffer & 0x80) > 0 {
					self.y_data_buffer ^= SPRITE_8X_FLIPMASK;
				}

				self.fetch_state = FetchState::ReadX;
			}
			FetchState::ReadX => {
				self.secondary_oam_addr += 1;
				self.oam_data_buffer = self.secondary_oam[self.secondary_oam_addr];
				self.xpos_counters[self.sprite_index] = self.oam_data_buffer;
				self.fetch_state = FetchState::Dummy0;
			}
			FetchState::Dummy0 => {
				self.fetch_state = FetchState::Dummy1;
			}
			FetchState::Dummy1 => {
				self.fetch_state = FetchState::Dummy2;
			}
			FetchState::Dummy2 => {
				self.fetch_state = FetchState::Dummy3;
			}
			FetchState::Dummy3 => {
				self.secondary_oam_addr += 1;
				self.fetch_state = FetchState::ReadY;
			}
		}
	}

	pub fn pattern0_address(&mut self, context: &mut Context) -> u16 {
        //let current_sprite_index= ((context.hpos - 1) >> 3) & 0x07;
        if context.control_reg.large_sprite() {
			((((self.tile_data_buffer as u16) & 1) << 12) | (((self.tile_data_buffer as u16) & 0xfe) << 4) | PATTERN0_OFFSET | ((self.y_data_buffer as u16) & 7) | (((self.y_data_buffer as u16) & 0x08) << 1)) & 0xffff
		}
		else {
			(context.control_reg.sprite_table_address()| (((self.tile_data_buffer as u16)) << 4) | PATTERN0_OFFSET | (self.y_data_buffer as u16)) & 0xffff
		}
    }

	pub fn pattern1_address(&mut self, context: &mut Context) -> u16 {
        //let current_sprite_index= ((context.hpos - 1) >> 3) & 0x07;
        if context.control_reg.large_sprite() {
			((((self.tile_data_buffer as u16) & 1) << 12) | (((self.tile_data_buffer as u16) & 0xfe) << 4) | PATTERN1_OFFSET | ((self.y_data_buffer as u16) & 7) | (((self.y_data_buffer as u16) & 0x08) << 1)) & 0xffff
		}
		else {
			(context.control_reg.sprite_table_address()| (((self.tile_data_buffer as u16)) << 4) | PATTERN1_OFFSET | (self.y_data_buffer as u16)) & 0xffff
		}
    }

	pub fn set_pattern0(&mut self, context: &mut Context, mut data: u8) {
		if self.sprite_index >= (self.sprite_count as usize) {
			//load pattern tables with transparent data
			self.pattern_queue_left[self.sprite_index] = 0;
		}
		else {
			if (self.attribute_latches[self.sprite_index] & 0x40) > 0 {
				// horizontal flip pattern
				data = REVERSE_BITS[data as usize];
			}

			self.pattern_queue_left[self.sprite_index] = data;
		}
    }

    pub fn set_pattern1(&mut self, context: &mut Context, mut data: u8) {
        if self.sprite_index >= (self.sprite_count as usize) {
			//load pattern tables with transparent data
			self.pattern_queue_right[self.sprite_index] = 0;
		}
		else {
			if (self.attribute_latches[self.sprite_index] & 0x40) > 0 {
				// horizontal flip pattern
				data = REVERSE_BITS[data as usize];
			}

			self.pattern_queue_right[self.sprite_index] = data;
		}
    }

	// called on cycle 65
	fn begin_evaluation(&mut self) {
		// reset sprite evaluation indices
		self.secondary_oam_addr = 0;
		self.eval_state = EvalState::SpriteFetchY;
		self.sprite_0_evaluated = false;
	}

	// called on cycle 256
	fn end_evaluation(&mut self) {
		self.sprite_0_visible = self.sprite_0_evaluated;
		self.sprite_count = (self.secondary_oam_addr as u8) >> 2;
	}

	// called on cycle 257
	fn begin_sprite_fetch(&mut self) {
		self.oam_addr = 0;
		self.secondary_oam_addr = 0;
		self.sprite_index = 0;
		self.tile_data_buffer = 0xFF;
		self.y_data_buffer = 0xFF;
		self.fetch_state = FetchState::ReadY;
	}

	fn sprite_in_range(&self, context: &Context, y_pos: u8) -> bool {
		if (y_pos as u16) >= context.vpos && y_pos < (context.control_reg.sprite_size()) {
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

}