use std::fmt;

bitflags! {
    pub struct Ctrl: u8 {
        const RD =   0b00000001;    // /RD specify PPU reading, wierd behaivor if RD and WR asserted at same time
        const WR =   0b00000010;    // /WR specify PPU writing, an exception, writing to the internal palette range (3F00-3FFF) will not assert /WR
        const ALE =  0b00000100;    // ALE address latch enable, used to latch lower 8 bits of PPU address bus
        
    }
}

impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl::RD | Ctrl::WR
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pinout {
    pub address: u16,
    pub data: u8,
    pub ext: u8,         // ext, allows combination of two PPUs (e.g. Playchoice)
    pub ctrl: Ctrl,
}

//external state of cpu
impl Pinout {
    pub fn new() -> Pinout {
        Pinout {
            address: 0,
            data: 0,
            ext: 0,
            ctrl: Default::default(),
        }
    }
}

impl fmt::Display for Pinout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.ctrl.contains(Ctrl::RD), self.ctrl.contains(Ctrl::WR)) {
            (true, true) => {
                return write!(f, " --- ");
            }
            (false, true) => {
                return write!(f, "{:#X} -R-> {:#X}", self.address, self.data);
            }
            (true, false) => {
                return write!(f, "{:#X} <-W- {:#X}", self.address, self.data);
            }
            (false, false) => {
                return write!(f, "{:#X} <-RW-> {:#X}", self.address, self.data);
            }
        }
    }
}