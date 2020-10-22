
#[derive(Debug, Clone, Copy)]
pub struct AddrReg {
    pub v: u16,     // 15 bit current vram address
    pub t: u16,     // 15 bit temporary vram address, address of top-left of screen
    pub x: u16,     // Fixe x scroll
    pub w: bool,    // Write toggle
}

impl AddrReg {
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
            let cba = ((data & 0x07) as u16) << 13;
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