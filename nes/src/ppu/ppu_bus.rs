use super::super::mappers::Mapper;

#[inline]
const fn to_address(address: u16, latch: u8) -> u16 {
    (address & 0xFF00) | (latch as u16) 
}

#[inline] 
const fn to_latch(address: u16) -> u8 {
    address as u8
}

bitflags! {
    pub struct Ctrl: u8 {
        const RD =   0b00000001;    // Read
        const WR =   0b00000010;    // Write, Write occurs if RD is also asserted
        const ALE =  0b00000100;    // Latch low address byte
    }
}

impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl::empty()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum IoStatus {
    RdCycle0,
    RdCycle1,
    WrCycle0,
    WrCycle1,
    Idle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PpuBus {
    pub addr_bus: u16,
    pub ale_latch: u8,
    pub rd_buffer: u8,
    pub wr_buffer: u8,
    io_status: IoStatus,
    ctrl: Ctrl,
}

impl PpuBus {
    pub fn new() -> Self {
        PpuBus {
            addr_bus: 0,
            ale_latch: 0,
            rd_buffer: 0,
            wr_buffer: 0,
            io_status: IoStatus::Idle,
            ctrl: Default::default(),
        }
    }   

    
    pub fn io_read(&mut self) -> u8 {
        self.io_status = IoStatus::RdCycle0;
        self.rd_buffer
    }

    pub fn io_read_palette(&mut self) {
        self.io_status = IoStatus::RdCycle0;
    }

    pub fn io_write(&mut self, data: u8) {
        self.io_status = IoStatus::WrCycle0;
        self.wr_buffer = data;
    }

    pub fn ale_cycle(&mut self, address: u16, mapper: &mut dyn Mapper, pinout: mos::Pinout) -> (bool, mos::Pinout) {
        self.ctrl.set(Ctrl::ALE, true);
        self.ctrl.set(Ctrl::RD, false);
        self.ctrl.set(Ctrl::WR, false);

        match self.io_status {
            IoStatus::Idle => {
                self.ale_latch = to_latch(address);
                (false, pinout)
            }
            IoStatus::RdCycle0 => {
                self.io_status = IoStatus::RdCycle1;
                self.ale_latch = to_latch(address);
                (false, pinout)
            }
            IoStatus::WrCycle0 => {
                self.io_status = IoStatus::WrCycle1;
                self.ale_latch = to_latch(address);
                (false, pinout)
            }
            IoStatus::RdCycle1 => {
                self.io_status = IoStatus::Idle;
                self.ctrl.set(Ctrl::RD, true);

                let vaddr = to_address(address, self.ale_latch);
                let (data, pinout) = match vaddr {
                    0x0000..=0x1FFF => mapper.read_pattern_table(vaddr, pinout),
                    0x2000..=0x3FFF => mapper.read_nametable(vaddr, pinout),
                    _ => panic!("PPU address out of bounds: {}", vaddr),
                };

                self.addr_bus = to_address(vaddr, data);
                self.ale_latch = data;
                self.rd_buffer = data;

                (true, pinout)
            }
            IoStatus::WrCycle1 => {
                self.io_status = IoStatus::Idle;
                self.ctrl.set(Ctrl::WR, true);


                (true, pinout)
            }
        }

    }


}