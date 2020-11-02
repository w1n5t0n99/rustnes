use ::nes_rom::ines;
use std::ptr;

use super::mappers::{Mapper, NametableOffset, NametableType, POWER_ON_PALETTE};
use super::ppu;

pub struct MapperDebug {
    pub sram: Vec<u8>,
    pub vram: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub palette_ram: Vec<u8>,
    pub prg_size: u32,
    pub prg_mask: u16,
    pub nt_offset: NametableOffset,
}

impl MapperDebug {
    pub fn new() -> MapperDebug {
        let v = POWER_ON_PALETTE.to_vec();
        MapperDebug {
            sram: vec![0; 0x800],
            vram: vec![0; 0x1000],
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            palette_ram: v,
            prg_size: 0,
            prg_mask: 0,
            nt_offset: NametableOffset::new(0, 0, 0, 0),
        }
    }

    pub fn init() -> MapperDebug {
        let mut nrom = MapperDebug::new();
        nrom.nt_offset = NametableOffset::from_nametable(NametableType::Vertical);
        nrom.prg_mask = 0xBFFF;

        nrom.prg_rom = vec![0; 16384];
        nrom.chr_rom = vec![0; 8192];

        let mut pdata = 0_u8;
        for pattern in nrom.chr_rom.chunks_exact_mut(16) {
            for elem in pattern.iter_mut() {
                *elem += pdata;
            }

            pdata += 1;
        }

        for nt in 0..960 {
            nrom.vram[nt as usize] = nt as u8;
        }

        for nt in 0x400..(0x400+960) {
            nrom.vram[nt as usize] = nt as u8;
        }

        nrom
    }
}

impl Mapper for MapperDebug {

    fn set_reset(&mut self, addr: u16) {
        let hb = (addr >> 8) as u8;
        let lb = addr as u8;

        let mut rst_vec = (0xFFFD & self.prg_mask) - 0x8000;
        self.prg_rom[rst_vec as usize] = hb;
        rst_vec = (0xFFFC & self.prg_mask) - 0x8000;
        self.prg_rom[rst_vec as usize] = lb;
    }

    fn read_internal_ram(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        pinout.data = self.sram[(pinout.address & 0x7FF) as usize];
        pinout
    }

    fn write_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        self.sram[(pinout.address & 0x7FF) as usize] = pinout.data;
        pinout
    }

    fn read_expansion_rom(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // nrom does not implement - open bus
        pinout
    }

    fn write_expansion_rom(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // nrom does not implement - open bus
        pinout
    }

    fn read_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // nrom does not implement - open bus
        pinout
    }

    fn write_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        // nrom does not implement - open bus
        pinout
    }

    fn read_prg(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        let addr = (pinout.address & self.prg_mask) - 0x8000;
        pinout.data = self.prg_rom[addr as usize];
        pinout
    }

    fn write_prg(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        // ROM can't tell whether you're doing a read or a write. It just sees "chip enabled" and "$8000", assumes everything is a read
        // (Some boards take special ROMs that can tell a read from a write. CNROM isn't one of them. AOROM is.)
        let addr = (pinout.address & self.prg_mask) - 0x8000;
        pinout.data = self.prg_rom[addr as usize];
        pinout
    }

    fn read_ppu(&mut self, mut ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout) {
        let addr = ppu_pinout.address();

        match addr {
            // CHR ROM
            0x000..=0x1FFF => { ppu_pinout.set_data(self.chr_rom[addr as usize]); },
             // NT A
             0x2000..=0x23FF => { ppu_pinout.set_data(self.vram[(addr - self.nt_offset.nt_a) as usize]); },
             // NT B
             0x2400..=0x27FF => { ppu_pinout.set_data(self.vram[(addr - self.nt_offset.nt_b) as usize]); },
             // NT C
             0x2800..=0x2BFF => { ppu_pinout.set_data(self.vram[(addr - self.nt_offset.nt_c) as usize]); },
             // NT D
             0x2C00..=0x2FFF => { ppu_pinout.set_data(self.vram[(addr - self.nt_offset.nt_d) as usize]); },
             _ => panic!("NROM PPU read out of bounds: {}", addr),

        }

        (ppu_pinout, cpu_pinout)
    }

    fn write_ppu(&mut self, ppu_pinout: ppu::Pinout, cpu_pinout: mos::Pinout) -> (ppu::Pinout, mos::Pinout) {
        let addr = ppu_pinout.address();

        match addr {
            // CHR ROM
            0x000..=0x1FFF => { /* returns whatevers on the bus already */ },
             // NT A
             0x2000..=0x23FF => { self.vram[(addr - self.nt_offset.nt_a) as usize] = ppu_pinout.data(); },
             // NT B
             0x2400..=0x27FF => { self.vram[(addr - self.nt_offset.nt_b) as usize] = ppu_pinout.data(); },
             // NT C
             0x2800..=0x2BFF => { self.vram[(addr - self.nt_offset.nt_c) as usize] = ppu_pinout.data(); },
             // NT D
             0x2C00..=0x2FFF => { self.vram[(addr - self.nt_offset.nt_d) as usize] = ppu_pinout.data(); },
             _ => panic!("NROM PPU write out of bounds: {}", addr),

        }

        (ppu_pinout, cpu_pinout)
    }

    fn peek_ppu(&mut self, addr: u16) -> u8 {
        match addr {
            // CHR
            0x0000..=0x1FFF => { self.chr_rom[addr as usize] }
            // A
            0x2000..=0x23FF => { self.vram[(addr - self.nt_offset.nt_a) as usize] },
            // B
            0x2400..=0x27FF => { self.vram[(addr - self.nt_offset.nt_b) as usize] },
            // C
            0x2800..=0x2BFF => { self.vram[(addr - self.nt_offset.nt_c) as usize] },
            // D
            0x2C00..=0x2FFF => { self.vram[(addr - self.nt_offset.nt_d) as usize] },
            _ => panic!("NROM PPU read out of bounds: {}", addr),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_nametable_read() {
        //assert_eq!(add(1, 2), 3);
    }

}






