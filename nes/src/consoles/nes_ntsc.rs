use super::*;
use crate::dma::Dma;
use crate::ppu::rp2c02::Rp2c02;
use crate::mappers;
use crate::mappers::Mapper;
use crate::controllers::{NesControllers, JoypadInput};
use crate::palette::*;
use crate::bus::*;
use mos::{Pinout, rp2a03::Rp2a03};

use std::fs::File;
use std::path::Path;
use std::io::Write;
use ::nes_rom::ines;
use std::fmt;

pub struct NesNtsc {
    cpu: Rp2a03,
    cpu_pinout: Pinout,
    dma: Dma,
    ppu: Rp2c02,
    controllers: NesControllers,
    mapper: Box<dyn Mapper>,
    pbuffer: Vec<u16>,
}

impl NesNtsc {
    pub fn new() -> Self {
        let (cpu, cpu_pinout) = Rp2a03::from_power_on();
        NesNtsc {
            cpu: cpu,
            cpu_pinout: cpu_pinout,
            dma: Dma::from_power_on(),
            ppu: Rp2c02::from_power_on(),
            controllers: NesControllers::from_power_on(),
            mapper: mappers::create_mapper_null(),
            pbuffer: vec![0; (WIDTH*HEIGHT) as usize],
        }
    }
}

impl Console for NesNtsc {
    fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) {
        // only accepting ines for now
        let ines_file = File::open(rom_path).unwrap();
        let ines = ines::Ines::from_rom(ines_file).unwrap();
        self.mapper = mappers::create_mapper(&ines);

        self.power_on_console();
    }

    fn power_on_console(&mut self) {
        let (cpu, cpu_pinout) = Rp2a03::from_power_on();
        self.cpu = cpu;
        self.cpu_pinout = cpu_pinout;
        
        self.ppu = Rp2c02::from_power_on();
        self.dma = Dma::from_power_on();
        self.pbuffer = vec![0; (WIDTH*HEIGHT) as usize];
    }

    fn restart_console(&mut self) {
        // TODO implement restart
    }

    fn execute_frame(&mut self, frame_buffer: &mut [u32]) {
        loop {
            {
                let mut bus = CpuBus::new(&mut *self.mapper, &mut self.dma, &mut self.ppu, &mut self.controllers);
                self.cpu_pinout = self.cpu.tick(&mut bus, self.cpu_pinout);
            }
    
            {
                let mut bus = DmaBus::new(&mut *self.mapper, &mut self.ppu, &mut self.controllers);
                self.cpu_pinout = self.dma.tick(&mut bus, self.cpu_pinout);
            }
    
            {
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                if self.ppu.is_end_of_frame() { break; }
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                if self.ppu.is_end_of_frame() { break; }
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                if self.ppu.is_end_of_frame() { break; }
            }

            {
                // APU
            }

            {
                self.cpu_pinout = (*self.mapper).cpu_tick(self.cpu_pinout);
            }
        }

        for it in frame_buffer.iter_mut().zip(self.pbuffer.iter_mut()) {
            let (fi, pi) = it;
            *fi = PALETTE[(*pi) as usize];
        }
    }

    fn execute_scanline(&mut self, frame_buffer: &mut [u32]) {
        loop {
            {
                let mut bus = CpuBus::new(&mut *self.mapper, &mut self.dma, &mut self.ppu, &mut self.controllers);
                self.cpu_pinout = self.cpu.tick(&mut bus, self.cpu_pinout);
            }
    
            {
                let mut bus = DmaBus::new(&mut *self.mapper, &mut self.ppu, &mut self.controllers);
                self.cpu_pinout = self.dma.tick(&mut bus, self.cpu_pinout);
            }
    
            {
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                if self.ppu.is_end_of_scanline() { break; }
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                if self.ppu.is_end_of_scanline() { break; }
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                if self.ppu.is_end_of_scanline() { break; }
            }

            {
                self.cpu_pinout = (*self.mapper).cpu_tick(self.cpu_pinout);
            }
        }

        for it in frame_buffer.iter_mut().zip(self.pbuffer.iter_mut()) {
            let (fi, pi) = it;
            *fi = PALETTE[(*pi) as usize];
        }
    }

    fn execute_cycle(&mut self) {
        {
            let mut bus = CpuBus::new(&mut *self.mapper, &mut self.dma, &mut self.ppu, &mut self.controllers);
            self.cpu_pinout = self.cpu.tick(&mut bus, self.cpu_pinout);
        }

        {
            let mut bus = DmaBus::new(&mut *self.mapper, &mut self.ppu, &mut self.controllers);
            self.cpu_pinout = self.dma.tick(&mut bus, self.cpu_pinout);
        }

        {
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
        }

        {
            self.cpu_pinout = (*self.mapper).cpu_tick(self.cpu_pinout);
        }
    }

    fn set_joypad1_state(&mut self, controller: JoypadInput) {
        self.controllers.set_joypad1_state(controller);
    }

    fn set_joypad2_state(&mut self, controller: JoypadInput) {
        self.controllers.set_joypad2_state(controller);
    }

    fn get_frame_number(&mut self) -> u64 {
        self.ppu.frame_number()
    }
}
