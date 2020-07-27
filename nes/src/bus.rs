use super::dma::{Dma, ApuDmaInterconnect};
use super::mappers::Mapper;

/*
    CPU memory map:
        |                 |    MOST     |      |
        |      RANGE      | SIGNIFICANT | SIZE |              CONTENTS
        |                 |   NIBBLE    |      |
        ----------------------------------------------------------------------------
        | 0x0000...0x07FF | 0000...0000 |  2kb | RAM
        | 0x0800...0x1FFF | 0000...0001 |  6kb | mirrors of RAM
        | 0x2000...0x2007 | 0010...0010 |   8b | I/O registers (PPU, 8 registers)
        | 0x2008...0x3FFF | 0010...0011 |      | mirrors of I/O registers (PPU)
        | 0x4000...0x401F | 0100...0100 |  32b | I/O registers (APU, DMA, Joypads)
        | 0x4020...0x5FFF | 0100...0101 |< 8kb | expansion ROM
        | 0x6000...0x7FFF | 0110...0111 |  8kb | work/save RAM
        | 0x8000...0xBFFF | 1000...1011 | 16kb | PRG-ROM lower bank
        | 0xC000...0xFFFF | 1100...1111 | 16kb | PRG-ROM upper bank
    Whole 0x4020...0xFFFF is mapped to the cartridge.
*/

//=========================================================
// CPU Bus
//=========================================================
pub struct CpuBus<'a> {
    mapper: &'a mut dyn Mapper,
    dma: &'a mut Dma,
    // TODO PPU, APU, Controller
}

impl<'a> CpuBus<'a> {
    pub fn new(mapper: &'a mut dyn Mapper, dma: &'a mut Dma) -> CpuBus<'a> {
        CpuBus {
            mapper: mapper,
            dma: dma
        }
    }
}

impl<'a> mos::bus::Bus for CpuBus<'a> {
    fn read(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x8000..=0xFFFF => { pinout = self.mapper.read_prg(pinout); }
            0x0000..=0x1FFF => { pinout = self.mapper.read_internal_ram(pinout); }
            0x4020..=0x5FFF => { pinout = self.mapper.read_expansion_rom(pinout); }
            0x6000..=0x7FFF => { pinout = self.mapper.read_wram(pinout); }
            _ => { /* open bus */ }
        }

        pinout
    }

    fn write(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x8000..=0xFFFF => { pinout = self.mapper.write_prg(pinout); }
            0x0000..=0x1FFF => { pinout = self.mapper.write_internal_ram(pinout); }
            0x4020..=0x5FFF => { pinout = self.mapper.write_expansion_rom(pinout); }
            0x6000..=0x7FFF => { pinout = self.mapper.write_wram(pinout); }
            0x4014 => self.dma.oam_execute(pinout.data),
            _ => { /* open bus */ }
        }

        pinout
    }
}

//==================================================
// DMA bus
//===================================================
pub struct DmaBus<'a> {
    mapper: &'a mut dyn Mapper,
    // TODO PPU, APU, Controller
}

impl<'a> DmaBus<'a> {
    pub fn new(mapper: &'a mut dyn Mapper) -> DmaBus<'a> {
        DmaBus {
            mapper: mapper,
        }
    }
}

impl<'a> mos::bus::Bus for DmaBus<'a> {
    fn read(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x8000..=0xFFFF => { pinout = self.mapper.read_prg(pinout); }
            0x0000..=0x1FFF => { pinout = self.mapper.read_internal_ram(pinout); }
            0x4020..=0x5FFF => { pinout = self.mapper.read_expansion_rom(pinout); }
            0x6000..=0x7FFF => { pinout = self.mapper.read_wram(pinout); }
            _ => { /* open bus */ }
        }

        pinout
    }

    fn write(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x8000..=0xFFFF => { pinout = self.mapper.write_prg(pinout); }
            0x0000..=0x1FFF => { pinout = self.mapper.write_internal_ram(pinout); }
            0x4020..=0x5FFF => { pinout = self.mapper.write_expansion_rom(pinout); }
            0x6000..=0x7FFF => { pinout = self.mapper.write_wram(pinout); }
            _ => { /* open bus */ }
        }

        pinout
    }
}

impl<'a> ApuDmaInterconnect for DmaBus<'a> {
    fn update_dmc_sample(&mut self, _sample: u8) {
        // TODO update APU
    }
}
