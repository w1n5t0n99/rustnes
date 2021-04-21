use std::unimplemented;

use super::*;

pub struct MapperNull;

impl Mapper for MapperNull {
      fn read_cpu_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); } 
      fn read_cpu_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }

      fn write_cpu_internal_ram(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_exp(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); } 
      fn write_cpu_wram(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_prg(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }

      fn read_ppu_chr(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_nt(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
  
      fn write_ppu_chr(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_nt(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }

      fn cpu_tick(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn ppu_tick(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
}