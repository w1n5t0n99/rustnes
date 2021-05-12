

#[derive(Clone, Copy)]
pub struct PaletteRam {
    ram: [u8; 32],
}

impl PaletteRam {
    pub fn from_power_on() -> PaletteRam {
        // unspecified at startup
        PaletteRam {
            ram: [0; 32],
        }
    }

    pub fn from_reset(&self) -> PaletteRam {
        // unchanged at reset
        self.clone()
    }

    pub fn read(&self, address: u16) -> u8 {
        let address = address & 0x1F;
        match address {
            0x10 => self.ram[0x00],
            0x14 => self.ram[0x04],
            0x18 => self.ram[0x08],
            0x1C => self.ram[0x0C],
            _ => self.ram[address as usize],
        }
    }

    pub fn read_during_render(&self, address: u16) -> u8 {
        let address = address & 0x1F;        
        match address {
            0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C => self.ram[0x00],
            _ => self.ram[address as usize],
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let address = address & 0x1F;
        // palette ram contains a 6 bit value
        match address {
            0x10 => { self.ram[0x00] = (data  & 0x3F); }
            0x14 => { self.ram[0x04] = (data  & 0x3F); }
            0x18 => { self.ram[0x08] = (data  & 0x3F); }
            0x1C => { self.ram[0x0C] = (data  & 0x3F); }
            _ => { self.ram[address as usize] = (data  & 0x3F); }
        }
    }
}