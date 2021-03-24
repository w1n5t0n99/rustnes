use std::thread::JoinHandle;


const NESS001_MASK: u8 = 0b11100000;
const NESS101_MASK: u8 = 0b11100100;

bitflags! {
    pub struct JoypadInput: u8 {
        const A                  = 0b00000001;
        const B                  = 0b00000010;
        const SELECT             = 0b00000100;
        const START              = 0b00001000;
        const UP                 = 0b00010000;
        const DOWN               = 0b00100000; 
        const LEFT               = 0b01000000;
        const RIGHT              = 0b10000000;
    }
}

impl JoypadInput {
    pub fn new() -> JoypadInput {
        JoypadInput::from_bits_truncate(0)
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NesControllers {
    joypad1: JoypadInput,
    joypad2: JoypadInput,
    joypad1_latch: u8,
    joypad2_latch: u8,
    shift1_count: u8,
    shift2_count: u8,
    polling: bool,
}

impl NesControllers {
    pub fn from_power_on() -> Self {
        NesControllers {
            joypad1: JoypadInput::new(),
            joypad2: JoypadInput::new(),
            shift1_count: 0,
            shift2_count: 0,
            joypad1_latch: 0,
            joypad2_latch: 0,
            polling: false,
        }
    }

    pub fn set_joypad1_state(&mut self, controller: JoypadInput) {
        self.joypad1 = controller;
    }

    pub fn set_joypad2_state(&mut self, controller: JoypadInput) {
        self.joypad2 = controller;
    }

    pub fn clear_joypads_state(&mut self) {
        self.joypad1.clear();
        self.joypad2.clear();
    }

    pub fn write_4016(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        if (pinout.data & 0x01) == 1 && self.polling == false {
            self.polling = true;
            self.shift1_count = 8;
            self.shift2_count = 8;
            self.joypad1_latch = self.joypad1.bits();
            self.joypad2_latch = self.joypad2.bits();
        }
        else {
            self.polling = false;
        }

        pinout
    }

    pub fn read_4016(&mut self, mut pinout: mos::Pinout) ->  mos::Pinout {
        if self.polling == true {
            pinout.data = (pinout.data & NESS001_MASK) | (self.joypad1_latch & 0x1);
        }
        else {
            if self.shift1_count > 0 {
                pinout.data = (pinout.data & NESS001_MASK) | (self.joypad1_latch & 0x1);
                self.joypad1_latch >>= 1;
                self.shift1_count = self.shift1_count.saturating_sub(1);
            }
            else {
                pinout.data = (pinout.data & NESS001_MASK) | 0x1;
            }
        }

        pinout
    }

    pub fn read_4017(&mut self, mut pinout: mos::Pinout) ->  mos::Pinout {
        if self.polling == true {
            pinout.data = (pinout.data & NESS001_MASK) | (self.joypad2_latch & 0x1);
        }
        else {
            if self.shift2_count > 0 {
                pinout.data = (pinout.data & NESS001_MASK) | (self.joypad2_latch & 0x1);
                self.joypad2_latch >>= 1;
                self.shift2_count = self.shift2_count.saturating_sub(1);
            }
            else {
                pinout.data = (pinout.data & NESS001_MASK) | 0x1;
            }
        }

        pinout
    }
}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_standard_controller_polling() {
        let mut ct = NesControllers::from_power_on();
        let mut pinout = mos::Pinout::new();
        pinout = ct.read_4016(pinout);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0x1);

        let mut p1 = JoypadInput::from_bits_truncate(0x0);
        p1.set(JoypadInput::A, true);
        p1.set(JoypadInput::UP, true);
        ct.set_joypad1_state(p1);

        // load shifts
        pinout.data = 1;
        pinout = ct.write_4016(pinout);
        // while polling controller should return A 
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0x1);
        // lock shifts end polling
        pinout.data = 0;
        pinout = ct.write_4016(pinout);
        
        pinout.data = 0b10100000;
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 1);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 0);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 0);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 0);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 1);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 0);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 0);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 0);
        // remaining should all be 1
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 1);
        pinout = ct.read_4016(pinout);
        assert_eq!(pinout.data, 0b10100000 | 1);
       
    }
}

