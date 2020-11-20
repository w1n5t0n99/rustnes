use crate::{StandardInput, ZapperInput};

const NESS001_MASK: u8 = 0b11100000;
const NESS101_MASK: u8 = 0b11100100;

#[derive(Debug, Clone, Copy)]
pub struct Controllers {
    controller1: StandardInput,
    zapper1: ZapperInput,
    controller2: StandardInput,
    zapper2: ZapperInput,
    shift1_count: u8,
    shift2_count: u8,
    polling: bool,
}

impl Controllers {
    pub fn from_power_on() -> Self {
        Controllers {
            controller1: StandardInput::from_bits_truncate(0),
            zapper1: ZapperInput::from_bits_truncate(0),
            controller2: StandardInput::from_bits_truncate(0),
            zapper2: ZapperInput::from_bits_truncate(0),
            shift1_count: 8,
            shift2_count: 8,
            polling: false,
        }
    }

    pub fn update_controller1(&mut self, controller: StandardInput) {
        if self.polling == false {
            return;
        }

        self.controller1 = controller;
    }

    pub fn update_zapper1(&mut self, zapper: ZapperInput) {
        if self.polling == false {
            return;
        }

        self.zapper1 = zapper;
    }

    pub fn update_controller2(&mut self, controller: StandardInput) {
        if self.polling == false {
            return;
        }

        self.controller2 = controller;
    }

    pub fn update_zapper2(&mut self, zapper: ZapperInput) {
        if self.polling == false {
            return;
        }

        self.zapper2 = zapper;
    }

    pub fn write_controller1(&mut self, pinout: mos::Pinout) -> mos::Pinout {
        if (pinout.data & 0x01) == 1 {
            self.polling = true;
            self.shift1_count = 8;
            self.shift2_count = 8;
        }
        else {
            self.polling = false;
        }

        pinout
    }

    pub fn read_controller1(&mut self, mut pinout: mos::Pinout) ->  mos::Pinout {
        if self.polling == true {
            pinout.data = self.controller1.bits() & 0x1;
        }
        else {
            let bits = self.controller1.bits();
            if self.shift1_count > 0 {
                pinout.data = (pinout.data & NESS001_MASK) | (bits & 0x1);
                self.controller1 = StandardInput::from_bits_truncate(bits >> 1);
                self.shift1_count -= 1;
            }
            else {
                pinout.data = (pinout.data & NESS001_MASK) | 0x1;
            }
        }

        pinout
    }

    pub fn read_controller2(&mut self, mut pinout: mos::Pinout) ->  mos::Pinout {
        if self.polling == true {
            pinout.data = self.controller2.bits() & 0x1;
        }
        else {
            let bits = self.controller2.bits();
            if self.shift1_count > 0 {
                pinout.data = (pinout.data & NESS001_MASK) | (bits & 0x1);
                self.controller2 = StandardInput::from_bits_truncate(bits >> 1);
                self.shift1_count -= 1;
            }
            else {
                pinout.data = (pinout.data & NESS001_MASK) | 0x1;
            }
        }

        pinout
    }
}

