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

    pub fn with_debug_values(nt_type: NametableType) -> MapperDebug {
        let mut nrom = MapperDebug::new();
        nrom.nt_offset = NametableOffset::from_nametable(nt_type);
        nrom.prg_mask = 0xBFFF;

        nrom.prg_rom = vec![0; 16384];
        nrom.chr_rom = vec![0; 8192];


        let mut pdata = 0_u8;
        for pattern in nrom.chr_rom.chunks_exact_mut(16) {
            for elem in pattern.iter_mut() {
                *elem = pdata;
            }

            if pdata == 0 { pdata = 0xFF; } else { pdata = 0; }
        }

       nrom.set_nt_mirroring(nt_type);
       nrom.set_nt_attribute(0x00);

        nrom
    }

    pub fn set_nt_mirroring(&mut self, nt_type: NametableType) {
        self.nt_offset = NametableOffset::from_nametable(nt_type);
        self.vram = vec![0; 0x1000];

        let mut index = 0_u8;
        for n in (self.nt_offset.nt_a - 0x2000)..((self.nt_offset.nt_a - 0x2000) + 0x400) {
            self.vram[n as usize] = index;
            index = index.wrapping_add(1);
        }

        index = 0;
        for n in (self.nt_offset.nt_b - 0x2000)..((self.nt_offset.nt_b - 0x2000) + 0x400) {
            self.vram[n as usize] = index;
            index = index.wrapping_add(1);
        }

        index = 0;
        for n in (self.nt_offset.nt_c - 0x2000)..((self.nt_offset.nt_c - 0x2000) + 0x400) {
            self.vram[n as usize] = index;
            index = index.wrapping_add(1);
        }

        index = 0;
        for n in (self.nt_offset.nt_d - 0x2000)..((self.nt_offset.nt_d - 0x2000) + 0x400) {
            self.vram[n as usize] = index;
            index = index.wrapping_add(1);
        }
    }

    pub fn set_nt_attribute(&mut self, value: u8) {
        for n in ((self.nt_offset.nt_a - 0x2000) + 0x3C0)..((self.nt_offset.nt_a - 0x2000) + 0x400) {
            self.vram[n as usize] = value;
        }

        for n in ((self.nt_offset.nt_b - 0x2000) + 0x3C0)..((self.nt_offset.nt_b - 0x2000) + 0x400) {
            self.vram[n as usize] = value;
        }

        for n in ((self.nt_offset.nt_c - 0x2000) + 0x3C0)..((self.nt_offset.nt_c - 0x2000) + 0x400) {
            self.vram[n as usize] = value;
        }

        for n in ((self.nt_offset.nt_d - 0x2000) + 0x3C0)..((self.nt_offset.nt_d - 0x2000) + 0x400) {
            self.vram[n as usize] = value;
        }
        
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
    fn test_nametable_vertical() {
        let mut mapper = MapperDebug::with_debug_values(NametableType::Vertical);
        //mapper.set_nt_mirroring(NametableType::Vertical);
        let mut cpu_pinout = mos::Pinout::new();
        let mut ppu_pinout = ppu::Pinout::new();

        ppu_pinout.set_address(0x2001);
        ppu_pinout.latch_address();
        ppu_pinout.set_address(0x2001);

        let p = mapper.read_ppu(ppu_pinout, cpu_pinout);
        ppu_pinout = p.0; 
        cpu_pinout = p.1;
        let d0 = ppu_pinout.data();

        ppu_pinout.set_address(0x2801);
        ppu_pinout.latch_address();
        ppu_pinout.set_address(0x2801);

        let p = mapper.read_ppu(ppu_pinout, cpu_pinout);
        ppu_pinout = p.0; 
        cpu_pinout = p.1;
        let d1 =ppu_pinout.data();

        assert_eq!(d0, d1);

        ppu_pinout.set_address(0x2401);
        ppu_pinout.latch_address();
        ppu_pinout.set_address(0x2401);

        let p = mapper.read_ppu(ppu_pinout, cpu_pinout);
        ppu_pinout = p.0; 
        cpu_pinout = p.1;
        let d2 =ppu_pinout.data();
        assert_ne!(d1, d2);

    }

    #[test]
    fn test_nametable_horizontal() {
        let mut mapper = MapperDebug::with_debug_values(NametableType::Horizontal);
        //mapper.set_nt_mirroring(NametableType::Horizontal);
        let mut cpu_pinout = mos::Pinout::new();
        let mut ppu_pinout = ppu::Pinout::new();

        ppu_pinout.set_address(0x2001);
        ppu_pinout.latch_address();
        ppu_pinout.set_address(0x2001);

        let p = mapper.read_ppu(ppu_pinout, cpu_pinout);
        ppu_pinout = p.0; 
        cpu_pinout = p.1;
        let d0 = ppu_pinout.data();

        ppu_pinout.set_address(0x2401);
        ppu_pinout.latch_address();
        ppu_pinout.set_address(0x2401);

        let p = mapper.read_ppu(ppu_pinout, cpu_pinout);
        ppu_pinout = p.0; 
        cpu_pinout = p.1;
        let d1 =ppu_pinout.data();

        assert_eq!(d0, d1);

        ppu_pinout.set_address(0x2801);
        ppu_pinout.latch_address();
        ppu_pinout.set_address(0x2801);

        let p = mapper.read_ppu(ppu_pinout, cpu_pinout);
        ppu_pinout = p.0; 
        cpu_pinout = p.1;
        let d2 =ppu_pinout.data();
        assert_ne!(d1, d2);

    }

}






