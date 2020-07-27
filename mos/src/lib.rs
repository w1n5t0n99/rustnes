#[macro_use]
extern crate bitflags;

mod core;
mod instructions;
mod operations;
pub mod bus;
pub mod rp2a03;

use std::fmt;

/*
Mos 6502

## Emulated Pins
************************************
*           +-----------+          *
*   IRQ --->|           |---> A0   *
*   NMI --->|           |...       *
*    RDY--->|           |---> A15  *
*    RES--->|           |          *
*    RW <---|           |<--- HALT *
*  SYNC <---|           |          *
*           |           |<--> D0   *
*   (P0)<-->|           |...       *
*        ...|           |<--> D7   *
*   (P5)<-->|           |          *
*           +-----------+          *
************************************

The input/output P0..P5 pins only exist on the m6510.

The HALT pin is only used by the 6502C (Atari 5200/ Sally), unlike the RDY pin HALT halts
the cpu during Rd or Wr cycles.

If the RDY pin is active (1) the CPU will loop on the next read
access until the pin goes inactive.

*/

bitflags! {
    pub struct Ctrl: u8 {
        const RW =   0b00000001;    // R read /W Write
        const SYNC = 0b00000010;    // SYNC first cycle of instruction
        const IRQ =  0b00000100;    // /IRQ maskable interrupt
        const NMI =  0b00001000;    // /NMI non maskable interrupt
        const RDY =  0b00010000;    // RDY cpu is ready /RDY cpu is not ready and is paused during next read cycle
        const HALT = 0b00100000;    // /HALT cpu is halted, only on Atari "SALLY" cpu
    }
}

impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl::RW | Ctrl::IRQ | Ctrl::NMI | Ctrl::RDY | Ctrl::HALT
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pinout {
    pub address: u16,
    pub opt0: u16,
    pub opt1: u8,
    pub data: u8,
    pub io: u8,         // (io pins 6510 / controller pins rp2a03) 
    pub ctrl: Ctrl,
}

//external state of cpu
impl Pinout {
    pub fn new() -> Pinout {
        Pinout {
            address: 0,
            data: 0,
            opt0: 0,
            opt1: 0,
            io: 0,
            ctrl: Default::default(),
        }
    }
}

impl fmt::Display for Pinout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ctrl.contains(Ctrl::RW) {
            return write!(f, "{:#X} -R-> {:#X}", self.address, self.data);
        }
        else {
            return write!(f, "{:#X} <-W- {:#X}", self.address, self.data);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{rp2a03, Ctrl};
    
    #[test]
    fn it_works() {
        let (_cpu, cpu_pinout) = rp2a03::Rp2a03::from_power_on();
        
        assert_eq!(cpu_pinout.ctrl.contains(Ctrl::RDY), true);
    }
}
