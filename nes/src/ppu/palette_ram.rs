

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
}