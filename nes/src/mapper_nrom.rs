use ::nes_rom::ines;
use std::ptr;
use super::mappers::{Mapper, NametableOffset, NametableType, POWER_ON_PALETTE};
use super::ppu_pinout;

pub struct MapperNrom {
    pub sram: Vec<u8>,
    pub vram: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub palette_ram: Vec<u8>,
    pub prg_size: u32,
    pub prg_mask: u16,
    pub nt_offset: NametableOffset,
}

impl MapperNrom {
    pub fn new() -> MapperNrom {
        let v = POWER_ON_PALETTE.to_vec();
        MapperNrom {
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

    pub fn from_ines(rom: &ines::Ines) -> MapperNrom {
        let mut nrom = MapperNrom::new();
        nrom.prg_size = rom.prg_rom_size;
        nrom.prg_rom = vec![0; (rom.prg_rom_size + 1) as usize];
        // check if rom is 16K or 32K
        if rom.prg_rom_size == 16384 {
            nrom.prg_mask = 0xBFFF;
        }
        else {
            nrom.prg_mask = 0xFFFF;
        }

        match rom.nametable_mirroring {
            ines::NametableMirroring::Horizontal => {
                nrom.nt_offset = NametableOffset::from_nametable(NametableType::Horizontal);
            }
            ines::NametableMirroring::Vertical => {
                nrom.nt_offset = NametableOffset::from_nametable(NametableType::Vertical);
            }
            _ => panic!("Invalid NROM nametable mirroring: {:?}", rom.nametable_mirroring),
        }

        // copy prg data
        unsafe {
            let src_len = rom.prg_data.len();
            let dst_ptr = nrom.prg_rom.as_mut_ptr();
            let src_ptr = rom.prg_data.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
        }

        nrom.chr_rom = vec![0; (rom.chr_rom_size + 1) as usize];
        // nrom chr size must be 8K

        // copy chr data
        unsafe {
            let src_len = rom.chr_data.len();
            let dst_ptr = nrom.chr_rom.as_mut_ptr();
            let src_ptr = rom.chr_data.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
        }

        nrom
    }
}

impl Mapper for MapperNrom {

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

    fn read_pattern_table(&mut self, mut pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { 
        //0x0000 - 0x1FFF
        pinout.data = self.chr_rom[pinout.address as usize];
        pinout
    }

    fn write_pattern_table(&mut self, mut pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { //0x0000 - 0x1FFF
        pinout.data = self.chr_rom[pinout.address as usize];
        pinout
    }
    
    fn read_nametable(&mut self, mut pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout {
        /*
        nametables are mirrored 0x3000 - 0x3EFF for simplicity we mirror 0x3000 - 0x3FFF
        the memory map should check the addresses and call the appropriate read function,
        so it shouldn't matter
        */
        let addr =  pinout.address & 0xD000;
         match addr {
             // A
             0x2000..=0x23FF => { pinout.data = self.vram[(addr - self.nt_offset.nt_a) as usize]; },
             // B
             0x2400..=0x27FF => { pinout.data = self.vram[(addr - self.nt_offset.nt_b) as usize]; },
             // C
             0x2800..=0x2BFF => { pinout.data = self.vram[(addr - self.nt_offset.nt_c) as usize]; },
             // D
             0x2C00..=0x2FFF => { pinout.data = self.vram[(addr - self.nt_offset.nt_d) as usize]; },
             _ => panic!("NROM PPU read out of bounds: {}", pinout.address),
         }

         pinout
    }


    fn write_nametable(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout {
        let addr =  pinout.address & 0xD000;
        match addr {
            // A
            0x2000..=0x23FF => { self.vram[(addr - self.nt_offset.nt_a) as usize] = pinout.data; },
            // B
            0x2400..=0x27FF => { self.vram[(addr - self.nt_offset.nt_b) as usize] = pinout.data; },
            // C
            0x2800..=0x2BFF => {self.vram[(addr - self.nt_offset.nt_c) as usize] = pinout.data; },
            // D
            0x2C00..=0x2FFF => { self.vram[(addr - self.nt_offset.nt_d) as usize] = pinout.data; },
            _ => panic!("NROM PPU write out of bounds: {}", pinout.address),
        }

        pinout
    }

    fn read_palette(&mut self, mut pinout: ppu_pinout::Pinout, forced_vblank: bool) -> ppu_pinout::Pinout { 
        /* 
        Addresses $3F04/$3F08/$3F0C can contain unique data, though these values are not used by the PPU when normally rendering
        (since the pattern values that would otherwise select those cells select the backdrop color instead)
        They can still be shown using the background palette hack during forced vblank
        */
        let addr = pinout.address & 0xFFE0;

        if !forced_vblank {
            match addr {
                0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C => { pinout.data = self.palette_ram[0x00] }
                _ => { pinout.data = self.palette_ram[addr as usize] }
            }
        }
        else {
            match addr {
                0x10 => { pinout.data = self.palette_ram[0x00] }
                0x14 => { pinout.data = self.palette_ram[0x04] }
                0x18 => { pinout.data = self.palette_ram[0x08] }
                0x1C => { pinout.data = self.palette_ram[0x0C] }
                _ => { pinout.data = self.palette_ram[addr as usize] }
            }
        }
        
        pinout
    }

    fn write_palette(&mut self, pinout: ppu_pinout::Pinout) -> ppu_pinout::Pinout { 
        /*
        Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C.
        Note that this goes for writing as well as reading
        */
        let addr = pinout.address & 0xFFE0;
        match addr {
            0x10 => { self.palette_ram[0x00] = pinout.data; }
            0x14 => { self.palette_ram[0x04] = pinout.data; }
            0x18 => { self.palette_ram[0x08] = pinout.data; }
            0x1C => { self.palette_ram[0x0C] = pinout.data; }
            _ => { self.palette_ram[addr as usize] = pinout.data; }
        }

        pinout
    }

    fn poke_prg(&mut self, addr: u16, data: u8) {
        let addr = (addr & self.prg_mask) - 0x8000;
        self.prg_rom[addr as usize] = data;
    }

    fn peek_pattern_table(&mut self, addr: u16) -> u8 {
        //0x0000 - 0x1FFF
        self.chr_rom[addr as usize]
    }

    fn peek_palette(&mut self, addr: u16) -> u8 {
        match addr {
            0x04 | 0x08 | 0x0C | 0x10 | 0x14 | 0x18 | 0x1C => { self.palette_ram[0x00] }
            _ => { self.palette_ram[addr as usize] }
        }
    }

    fn peek_nametable(&mut self, addr: u16) ->u8 {
        let addr =  addr & 0xD000;
         match addr {
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






