use std::unimplemented;

use super::*;

pub struct MapperNull;

impl Mapper for MapperNull {
      // CPU
      fn read_cpu_0000_1fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_4020_5fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); } 
      fn read_cpu_6000_7fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_8000_8fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_9000_9fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_a000_afff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_b000_bfff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_c000_cfff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_d000_dfff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_e000_efff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn read_cpu_f000_ffff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
  
      fn write_cpu_0000_1fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_4020_5fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }   
      fn write_cpu_6000_7fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }   
      fn write_cpu_8000_8fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }   
      fn write_cpu_9000_9fff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_a000_afff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_b000_bfff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_c000_cfff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_d000_dfff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_e000_efff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn write_cpu_f000_ffff(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      // PPU   
      fn read_ppu_0000_03ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_0400_07ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_0800_0bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_0c00_0fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_1000_13ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_1400_17ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_1800_1bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_1c00_1fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_2000_23ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_2400_27ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_2800_2bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn read_ppu_2c00_2fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
  
      fn write_ppu_0000_03ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_0400_07ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_0800_0bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_0c00_0fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_1000_13ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_1400_17ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_1800_1bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_1c00_1fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_2000_23ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_2400_27ff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_2800_2bff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn write_ppu_2c00_2fff(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
      fn cpu_tick(&mut self, pinout: mos::Pinout) -> mos::Pinout { unimplemented!(); }
      fn ppu_tick(&mut self, pinout: ppu::Pinout) -> ppu::Pinout { unimplemented!(); }
}