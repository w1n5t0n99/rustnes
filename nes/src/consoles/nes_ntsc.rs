use super::*;
use crate::dma::Dma;
use crate::ppu::rp2c02::Rp2c02;
use crate::mappers;
use crate::mappers::Mapper;
use crate::palette::*;
use crate::bus::*;
use mos::{Pinout, rp2a03::Rp2a03};

use std::fs::File;
use std::path::Path;
use ::nes_rom::ines;
use std::fmt;

pub struct NesNtsc {
    cpu: Rp2a03,
    cpu_pinout: Pinout,
    dma: Dma,
    ppu: Rp2c02,
    mapper: Box<dyn Mapper>,
    pbuffer: Vec<u16>,
    palette: Vec<u32>,
    nt_index: u8,
}

impl NesNtsc {
    pub fn new() -> Self {
        let (cpu, cpu_pinout) = Rp2a03::from_power_on();
        NesNtsc {
            cpu: cpu,
            cpu_pinout: cpu_pinout,
            dma: Dma::from_power_on(),
            ppu: Rp2c02::from_power_on(),
            mapper: mappers::create_mapper_null(),
            pbuffer: vec![0; (WIDTH*HEIGHT) as usize],
            palette: generate_palette(DEFAULT_SATURATION, DEFAULT_HUE, DEFAULT_CONTRAST, DEFAULT_BRIGHTNESS, DEFAULT_GAMMA),
            nt_index: 0,
        }
    }

    pub fn set_entry(&mut self, addr: u16) {
        self.mapper.rst_vector(addr);
    }

    pub fn load_debug_rom(&mut self) {
        self.mapper = mappers::create_mapper_debug();

        self.power_on();
    }

    pub fn nametable_framebuffer(&mut self, frame_buffer: &mut [u32]) {
        let cpu_cycles = if self.ppu.is_odd_frame() { 29780 } else { 29781 };
        let old_ppu = self.ppu;

        self.ppu.enable_rendering(false);
        self.ppu.reset_renderer();
        let mut cpu_pinout = Pinout::new();
        cpu_pinout.address = 0x23C0;
        
        cpu_pinout = self.ppu.write_ppuaddr(cpu_pinout);
        cpu_pinout = self.ppu.write_ppuaddr(cpu_pinout);

        for n in 0x23C0..0x23FF {
            cpu_pinout.data = self.nt_index;
            cpu_pinout = self.ppu.write_ppudata(cpu_pinout);
            //self.nt_index ^= 0xFF;

            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
        }

        cpu_pinout.address = 0x2000;
        
        cpu_pinout = self.ppu.write_ppuaddr(cpu_pinout);
        cpu_pinout = self.ppu.write_ppuaddr(cpu_pinout);


        self.ppu.reset_renderer();
        self.ppu.enable_rendering(true);

        for _cycle in 0..=cpu_cycles {
            {
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            }
        }

        for it in frame_buffer.iter_mut().zip(self.pbuffer.iter_mut()) {
            let (fi, pi) = it;
            *fi = self.palette[(*pi) as usize];
        }
        
        self.ppu = old_ppu;
    }
}

impl Console for NesNtsc {
    fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) {
        // only accepting ines for now
        let ines_file = File::open(rom_path).unwrap();
        let ines = ines::Ines::from_rom(ines_file).unwrap();
        self.mapper = mappers::create_mapper(&ines);

        self.power_on();
    }

    fn power_on(&mut self) {
        let (cpu, cpu_pinout) = Rp2a03::from_power_on();
        self.cpu = cpu;
        self.cpu_pinout = cpu_pinout;
        
        self.ppu = Rp2c02::from_power_on();
        self.dma = Dma::from_power_on();
        self.pbuffer = vec![0; (WIDTH*HEIGHT) as usize];
        // RAM values are undefined on power-on so prob don't need to zero out
    }

    fn restart(&mut self) {
        // TODO implement restart
    }

    fn execute_frame(&mut self, frame_buffer: &mut [u32]) {
        let cpu_cycles = if self.ppu.is_odd_frame() { 29780 } else { 29781 };

        for _cycle in 0..=cpu_cycles {
            {
                let mut bus = CpuBus::new(&mut *self.mapper, &mut self.dma, &mut self.ppu);
                self.cpu_pinout = self.cpu.tick(&mut bus, self.cpu_pinout);
            }
    
            {
                let mut bus = DmaBus::new(&mut *self.mapper, &mut self.ppu);
                self.cpu_pinout = self.dma.tick(&mut bus, self.cpu_pinout);
            }
    
            {
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
                self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            }
        }

        for it in frame_buffer.iter_mut().zip(self.pbuffer.iter_mut()) {
            let (fi, pi) = it;
            *fi = self.palette[(*pi) as usize];
        }
    }

    fn execute_cycle(&mut self) {
        {
            let mut bus = CpuBus::new(&mut *self.mapper, &mut self.dma, &mut self.ppu);
            self.cpu_pinout = self.cpu.tick(&mut bus, self.cpu_pinout);
        }

        {
            let mut bus = DmaBus::new(&mut *self.mapper, &mut self.ppu);
            self.cpu_pinout = self.dma.tick(&mut bus, self.cpu_pinout);
        }

        {
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
            self.cpu_pinout = self.ppu.tick(&mut self.pbuffer, &mut *self.mapper, self.cpu_pinout);
        }
    }
}

impl fmt::Display for NesNtsc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {  
        write!(f, "[ {} ] [ {} ] [ {} ] [ {} ]\n",self.cpu, self.dma, self.cpu_pinout, self.ppu)
    }
}