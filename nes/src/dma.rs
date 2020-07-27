use mos::{Pinout, Ctrl};
use mos::bus::Bus;
use std::fmt;

macro_rules! cpu_read {
    ($bus:ident, $pinout:ident, $addr:expr) => {{
        $pinout.ctrl.set(Ctrl::RW, true);
        $pinout.address = $addr;
        // get results from bus
        $pinout = $bus.read($pinout);
        $pinout.data
    }};
}

macro_rules! cpu_write {
    ($bus:ident, $pinout:ident, $addr:expr, $data: expr) => {
        $pinout.ctrl.set(Ctrl::RW, false);
        $pinout.address = $addr;
        $pinout.data = $data;
        // get results from bus
        $pinout = $bus.write($pinout);
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
enum OamStatus {
    Idle,
    Halt,
    Align,
    Read,
    Write,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum DmcStatus {
    Idle,
    Halt,
    Dummy,
    Align,
    Dmc,
}

// The Apu and Dma are apart of the Cpu package and
// such the sample is sent directly to Apu and not over the Bus
pub trait ApuDmaInterconnect {
    fn update_dmc_sample(&mut self, sample: u8);
}

#[derive(Debug)]
pub struct Dma {
    oam_status: OamStatus,
    dmc_status: DmcStatus,
    cur_oam_status: OamStatus,
    cur_dmc_status: DmcStatus,
    oam_addr: u16,
    dmc_addr: u16,
    data: u8,
    oam_tm: u8,
    oam_triggered: bool,
    dmc_triggered: bool,
    get_cycle: bool,
    put_cycle: bool,
    oam_rdy: bool,
    dmc_rdy: bool,
}

impl Dma {
    pub fn from_power_on() -> Dma {
        Dma {
            oam_status: OamStatus::Idle,
            dmc_status: DmcStatus::Idle,
            cur_oam_status: OamStatus::Idle,
            cur_dmc_status: DmcStatus::Idle,
            oam_addr: 0,
            dmc_addr: 0,
            data: 0,
            oam_tm: 0,
            oam_triggered: false,
            dmc_triggered: false,
            get_cycle: true,
            put_cycle: false,
            oam_rdy: true,
            dmc_rdy: true,
        }
    }

    pub fn reset(&mut self) {
        self.oam_status = OamStatus::Idle;
        self.dmc_status = DmcStatus::Idle;
        self.cur_oam_status = OamStatus::Idle;
        self.cur_dmc_status = DmcStatus::Idle;
        self.oam_addr = 0;
        self.dmc_addr = 0;
        self.data = 0;
        self.oam_tm = 0;
        self.oam_triggered = false;
        self.dmc_triggered = false;
        self.get_cycle = true;
        self.put_cycle = false;
        self.oam_rdy = true;
        self.dmc_rdy = true;
    }

    pub fn oam_execute(&mut self, addr: u8) {
        // dma only watches address 0x4014
        self.oam_addr = (addr as u16) << 8;
        self.oam_triggered = true;
    }

    pub fn dmc_execute(&mut self, sample_addr: u8)  {
        // DPCM samples must begin in the memory range
        // $C000-FFFF at an address set by register $4012 (address = %11AAAAAA.AA000000)
        self.dmc_addr = 0xC000 | ((sample_addr as u16) << 6);
        self.dmc_triggered = true;
    }

    pub fn tick<B: Bus + ApuDmaInterconnect>(&mut self, bus: &mut B, mut pinout: Pinout) -> Pinout {
        // set cur om and dmc state used for logging
        self.cur_dmc_status = self.dmc_status;
        self.cur_oam_status = self.oam_status;

        let mut oam_pause = false;
        match self.dmc_status {
            DmcStatus::Halt => {
                // if read cycle, the cpu has been halted
                if pinout.ctrl.contains(Ctrl::RW) {
                    self.dmc_status = DmcStatus::Dummy;
                }
            },
            DmcStatus::Dummy => {
                self.dmc_status = if self.get_cycle == true { DmcStatus::Align } else { DmcStatus::Dmc };
            },
            DmcStatus::Align => {
                self.dmc_status = DmcStatus::Dmc;
            },
            DmcStatus::Dmc => {
                let d = cpu_read!(bus, pinout, self.dmc_addr);
                bus.update_dmc_sample(d);

                oam_pause = true;
                self.dmc_rdy = true;
                self.dmc_status = DmcStatus::Idle;
            },
            DmcStatus::Idle => {

            },
        }

        match (self.oam_status, oam_pause) {
            (OamStatus::Halt, _) => {
                if pinout.ctrl.contains(Ctrl::RW) {
                    self.oam_status =  if self.get_cycle { OamStatus::Align } else { OamStatus::Read };
                }
            },
            (OamStatus::Align, false) => {
                self.oam_status = OamStatus::Read;
            },
            (OamStatus::Read, false) => {
                self.data = cpu_read!(bus, pinout, self.oam_addr);
                self.oam_status = OamStatus::Write;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            },
            (OamStatus::Read, true) => {
                // paused for dmc need to re-align
                self.oam_status = OamStatus::Align;
            },
            (OamStatus::Write, false) => {
                // write oam data to OAMDATA port
                cpu_write!(bus, pinout, 0x2004, self.data);
                self.oam_status = if self.oam_tm == 255 {
                    self.oam_rdy = true;
                    OamStatus::Idle
                }
                else {
                    OamStatus::Read
                };

                self.oam_tm = self.oam_tm.wrapping_add(1);

            },
            (_, true) => {
                // pause for dmc
            },
            _ => {},
        }

        if self.dmc_triggered {
            // if oam dma is already occuring we skip dmc halt cycle
            self.dmc_status = if self.oam_status == OamStatus::Idle { DmcStatus::Halt } else { DmcStatus::Dummy };
            self.dmc_rdy = false;
            self.dmc_triggered = false;
        }

        if self.oam_triggered {
            self.oam_status = OamStatus::Halt;
            self.oam_tm = 0;
            self.oam_rdy = false;
            self.oam_triggered = false;
        }

        // only un-halt cpu if dmc and oam are both idle
        pinout.ctrl.set(Ctrl::RDY, self.oam_rdy & self.dmc_rdy);
        // alternate "put"/"get" cycles
        self.put_cycle = !self.put_cycle;
        self.get_cycle = !self.get_cycle;
        pinout
    }
}

impl fmt::Display for Dma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let oam_str = match self.cur_oam_status {
            OamStatus::Idle => "Idle",
            OamStatus::Align => "Align",
            OamStatus::Halt => "Halt", 
            OamStatus::Read => "Read", 
            OamStatus::Write => "Write",
        };

        let dmc_str = match self.cur_dmc_status {
            DmcStatus::Idle => "Idle",
            DmcStatus::Align => "Align",
            DmcStatus::Halt => "Halt",
            DmcStatus::Dummy => "Dummy",
            DmcStatus::Dmc => "Dmc",
        };
        
        // if put then the current cycle must have been get
        if self.put_cycle == true {
            return write!(f, "[G] OAM: {} - DMC: {}", oam_str, dmc_str);
        }
        else {
            return write!(f, "[P] OAM: {} - DMC: {}", oam_str, dmc_str);
        } 
    }
}