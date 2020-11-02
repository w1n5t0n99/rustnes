pub mod error;
mod dma;
mod mappers;
mod mapper_nrom;
mod mapper_debug;
mod bus;
mod ppu;
mod palette;

pub use error::NesError;
use mos::rp2a03::Rp2a03;
use dma::Dma;
use ppu::ppu_viewer::PpuViewer;

use std::fs::File;
use std::path::Path;
use ::nes_rom::ines;
use std::fmt;

#[macro_use]
extern crate bitflags;

pub struct Nes {
    cpu: Rp2a03,
    cpu_pinout: mos::Pinout,
    dma: Dma,
    mapper: Box<dyn mappers::Mapper>,
    ppu_viewer: PpuViewer,
}

impl Nes {
    pub fn from_power_on() -> Nes {
        let (cpu, cpu_pinout) = Rp2a03::from_power_on();
        Nes {
            cpu: cpu,
            cpu_pinout: cpu_pinout,
            dma: Dma::from_power_on(),
            mapper: Box::new(mappers::MapperNull),
            ppu_viewer: PpuViewer::new(),
        }
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), NesError> {
        // only accepting ines for now
        let ines_file = File::open(rom_path)?;
        let ines = ines::Ines::from_rom(ines_file)?;
        self.mapper = mappers::create_mapper(&ines)?;
        Ok(())
    }

    // place the starting address in the reset vector
    pub fn debug_reset(&mut self, addr: u16) {
        self.cpu.reset();
        self.dma.reset();
        self.mapper.set_reset(addr);
    }

    pub fn execute_cycle(&mut self) {
        /*
        The NES's master clock frequency is 21.477272 Mhz.
        The CPU divides it by 12, hence runs at 1.7897727 Mhz.
        The PPU divides it by 4, hence runs at 5.369318 Mhz.
        The APU divides it by 89490, hence runs at 239.996335 Hz.
        Since 12 / 4 = 3 there are 3 PPU clocks per 1 CPU clock.
        Since 89490 / 12 = 7457.5 there are 7457.5 CPU clocks per 1 APU clock.
        */
        {
            let mut bus = bus::CpuBus::new(&mut *self.mapper, &mut self.dma);
            self.cpu_pinout = self.cpu.tick(&mut bus, self.cpu_pinout);
        }

        {
            let mut bus = bus::DmaBus::new(&mut *self.mapper);
            self.cpu_pinout = self.dma.tick(&mut bus, self.cpu_pinout);
        }
    }

    pub fn framebuffer(&mut self) -> Vec<u32> {
        unimplemented!();
    }

    pub fn chr_framebuffer(&mut self) -> Vec<u32> {
        self.ppu_viewer.gen_chr_data(&mut *self.mapper);
        self.ppu_viewer.chr_buffer()
    }
}

impl fmt::Display for Nes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {  
        write!(f, "{} {} [{}]", self.cpu, self.dma, self.cpu_pinout)
    }
}

