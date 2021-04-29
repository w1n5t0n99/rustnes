use super::dma::{Dma, ApuDmaInterconnect};
use super::mappers::Mapper;
use super::ppu::rp2c02::Rp2c02;
use super::controllers::NesControllers;


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
    ppu: &'a mut Rp2c02,
    controllers: &'a mut NesControllers,
    // TODO PPU, APU, Controller
}

impl<'a> CpuBus<'a> {
    pub fn new(mapper: &'a mut dyn Mapper, dma: &'a mut Dma, ppu: &'a mut Rp2c02, controllers: &'a mut NesControllers) -> CpuBus<'a> {
        CpuBus {
            mapper: mapper,
            dma: dma,
            ppu: ppu,
            controllers: controllers,
        }
    }
}

impl<'a> mos::bus::Bus for CpuBus<'a> {
    fn read(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x0000..=0x1fff => { pinout = self.mapper.read_cpu_internal_ram(pinout); }
            0x4020..=0x5fff => { pinout = self.mapper.read_cpu_exp(pinout); }
            0x6000..=0x7fff => { pinout = self.mapper.read_cpu_wram(pinout); }
            0x8000..=0xffff => { pinout = self.mapper.read_cpu_prg(pinout); }
            0x2000..=0x3FFF => {
                match pinout.address & 0x07 {
                    0 => { pinout = self.ppu.read_port(pinout); }
                    1 => { pinout = self.ppu.read_port(pinout); }
                    2 => { pinout = self.ppu.read_ppustatus(pinout); }
                    3 => { pinout = self.ppu.read_port(pinout); }
                    4 => { pinout = self.ppu.read_oamdata(pinout); }
                    5 => { pinout = self.ppu.read_port(pinout); }
                    6 => { pinout = self.ppu.read_port(pinout); }
                    7 => { pinout = self.ppu.read_ppudata(pinout); }
                    _ => { panic!("Cpu Bus - PPU address out of bounds"); }
                }
            }
            0x4000..=0x4015 => {
                //APU
                //TODO implement
                pinout.data = 0;
            }
            0x4016 => {
                pinout = self.controllers.read_4016(pinout);
            }
            0x4017 => {
                pinout = self.controllers.read_4017(pinout);
            }
            0x4018..=0x401F => {
                // The range $4018-$401F does nothing on a retail NES. It was intended for 2A03 functionality that never made it to production
            }
            _ => { /* open bus */ }
        }

        pinout
    }

    fn write(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x0000..=0x1fff => { pinout = self.mapper.write_cpu_internal_ram(pinout); }
            0x4020..=0x5fff => { pinout = self.mapper.write_cpu_exp(pinout); }
            0x6000..=0x7fff => { pinout = self.mapper.write_cpu_wram(pinout); }
            0x8000..=0xffff => { pinout = self.mapper.write_cpu_prg(pinout); }
            0x4014 => { self.dma.oam_execute(pinout.data) },
            0x2000..=0x3FFF => {
                match pinout.address & 0x07 {
                    0 => { pinout = self.ppu.write_ppuctrl(pinout); }
                    1 => { pinout = self.ppu.write_ppumask(pinout); }
                    2 => { pinout = self.ppu.write_ppustatus(pinout); }
                    3 => { pinout = self.ppu.write_oamaddr(pinout); }
                    4 => { pinout = self.ppu.write_oamdata(pinout); }
                    5 => { pinout = self.ppu.write_ppuscroll(pinout); }
                    6 => { pinout = self.ppu.write_ppuaddr(pinout); }
                    7 => { pinout = self.ppu.write_ppudata(pinout); }
                    _ => { panic!("Cpu Bus - PPU address out of bounds"); }
                }
            }
            0x4000..=0x4015 => {
                //APU + IO
                //TODO implement
            }
            0x4016 => {
                pinout = self.controllers.write_4016(pinout);
            }
            0x4018..=0x401F => {
                // The range $4018-$401F does nothing on a retail NES. It was intended for 2A03 functionality that never made it to production
            }
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
    ppu: &'a mut Rp2c02,
    controllers: &'a mut NesControllers,
    // TODO PPU, APU
}

impl<'a> DmaBus<'a> {
    pub fn new(mapper: &'a mut dyn Mapper,  ppu: &'a mut Rp2c02, controllers: &'a mut NesControllers) -> DmaBus<'a> {
        DmaBus {
            mapper: mapper,
            ppu: ppu,
            controllers: controllers,
        }
    }
}

impl<'a> mos::bus::Bus for DmaBus<'a> {
    fn read(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x0000..=0x1fff => { pinout = self.mapper.read_cpu_internal_ram(pinout); }
            0x4020..=0x5fff => { pinout = self.mapper.read_cpu_exp(pinout); }
            0x6000..=0x7fff => { pinout = self.mapper.read_cpu_wram(pinout); }
            0x8000..=0xffff => { pinout = self.mapper.read_cpu_prg(pinout); }
            0x2000..=0x3FFF => {
                match pinout.address & 0x07 {
                    0 => { pinout = self.ppu.read_port(pinout); }
                    1 => { pinout = self.ppu.read_port(pinout); }
                    2 => { pinout = self.ppu.read_ppustatus(pinout); }
                    3 => { pinout = self.ppu.read_port(pinout); }
                    4 => { pinout = self.ppu.read_oamdata(pinout); }
                    5 => { pinout = self.ppu.read_port(pinout); }
                    6 => { pinout = self.ppu.read_port(pinout); }
                    7 => { pinout = self.ppu.read_ppudata(pinout); }
                    _ => { panic!("Cpu Bus - PPU address out of bounds"); }
                }
            }
            0x4016 => {
                pinout = self.controllers.read_4016(pinout);
            }
            0x4017 => {
                pinout = self.controllers.read_4017(pinout);
            }
            _ => { /* open bus */ }
        }

        pinout
    }

    fn write(&mut self, mut pinout: mos::Pinout) -> mos::Pinout {
        match pinout.address {
            0x0000..=0x1fff => { pinout = self.mapper.write_cpu_internal_ram(pinout); }
            0x4020..=0x5fff => { pinout = self.mapper.write_cpu_exp(pinout); }
            0x6000..=0x7fff => { pinout = self.mapper.write_cpu_wram(pinout); }
            0x8000..=0xffff => { pinout = self.mapper.write_cpu_prg(pinout); }
            0x2000..=0x3FFF => {
                match pinout.address & 0x07 {
                    0 => { pinout = self.ppu.write_ppuctrl(pinout); }
                    1 => { pinout = self.ppu.write_ppumask(pinout); }
                    2 => { pinout = self.ppu.write_ppustatus(pinout); }
                    3 => { pinout = self.ppu.write_oamaddr(pinout); }
                    4 => { pinout = self.ppu.write_oamdata(pinout); }
                    5 => { pinout = self.ppu.write_ppuscroll(pinout); }
                    6 => { pinout = self.ppu.write_ppuaddr(pinout); }
                    7 => { pinout = self.ppu.write_ppudata(pinout); }
                    _ => { panic!("Cpu Bus - PPU address out of bounds"); }
                }
            }
            0x4016 => {
                pinout = self.controllers.write_4016(pinout);
            }
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
