

// Groups PPUSCROLL and PPUADDR to handle VRAM logic
#[derive(Debug, Clone, Copy)]
pub struct AddrReg {
    pub v: u16,     // 15 bit current vram address
    pub t: u16,     // 15 bit temporary vram address, address of top-left of screen
    pub x: u16,     // Fixe x scroll
    pub w: bool,    // Write toggle
}

impl AddrReg {
    pub fn new() -> Self {
        AddrReg {
            v: 0,
            t: 0,
            x: 0,
            w: false,
        }
    }

    pub fn io_write_2006(&mut self, data: u8) {
        // First write
        if self.w == false {
            self.t = (self.t & 0xC0FF) | ((data as u16) << 8);
            // Clear bit 15 and 16, v has 15 bits but ppu address bus is only 14 bits
            self.t = self.t & 0x3FFF;
        }
        // Second write
        else {
            self.t = (self.t & 0xFF00) | (data as u16);
            self.v = self.t;
        }

        self.w = !self.w;
    }

    pub fn io_write_2005(&mut self, data: u8) {
        // First write
        if self.w == false {
           self.x = (data as u16) & 0x07;
           self.t = (self.t & 0xFFE0) | ((data as u16) >> 3);
        }
        // Second write
        else {
            let cba = ((data & 0x07) as u16) << 12;
            let hgfed = ((data >> 3) as u16) << 5;

            self.t = (self.t & 0x8FFF) | cba;
            self.t = (self.t & 0xFC1F) | hgfed;
        }

        self.w = !self.w;
    }

    pub fn increment(&mut self, inc: u16) {
        self.v = self.v.wrapping_add(inc);
    }

    pub fn coarse_x_increment(&mut self) {
        if (self.v & 0x001F) == 31 {    // If coarse x == 31
            self.v &= !0x001F;        // Set coarse x = 0
            self.v ^= 0x0400;           // switch horizontal nametable
        }
        else {
            self.v += 1;                // Increment coarse x
        }
    }

    pub fn y_increment(&mut self) {
        if (self.v & 0x7000) != 0x7000 {                // If fine y < 7
            self.v += 0x1000;                           // Increment fine y
        }
        else {
            self.v &= !0x7000;                          // Fine y = 0
            let mut y = (self.v & 0x03E0) >> 5;    // Let y = coarse y
            if y == 29 {
                y = 0;
                self.v ^= 0x0800;                       // Switch vertical nametable
            }
            else if y ==31 {
                y = 0;                                  // Coarse Y = 0, nametable not switched
            }
            else {
                y += 1;                                 // Increment coarse y
            }

            self.v = (self.v & !0x03E0) | (y << 5);     // Put coarse Y back into v
        }
    }
}

bitflags! {
    // 7  bit  0
    // ---- ----
    // VPHB SINN
    // |||| ||||
    // |||| ||++- Base nametable address
    // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
    // |||| |     (0: add 1, going across; 1: add 32, going down)
    // |||| +---- Sprite pattern table address for 8x8 sprites
    // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
    // |||+------ Background pattern table address (0: $0000; 1: $1000)
    // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
    // |+-------- PPU master/slave select
    // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
    // +--------- Generate an NMI at the start of the
    //            vertical blanking interval (0: off; 1: on)
    pub struct ControlRegister: u8 {
        const NAMETABLE0              = 0b00000001;
        const NAMETABLE1              = 0b00000010;
        const VRAM_ADD_INCREMENT      = 0b00000100;
        const SPRITE_PATTERN_ADDR     = 0b00001000;
        const BACKROUND_PATTERN_ADDR  = 0b00010000;
        const SPRITE_SIZE             = 0b00100000;
        const MASTER_SLAVE_SELECT     = 0b01000000;
        const GENERATE_NMI            = 0b10000000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0x00)
    }

    pub fn vram_addr_increment(&self) -> u16 {
        match self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            true => 32,
            false => 1,
        }
    }

    pub fn base_nametable_address(&self) -> u16 {
        match (self.contains(ControlRegister::NAMETABLE1), self.contains(ControlRegister::NAMETABLE0)) {
            (false, false) => 0x2000,
            (false, true) => 0x2400,
            (true, false) => 0x2800,
            (true, true) => 0x2C00,
        }
    }

    pub fn io_write(&mut self, data: u8) {
        self.bits = data;
    }
}

bitflags! {
    // 7  bit  0
    // ---- ----
    // BGRs bMmG
    // |||| ||||
    // |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
    // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
    // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    // |||| +---- 1: Show background
    // |||+------ 1: Show sprites
    // ||+------- Emphasize red
    // |+-------- Emphasize green
    // +--------- Emphasize blue
    pub struct MaskRegister: u8 {
        const GREYSCALE               = 0b00000001;
        const LEFTMOST_8PXL_BACKGROUND  = 0b00000010;
        const LEFTMOST_8PXL_SPRITE      = 0b00000100;
        const SHOW_BACKGROUND         = 0b00001000;
        const SHOW_SPRITES            = 0b00010000;
        //const EMPHASISE_RED           = 0b00100000;
        //const EMPHASISE_GREEN         = 0b01000000;
        //const EMPHASISE_BLUE          = 0b10000000;
    }
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0x00)
    }

    pub fn emphasis_mask(&self) -> u16 {
        // emphasis bits make up bits 6,7,8 of PPU color index
        ((self.bits & 0xE0) as u16) << 1
    }

    pub fn io_write(&mut self, data: u8) {
        self.bits = data;
    }
}

bitflags! {
    // 7  bit  0
    // ---- ----
    // VSO. ....
    // |||| ||||
    // |||+-++++- Least significant bits previously written into a PPU register
    // |||        (due to register not being updated for this address)
    // ||+------- Sprite overflow. The intent was for this flag to be set
    // ||         whenever more than eight sprites appear on a scanline, but a
    // ||         hardware bug causes the actual behavior to be more complicated
    // ||         and generate false positives as well as false negatives; see
    // ||         PPU sprite evaluation. This flag is set during sprite
    // ||         evaluation and cleared at dot 1 (the second dot) of the
    // ||         pre-render line.
    // |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    // |          a nonzero background pixel; cleared at dot 1 of the pre-render
    // |          line.  Used for raster timing.
    // +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
    //            Set at dot 1 of line 241 (the line *after* the post-render
    //            line); cleared after reading $2002 and at dot 1 of the
    //            pre-render line.
    pub struct StatusRegister: u8 {
        //const NOTUSED          = 0b00000001;
        //const NOTUSED2         = 0b00000010;
        //const NOTUSED3         = 0b00000100;
        //const NOTUSED4         = 0b00001000;
        //const NOTUSED5         = 0b00010000;
        const SPRITE_OVERFLOW  = 0b00100000;
        const SPRITE_ZERO_HIT  = 0b01000000;
        const VBLANK_STARTED   = 0b10000000;
    }
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from_bits_truncate(0x00)
    }

    pub fn io_read(&mut self, io_latch: u8) -> u8 {
        // Contains least significant bits previously written into a PPU register
        self.bits | (io_latch & 0xE0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addr_reg() {
        let mut addr_reg = AddrReg {
            v: 0,
            t: 0,
            x: 0,
            w: false,
        };

        addr_reg.io_write_2005(0xFF);
        assert_eq!(addr_reg.t, 31);
        assert_eq!(addr_reg.x, 7);
        addr_reg.io_write_2005(0xFF);
        assert_eq!(addr_reg.t, 0x73FF);

        addr_reg.v = 0;
        addr_reg.t = 0;
        addr_reg.x = 0;
        addr_reg.w = false;

        addr_reg.io_write_2006(0xFF);
        assert_eq!(addr_reg.t, 0x3F00);
        addr_reg.io_write_2006(0xFF);
        assert_eq!(addr_reg.t, 0x3FFF);
        assert_eq!(addr_reg.v, addr_reg.t);
    }

}